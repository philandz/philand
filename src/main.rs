use std::net::SocketAddr;
use std::sync::Arc;
use axum::{
    routing::{get, post, patch, delete}, 
    Router, 
    http::Method,
    body::Body,
    http::Request,
    middleware::Next,
    response::IntoResponse,
    extract::{State, ConnectInfo},
};
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders};
use dotenvy::dotenv;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use crate::config::config::get_config;
use crate::utils::database;
use crate::utils::rate_limiter::RateLimiter;
use crate::utils::error::error::AppError;

mod config;
mod handler;
mod manager;
mod utils;

// Rate limiting middleware for auth routes
async fn rate_limit_middleware(
    State(state): State<Arc<handler::AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    state.rate_limiter.check_rate_limit(addr.ip())
        .map_err(|msg| AppError::TooManyRequests(msg))?;
    Ok(next.run(req).await)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    dotenv().ok();
    config::config::init();
    let db_url = get_config().get_database_config().get_url();
    let pool = database::database::create_pool(&db_url).await?;
    //sqlx::migrate!("./migrations").run(&pool).await?;
    
    // Initialize S3 client for avatar storage
    utils::s3_storage::init_s3_client().await?;
    info!("S3 storage initialized");
    
    // Initialize rate limiter
    let rate_limit_cfg = get_config().get_rate_limit_config();
    let rate_limiter = Arc::new(RateLimiter::new(
        rate_limit_cfg.short_window_sec,
        rate_limit_cfg.short_max,
        rate_limit_cfg.long_window_sec,
        rate_limit_cfg.long_max,
        rate_limit_cfg.fail_threshold,
        rate_limit_cfg.fail_lock_min,
    ));
    
    let state = Arc::new(handler::AppState { 
        pool: pool.clone(),
        rate_limiter: rate_limiter.clone(),
    });
    
    // Start cleanup cron job (runs every 24 hours)
    let cleanup_pool = pool.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(24 * 60 * 60)); // 24 hours
        loop {
            interval.tick().await;
            info!("Running cleanup job for soft-deleted records...");
            match utils::cleanup::CleanupService::cleanup_daily(&cleanup_pool).await {
                Ok(stats) => {
                    info!("{}", stats);
                }
                Err(e) => {
                    tracing::error!("Cleanup job failed: {}", e);
                }
            }
        }
    });

    let cors_layer = {
        let origins_vec = get_config().get_cors_origins();
        if !origins_vec.is_empty() {
            let origins: Vec<axum::http::HeaderValue> = origins_vec
                .into_iter()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .map(|s| axum::http::HeaderValue::from_str(&s).expect("invalid CORS origin"))
                .collect();
            CorsLayer::new().allow_origin(AllowOrigin::list(origins))
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS, Method::PATCH])
                .allow_headers(AllowHeaders::list(vec![
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::ACCEPT,
                    axum::http::header::ORIGIN,
                ]))
                .allow_credentials(true)
        } else {
            CorsLayer::permissive()
        }
    };

    let protected = Router::new()
        .route("/api/users", get(handler::users::list))
        .route("/api/profile", get(handler::profile::get_profile).patch(handler::profile::update_profile))
        .route("/api/profile/avatar", post(handler::profile::upload_avatar).delete(handler::profile::delete_avatar))
        .route("/api/budgets", get(handler::budgets::list).post(handler::budgets::create))
        .route("/api/budgets/{id}", get(handler::budgets::get).patch(handler::budgets::update).delete(handler::budgets::delete))
        .route("/api/budgets/{id}/balance", get(handler::budgets::get_balance))
        .route("/api/budgets/{id}/categories", get(handler::categories::list).post(handler::categories::create))
        .route("/api/budgets/{id}/categories/{category_id}", get(handler::categories::get_by_id).patch(handler::categories::update).delete(handler::categories::delete))
        .route("/api/budgets/{id}/entries", get(handler::entries::list).post(handler::entries::create))
        .route("/api/budgets/{id}/entries/{entry_id}", patch(handler::entries::update).delete(handler::entries::delete))
        .route("/api/budgets/{id}/entries/{entry_id}/comments", get(handler::comments::list_comments).post(handler::comments::create_comment))
        .route("/api/budgets/{id}/entries/{entry_id}/comments/{comment_id}", patch(handler::comments::update_comment).delete(handler::comments::delete_comment))
        .route("/api/budgets/{id}/entries/{entry_id}/attachments", post(handler::comments::upload_attachment))
        .route("/api/budgets/{id}/entries/{entry_id}/attachments/{attachment_id}", delete(handler::comments::delete_attachment))
        .route("/api/budgets/{id}/summary/monthly", get(handler::summaries::monthly))
        .route("/api/budgets/{id}/members", get(handler::members::list).post(handler::members::upsert))
        .route("/api/budgets/{id}/members/{user_id}", patch(handler::members::update).delete(handler::members::delete))
        .route("/api/notifications", get(handler::notifications::list_notifications))
        .route("/api/notifications/unread-count", get(handler::notifications::get_unread_count))
        .route("/api/notifications/mark-read", post(handler::notifications::mark_as_read))
        .route("/api/notifications/mark-all-read", post(handler::notifications::mark_all_as_read))
        .route("/api/transfers", post(handler::transfers::create_transfer))
        .route("/api/admin/cleanup", post(handler::cleanup::manual_cleanup))
        .route_layer(axum::middleware::from_fn(handler::auth::auth_middleware));

    // Auth routes with rate limiting
    let auth_routes = Router::new()
        .route("/auth/signup", post(handler::auth::signup))
        .route("/auth/login", post(handler::auth::login))
        .route("/auth/google", post(handler::google_auth::google_auth))
        .route("/auth/forgot/email", post(handler::auth::forgot_email))
        .route("/auth/forgot/otp", post(handler::auth::forgot_otp))
        .route("/auth/reset", post(handler::auth::reset_password))
        .route("/auth/logout", post(handler::auth::logout))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware
        ));

    let app = Router::new()
        .route("/healthz", get(handler::health::health))
        .route("/api", get(philand::docs::swagger_ui))
        .route("/api/openapi.json", get(philand::docs::openapi_json))
        .merge(auth_routes)
        .merge(protected)
        .with_state(state)
        .layer(cors_layer);

    let port: u16 = get_config().get_network_config().get_port().parse().unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
    Ok(())
}
