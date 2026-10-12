//! Ferris Academy backend — Axum JSON API for the Rust learning platform.
//!
//! Run with:
//! ```sh
//! DATABASE_URL=sqlite://data/dev.db JWT_SECRET=<at-least-16-chars> cargo run
//! ```
//! `JWT_SECRET` is required (minimum 16 characters); the server exits with a
//! clear error at startup when it is missing or too short. There is no
//! insecure default.

use std::{path::PathBuf, sync::Arc};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use ferris_academy_backend::{build_router, content_loader::ContentStore, db, state::AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/dev.db".to_string());
    // SQLite creates the file but not parent directories — ensure them.
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        if let Some(parent) = std::path::Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).expect("failed to create database directory");
            }
        }
    }
    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(s) if s.len() >= 16 => s,
        Ok(_) => {
            eprintln!(
                "FATAL: JWT_SECRET must be at least 16 characters; set a strong random value"
            );
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!(
                "FATAL: JWT_SECRET env var is required (minimum 16 characters); refusing to start with an insecure default"
            );
            std::process::exit(1);
        }
    };
    let content_dir = std::env::var("CONTENT_DIR").unwrap_or_else(|_| "content".to_string());
    let port: u16 = match std::env::var("PORT") {
        Ok(raw) => match raw.parse() {
            Ok(p) => p,
            Err(_) => {
                tracing::warn!(value = %raw, "invalid PORT, falling back to 8001");
                // 8001: the Symfony platform uses 8000, so both can run side by side.
                8001
            }
        },
        Err(_) => {
            // 8001: the Symfony platform uses 8000, so both can run side by side.
            8001
        }
    };

    let db = db::connect(&database_url)
        .await
        .expect("database connection + migrations failed");

    let store = ContentStore::load(&PathBuf::from(&content_dir));
    tracing::info!(
        lessons = store.order.len(),
        malformed = store.malformed.len(),
        "course content loaded"
    );
    for bad in &store.malformed {
        tracing::warn!("malformed lesson skipped: {bad}");
    }

    let state = AppState {
        db,
        content: Arc::new(store),
        jwt_secret,
    };

    let app = build_router(state);

    let host_str = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let host: std::net::IpAddr = host_str.parse().unwrap_or_else(|_| {
        tracing::warn!(value = %host_str, "invalid HOST, falling back to 127.0.0.1");
        "127.0.0.1".parse().expect("loopback parses")
    });
    let addr = std::net::SocketAddr::from((host, port));
    tracing::info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
