//! Shared application state: DB pool, loaded course content, JWT secret.

use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

use crate::content_loader::ContentStore;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub content: Arc<ContentStore>,
    pub jwt_secret: String,
}
