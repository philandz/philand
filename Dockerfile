FROM rust:1.92.0-slim-trixie AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config \
  libssl-dev \
  libmariadb-dev \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src && printf "fn main() {}\n" > src/main.rs
RUN cargo build --release && rm -rf src

COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  libssl3 \
  libmariadb3 \
  curl \
  && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /usr/sbin/nologin -U philand

WORKDIR /app

COPY --from=builder /app/target/release/philand /app/philand
COPY --from=builder /app/migrations /app/migrations

RUN chown -R philand:philand /app

USER philand

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
  CMD curl -fsS http://127.0.0.1:8080/healthz || exit 1

CMD ["/app/philand"]