use axum::{body::Body, Json, extract::State, http::Request, middleware::Next, response::IntoResponse};
use headers::{authorization::Bearer, Authorization, HeaderMapExt};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};
use bcrypt::verify;

use crate::manager::models::{user::{User, CreateUserReq}};
use crate::manager::biz::users::UserService;
use crate::manager::biz::password_reset::PasswordResetService;
use crate::utils::error::error::AppError;
use crate::config::config::get_config;
use super::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims { pub sub: String, pub email: String, pub exp: usize }

fn encoding_key() -> EncodingKey {
    let secret = get_config().get_jwt_config().get_secret();
    EncodingKey::from_secret(secret.as_bytes())
}
fn decoding_key() -> DecodingKey {
    let secret = get_config().get_jwt_config().get_secret();
    DecodingKey::from_secret(secret.as_bytes())
}

#[derive(Deserialize)] 
pub struct LoginReq { 
    pub email: String, 
    pub password: String 
}

#[derive(Serialize)] 
pub struct LoginResp { 
    pub token: String, 
    pub user: User 
}

pub async fn signup(State(state): State<Arc<AppState>>, Json(req): Json<CreateUserReq>) -> Result<Json<User>, AppError> {
    let cost = get_config().get_jwt_config().get_bcrypt_cost();
    Ok(Json(UserService::create(&state.pool, req, cost).await?))
}

pub async fn login(State(state): State<Arc<AppState>>, Json(req): Json<LoginReq>) -> Result<Json<LoginResp>, AppError> {
    let auth = UserService::find_auth(&state.pool, &req.email).await?;
    let (user_id, password_hash) = auth.ok_or(AppError::Unauthorized)?;
    let valid = verify(&req.password, &password_hash).map_err(|_| AppError::Internal)?;
    if !valid { return Err(AppError::Unauthorized); }
    let user = sqlx::query_as::<_, User>("SELECT id, email, name, avatar, bio, timezone, locale, created_at, updated_at FROM users WHERE id = ?").bind(&user_id).fetch_one(&state.pool).await?;
    let ttl_min: i64 = get_config().get_jwt_config().get_ttl_min();
    let exp = (OffsetDateTime::now_utc() + Duration::minutes(ttl_min)).unix_timestamp() as usize;
    let claims = Claims { sub: user.id.clone(), email: user.email.clone(), exp };
    let token = encode(&Header::new(Algorithm::HS256), &claims, &encoding_key()).map_err(|_| AppError::Internal)?;
    Ok(Json(LoginResp { token, user }))
}

pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Result<impl IntoResponse, AppError> {
    let headers = req.headers();
    let auth = headers.typed_get::<Authorization<Bearer>>().ok_or(AppError::Unauthorized)?;
    let token_data = decode::<Claims>(auth.token(), &decoding_key(), &Validation::new(Algorithm::HS256))
        .map_err(|_| AppError::Unauthorized)?;
    req.extensions_mut().insert(token_data.claims);
    Ok(next.run(req).await)
}

// Password Reset Endpoints

#[derive(Deserialize)]
pub struct ForgotEmailReq {
    pub email: String,
}

#[derive(Serialize)]
pub struct ForgotEmailResp {
    pub token: String,
    pub message: String,
}

pub async fn forgot_email(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ForgotEmailReq>,
) -> Result<Json<ForgotEmailResp>, AppError> {
    let token_ttl = get_config().get_reset_config().token_ttl_min;
    let token = PasswordResetService::create_email_reset(&state.pool, &req.email, token_ttl).await?;
    
    // In production, send email here
    tracing::info!("Password reset token for {}: {}", req.email, token);
    
    Ok(Json(ForgotEmailResp {
        token,
        message: "Password reset token generated. Check your email.".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct ForgotOtpReq {
    pub email: String,
}

#[derive(Serialize)]
pub struct ForgotOtpResp {
    pub otp: String,
    pub message: String,
}

pub async fn forgot_otp(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ForgotOtpReq>,
) -> Result<Json<ForgotOtpResp>, AppError> {
    let otp_ttl = get_config().get_reset_config().otp_ttl_min;
    let otp = PasswordResetService::create_otp_reset(&state.pool, &req.email, otp_ttl).await?;
    
    // In production, send SMS/email here
    tracing::info!("Password reset OTP for {}: {}", req.email, otp);
    
    Ok(Json(ForgotOtpResp {
        otp,
        message: "OTP code generated. Check your email/SMS.".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct ResetPasswordReq {
    pub token: Option<String>,
    pub email: Option<String>,
    pub otp: Option<String>,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct ResetPasswordResp {
    pub message: String,
}

pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ResetPasswordReq>,
) -> Result<Json<ResetPasswordResp>, AppError> {
    let bcrypt_cost = get_config().get_jwt_config().get_bcrypt_cost();
    
    PasswordResetService::reset_password(
        &state.pool,
        req.token,
        req.email,
        req.otp,
        req.new_password,
        bcrypt_cost,
    ).await?;
    
    Ok(Json(ResetPasswordResp {
        message: "Password reset successful. You can now login with your new password.".to_string(),
    }))
}

#[derive(Serialize)]
pub struct LogoutResp {
    pub message: String,
}

pub async fn logout() -> Result<Json<LogoutResp>, AppError> {
    // JWT is stateless, so logout is handled client-side by removing the token
    // This endpoint exists for API consistency
    Ok(Json(LogoutResp {
        message: "Logged out successfully. Please remove your token.".to_string(),
    }))
}
