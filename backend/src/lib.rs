//! Ferris Academy backend library — shared modules + router builder.
//!
//! The binary (`src/main.rs`) parses env config and then calls
//! [`build_router`]. Integration tests (`tests/api_tests.rs`) build an
//! isolated [`AppState`] (temp SQLite DB + tiny fixture content dir) and call
//! [`build_router`] in-process via `axum-test`.

pub mod api;
pub mod auth;
pub mod content_loader;
pub mod db;
pub mod error;
pub mod models;
pub mod state;

use axum::{
    http::{header, Method},
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer};

use crate::state::AppState;

/// Build the full API `Router` (same routes as production `main.rs`).
pub fn build_router(state: AppState) -> Router {
    // Explicit CORS origins (no permissive `*`): local backend + Vite dev
    // server, plus any extra origins from FRONTEND_ORIGIN (comma-separated).
    let mut allowed_origins: Vec<axum::http::HeaderValue> = vec![
        "http://127.0.0.1:8001"
            .parse()
            .expect("static origin parses"),
        "http://localhost:5173"
            .parse()
            .expect("static origin parses"),
    ];
    if let Ok(extra) = std::env::var("FRONTEND_ORIGIN") {
        for origin in extra.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            match origin.parse() {
                Ok(v) => allowed_origins.push(v),
                Err(_) => tracing::warn!(origin, "ignoring invalid FRONTEND_ORIGIN entry"),
            }
        }
    }
    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    Router::new()
        .route("/api/health", get(api::courses::health))
        .route("/api/courses", get(api::courses::courses))
        .route("/api/lessons/:slug", get(api::courses::lesson))
        .route("/api/search", get(api::courses::search))
        .route("/api/auth/register", post(api::auth::register))
        .route("/api/auth/login", post(api::auth::login))
        .route("/api/me", get(api::auth::me))
        .route("/api/progress", get(api::progress::progress))
        .route(
            "/api/lessons/:slug/complete",
            post(api::progress::complete).delete(api::progress::uncomplete),
        )
        .route(
            "/api/quizzes/:slug/attempt",
            post(api::progress::quiz_attempt),
        )
        // 2 MB body cap to bound memory on JSON endpoints. Full
        // rate-limiting (e.g. tower_governor) is out of scope for now.
        .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
        .layer(cors)
        .with_state(state)
}
