//! Progress + quiz handlers (authenticated).

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use serde_json::{json, Value};

use crate::{
    auth::AuthUser,
    error::ApiError,
    models::{ProgressView, QuizAttemptRequest},
    state::AppState,
};

/// POST /api/lessons/:slug/complete
pub async fn complete(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<Json<Value>, ApiError> {
    if !state.content.lessons.contains_key(&slug) {
        return Err(ApiError::not_found(format!(
            "no lesson found for slug \"{slug}\""
        )));
    }
    let now = Utc::now().to_rfc3339();
    let mut tx = state.db.begin().await.map_err(|e| {
        tracing::error!(?e, "database error starting complete transaction");
        ApiError::internal("database error")
    })?;
    sqlx::query(
        "INSERT INTO lesson_completions (user_id, lesson_slug, completed_at) VALUES (?, ?, ?)
         ON CONFLICT(user_id, lesson_slug) DO UPDATE SET completed_at = excluded.completed_at",
    )
    .bind(user.id)
    .bind(&slug)
    .bind(&now)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(?e, "database error inserting lesson completion");
        ApiError::internal("database error")
    })?;

    // Remember where the learner left off.
    sqlx::query("UPDATE users SET last_lesson_slug = ? WHERE id = ?")
        .bind(&slug)
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!(?e, "database error updating last lesson slug");
            ApiError::internal("database error")
        })?;
    tx.commit().await.map_err(|e| {
        tracing::error!(?e, "database error committing complete transaction");
        ApiError::internal("database error")
    })?;

    Ok(Json(json!({ "slug": slug, "completed": true })))
}

/// DELETE /api/lessons/:slug/complete
pub async fn uncomplete(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<Json<Value>, ApiError> {
    if !state.content.lessons.contains_key(&slug) {
        return Err(ApiError::not_found(format!(
            "no lesson found for slug \"{slug}\""
        )));
    }
    sqlx::query("DELETE FROM lesson_completions WHERE user_id = ? AND lesson_slug = ?")
        .bind(user.id)
        .bind(&slug)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(?e, "database error deleting lesson completion");
            ApiError::internal("database error")
        })?;

    // If the removed lesson was the recorded "left off" spot, recompute it
    // from the most recent remaining completion (or clear it when none remain).
    let stored: Option<String> =
        sqlx::query_scalar("SELECT last_lesson_slug FROM users WHERE id = ?")
            .bind(user.id)
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "database error reading last lesson slug");
                ApiError::internal("database error")
            })?;
    if stored.as_deref() == Some(slug.as_str()) {
        // completed_at is RFC3339 text, so lexicographic order == chronological.
        let next: Option<String> = sqlx::query_scalar(
            "SELECT lesson_slug FROM lesson_completions WHERE user_id = ? ORDER BY completed_at DESC, rowid DESC LIMIT 1",
        )
        .bind(user.id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(?e, "database error recomputing last lesson slug");
            ApiError::internal("database error")
        })?;
        sqlx::query("UPDATE users SET last_lesson_slug = ? WHERE id = ?")
            .bind(&next)
            .bind(user.id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "database error clearing last lesson slug");
                ApiError::internal("database error")
            })?;
    }
    Ok(Json(json!({ "slug": slug, "completed": false })))
}

/// GET /api/progress
pub async fn progress(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Value>, ApiError> {
    let mut completed: Vec<String> =
        sqlx::query_scalar("SELECT lesson_slug FROM lesson_completions WHERE user_id = ?")
            .bind(user.id)
            .fetch_all(&state.db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "database error loading completions");
                ApiError::internal("database error")
            })?;

    // Ignore completions for lessons no longer present in content so removed
    // lessons never inflate counts, then sort for a deterministic response.
    completed.retain(|slug| state.content.lessons.contains_key(slug));
    completed.sort();

    let attempts: Vec<(i64, i64)> =
        sqlx::query_as("SELECT score, total FROM quiz_attempts WHERE user_id = ?")
            .bind(user.id)
            .fetch_all(&state.db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "database error loading quiz attempts");
                ApiError::internal("database error")
            })?;

    let total_lessons = state.content.order.len();
    let completed_count = completed.len();
    let percentage = if total_lessons == 0 {
        0
    } else {
        ((completed_count as f64 / total_lessons as f64) * 100.0).round() as u32
    };
    let quiz_average = if attempts.is_empty() {
        0
    } else {
        let sum: f64 = attempts
            .iter()
            .map(|(s, t)| *s as f64 / (*t).max(1) as f64)
            .sum();
        ((sum / attempts.len() as f64) * 100.0).round() as u32
    };

    // Last completed lesson = "where I left off". completed_at is RFC3339
    // text so lexicographic ORDER BY matches chronological order.
    let last: Option<String> = sqlx::query_scalar(
        "SELECT lesson_slug FROM lesson_completions WHERE user_id = ? ORDER BY completed_at DESC, rowid DESC LIMIT 1",
    )
    .bind(user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "database error loading last lesson slug");
        ApiError::internal("database error")
    })?
    .filter(|slug| state.content.lessons.contains_key(slug));

    let view = ProgressView {
        completed_count,
        total_lessons,
        completion_percentage: percentage,
        last_lesson_slug: last,
        quiz_attempts_count: attempts.len(),
        quiz_average_score: quiz_average,
    };
    let mut value =
        serde_json::to_value(&view).map_err(|_| ApiError::internal("serialization error"))?;
    value["completedLessonSlugs"] =
        serde_json::to_value(&completed).map_err(|_| ApiError::internal("serialization error"))?;
    Ok(Json(value))
}

/// POST /api/quizzes/:slug/attempt — grade server-side, persist, return breakdown.
pub async fn quiz_attempt(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    Json(body): Json<QuizAttemptRequest>,
) -> Result<(axum::http::StatusCode, Json<Value>), ApiError> {
    let lesson = state
        .content
        .lessons
        .get(&slug)
        .ok_or_else(|| ApiError::not_found(format!("no lesson found for slug \"{slug}\"")))?;
    if lesson.quiz_answers.is_empty() {
        return Err(ApiError::not_found("this lesson has no quiz"));
    }

    let mut score: i64 = 0;
    let mut breakdown = Vec::new();
    for (i, q) in lesson.quiz_answers.iter().enumerate() {
        let selected: i64 = body.answers.get(i).copied().unwrap_or(-1);
        let correct = selected == q.answer as i64;
        if correct {
            score += 1;
        }
        breakdown.push(json!({
            "question": q.question,
            "selected": selected,
            "correct": q.answer,
            "isCorrect": correct,
            "explanation": q.explanation,
        }));
    }
    let total = lesson.quiz_answers.len() as i64;
    let percentage = ((score as f64 / total as f64) * 100.0).round() as i64;

    let now = Utc::now().to_rfc3339();
    let answers_json = serde_json::to_string(&breakdown).map_err(|e| {
        tracing::error!(?e, "failed to serialize quiz answers");
        ApiError::internal("serialization error")
    })?;
    sqlx::query(
        "INSERT INTO quiz_attempts (user_id, lesson_slug, score, total, answers_json, created_at)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(user.id)
    .bind(&slug)
    .bind(score)
    .bind(total)
    .bind(&answers_json)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "database error inserting quiz attempt");
        ApiError::internal("database error")
    })?;

    Ok((
        axum::http::StatusCode::CREATED,
        Json(json!({
            "slug": slug,
            "score": score,
            "totalQuestions": total,
            "percentage": percentage,
            "answers": breakdown,
        })),
    ))
}
