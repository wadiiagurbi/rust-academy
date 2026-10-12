//! Course + lesson + search handlers (public, with optional auth on /api/courses).

use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{auth::bearer_user_id, error::ApiError, state::AppState};

/// GET /api/courses — full sidebar tree (+ completed slugs when authed).
/// Stays public: without a valid Bearer token `completedLessonSlugs` is empty.
pub async fn courses(State(state): State<AppState>, headers: HeaderMap) -> Json<Value> {
    let completed: Vec<String> = match bearer_user_id(&state.jwt_secret, &headers) {
        Some(user_id) => {
            match sqlx::query_scalar::<_, String>(
                "SELECT lesson_slug FROM lesson_completions WHERE user_id = ?",
            )
            .bind(user_id)
            .fetch_all(&state.db)
            .await
            {
                Ok(rows) => rows,
                Err(e) => {
                    tracing::error!(?e, "database error loading completions for courses");
                    Vec::new()
                }
            }
        }
        None => Vec::new(),
    };
    Json(json!({
        "tracks": state.content.tracks,
        "completedLessonSlugs": completed,
    }))
}

/// GET /api/lessons/:slug — structured lesson (quiz answers stripped).
pub async fn lesson(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let lesson = state
        .content
        .lessons
        .get(&slug)
        .ok_or_else(|| ApiError::not_found(format!("no lesson found for slug \"{slug}\"")))?;
    Ok(Json(serde_json::to_value(&lesson.content).map_err(
        |_| ApiError::internal("failed to serialize lesson"),
    )?))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

/// GET /api/search?q= — lesson search with excerpts.
pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Value>, ApiError> {
    let q = params.q.unwrap_or_default();
    if q.chars().count() > 200 {
        return Err(ApiError::bad_request("search query too long"));
    }
    let results: Vec<Value> = state
        .content
        .search(&q)
        .into_iter()
        .take(20)
        .map(|(lesson, excerpt)| json!({ "lesson": lesson, "excerpt": excerpt }))
        .collect();
    Ok(Json(json!({ "results": results })))
}

/// GET /api/health
pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "app": "ferris-academy-backend" }))
}
