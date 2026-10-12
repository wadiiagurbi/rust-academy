//! Auth handlers: registration + login + current user.

use axum::{extract::State, Json};
use chrono::Utc;
use serde_json::{json, Value};

use crate::{
    auth::{hash_password_async, mint_token, verify_password_async, AuthUser, DUMMY_PASSWORD_HASH},
    error::ApiError,
    models::{LoginRequest, RegisterRequest},
    state::AppState,
};

fn valid_email(email: &str) -> bool {
    // Pragmatic check (full RFC validation is overkill here).
    let parts: Vec<&str> = email.split('@').collect();
    parts.len() == 2 && !parts[0].is_empty() && parts[1].contains('.')
}

fn is_unique_violation(e: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = e {
        if db_err.code().as_deref() == Some("2067") {
            return true;
        }
        let msg = db_err.message();
        if msg.contains("UNIQUE constraint failed") {
            return true;
        }
    }
    false
}

/// POST /api/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<(axum::http::StatusCode, Json<Value>), ApiError> {
    let email = body.email.trim().to_lowercase();
    if email.len() > 254 {
        return Err(ApiError::bad_request(
            "email must be at most 254 characters",
        ));
    }
    if !valid_email(&email) {
        return Err(ApiError::bad_request(
            "please provide a valid email address",
        ));
    }
    if body.password.len() < 8 {
        return Err(ApiError::bad_request(
            "password must be at least 8 characters",
        ));
    }
    if body.password.len() > 128 {
        return Err(ApiError::bad_request(
            "password must be at most 128 characters",
        ));
    }
    let display_name = body.display_name.trim().to_string();
    if display_name.is_empty() {
        return Err(ApiError::bad_request("display name is required"));
    }
    if display_name.chars().count() > 64 {
        return Err(ApiError::bad_request(
            "display name must be at most 64 characters",
        ));
    }

    let existing: Option<i64> = sqlx::query_scalar("SELECT id FROM users WHERE email = ?")
        .bind(&email)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(?e, "database error checking existing user");
            ApiError::internal("database error")
        })?;
    if existing.is_some() {
        return Err(ApiError::conflict(
            "an account with this email already exists",
        ));
    }

    // CPU-bound argon2 hashing runs in spawn_blocking (see auth::hash_password_async).
    let hash = hash_password_async(body.password.clone()).await?;
    let now = Utc::now().to_rfc3339();
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO users (email, display_name, password_hash, created_at) VALUES (?, ?, ?, ?) RETURNING id",
    )
    .bind(&email)
    .bind(&display_name)
    .bind(&hash)
    .bind(&now)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        if is_unique_violation(&e) {
            return ApiError::conflict("an account with this email already exists");
        }
        tracing::error!(?e, "database error inserting user");
        ApiError::internal("database error")
    })?;

    Ok((
        axum::http::StatusCode::CREATED,
        Json(json!({ "id": id, "email": email, "displayName": display_name })),
    ))
}

/// POST /api/auth/login — verifies credentials, returns `{ "token": "<jwt>" }`.
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<Value>, ApiError> {
    let email = body.email.trim().to_lowercase();
    if email.len() > 254 {
        return Err(ApiError::bad_request(
            "email must be at most 254 characters",
        ));
    }
    if body.password.len() > 128 {
        return Err(ApiError::bad_request(
            "password must be at most 128 characters",
        ));
    }
    let row: Option<(i64, String, String, String)> =
        sqlx::query_as("SELECT id, email, display_name, password_hash FROM users WHERE email = ?")
            .bind(&email)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "database error loading user for login");
                ApiError::internal("database error")
            })?;

    let Some((id, email, display_name, hash)) = row else {
        // Unknown email: still run an argon2 verification against a dummy
        // hash (constant-time mitigation against user-enumeration timing).
        let _ = verify_password_async(body.password.clone(), DUMMY_PASSWORD_HASH.to_string()).await;
        return Err(ApiError::unauthorized("invalid email or password"));
    };
    // CPU-bound argon2 verification runs in spawn_blocking.
    if !verify_password_async(body.password.clone(), hash.clone()).await {
        return Err(ApiError::unauthorized("invalid email or password"));
    }

    let user = crate::models::User {
        id,
        email,
        display_name,
        password_hash: hash,
        created_at: Utc::now(),
    };
    let token = mint_token(&state, &user)?;
    Ok(Json(json!({ "token": token })))
}

/// GET /api/me
pub async fn me(AuthUser(user): AuthUser) -> Json<Value> {
    Json(json!({ "id": user.id, "email": user.email, "displayName": user.display_name }))
}
