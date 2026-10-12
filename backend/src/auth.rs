//! Authentication: argon2 password hashing + JWT bearer tokens.
//!
//! Registration stores an argon2 hash (never the password). Login verifies
//! the hash and mints a short-lived JWT. Protected routes use the
//! [`AuthUser`] extractor, which rejects requests without a valid token.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, models::User, state::AppState};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64, // user id
    email: String,
    exp: usize, // expiry as unix timestamp
}

/// Token lifetime: 24 hours.
///
/// Rationale: a 24h TTL bounds the blast radius of a leaked token without
/// forcing learners to log in multiple times per day. Longer-lived sessions
/// would need refresh-token rotation, which is out of scope for this service.
const TOKEN_TTL_SECS: u64 = 24 * 3600;

/// Dummy argon2 hash used to equalize login timing for unknown emails.
/// The corresponding password is a fixed dummy value; verification always
/// runs (in `spawn_blocking`) and its result is discarded so unknown-email
/// logins take ~as long as real ones without leaking existence.
pub const DUMMY_PASSWORD_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$xaUKQLroRNyme69NWvRx4w$//QL+t45H1Fpt9loHRtbnk/Kzk/ivWogswJU97+YARc";

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| ApiError::internal("password hashing failed"))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

/// Argon2 hashing off the async runtime (CPU-bound, would block the event loop).
pub async fn hash_password_async(password: String) -> Result<String, ApiError> {
    tokio::task::spawn_blocking(move || hash_password(&password))
        .await
        .map_err(|e| {
            tracing::error!(?e, "password hashing task failed");
            ApiError::internal("password hashing failed")
        })?
}

/// Argon2 verification off the async runtime (CPU-bound).
pub async fn verify_password_async(password: String, hash: String) -> bool {
    tokio::task::spawn_blocking(move || verify_password(&password, &hash))
        .await
        .unwrap_or(false)
}

/// Decode a Bearer token to its `sub` (user id), or `None` if invalid/expired.
/// Shared by the [`AuthUser`] extractor and the optionally-authed `/api/courses`.
pub fn decode_user_id(jwt_secret: &str, token: &str) -> Option<i64> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .ok()?;
    Some(data.claims.sub)
}

/// Extract the Bearer token from headers and decode it to a user id.
pub fn bearer_user_id(jwt_secret: &str, headers: &axum::http::HeaderMap) -> Option<i64> {
    let token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))?;
    decode_user_id(jwt_secret, token)
}

pub fn mint_token(state: &AppState, user: &User) -> Result<String, ApiError> {
    let exp = (chrono::Utc::now() + chrono::Duration::seconds(TOKEN_TTL_SECS as i64)).timestamp()
        as usize;
    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiError::internal("token creation failed"))
}

/// Extractor: the authenticated user, or 401 without a valid Bearer token.
pub struct AuthUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: axum::extract::FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let token = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(ApiError::unauthorized("missing bearer token"))?;

        let user_id = decode_user_id(&app_state.jwt_secret, token)
            .ok_or(ApiError::unauthorized("invalid or expired token"))?;

        let user = sqlx::query_as::<_, (i64, String, String, String, String)>(
            "SELECT id, email, display_name, password_hash, created_at FROM users WHERE id = ?",
        )
        .bind(user_id)
        .fetch_optional(&app_state.db)
        .await
        .map_err(|e| {
            tracing::error!(?e, "database error loading auth user");
            ApiError::internal("database error")
        })?
        .map(
            |(id, email, display_name, password_hash, created_at)| User {
                id,
                email,
                display_name,
                password_hash,
                created_at: created_at.parse().unwrap_or_else(|_| chrono::Utc::now()),
            },
        )
        .ok_or(ApiError::unauthorized("user no longer exists"))?;

        Ok(AuthUser(user))
    }
}
