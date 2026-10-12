//! Shared data models: course tree DTOs and database rows.
//!
//! The course tree (`Track` → `Chapter` → `LessonSummary`) is built from
//! Markdown files by [`crate::content_loader`]. Database rows map 1:1 to
//! the SQL tables created in `migrations/`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Course content DTOs (served as JSON, mirror the reference platform)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct Track {
    pub key: String,
    pub title: String,
    pub order: i64,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Chapter {
    pub key: String,
    pub title: String,
    pub order: i64,
    pub lessons: Vec<LessonSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LessonSummary {
    pub slug: String,
    pub title: String,
    pub track: String,
    pub chapter: String,
    #[serde(rename = "chapterTitle")]
    pub chapter_title: String,
    pub difficulty: String,
    pub order: i64,
    #[serde(rename = "globalOrder")]
    pub global_order: i64,
}

/// Full lesson body. `quiz` never contains the correct `answer` index —
/// answers are graded server-side only.
#[derive(Debug, Clone, Serialize)]
pub struct LessonContent {
    pub slug: String,
    pub track: String,
    #[serde(rename = "trackTitle")]
    pub track_title: String,
    pub chapter: String,
    #[serde(rename = "chapterTitle")]
    pub chapter_title: String,
    pub title: String,
    pub difficulty: String,
    pub order: i64,
    #[serde(rename = "estimatedMinutes")]
    pub estimated_minutes: i64,
    pub tags: Vec<String>,
    pub sections: LessonSections,
    pub quiz: Vec<QuizQuestionPublic>,
    pub navigation: Navigation,
}

#[derive(Debug, Clone, Serialize)]
pub struct LessonSections {
    #[serde(rename = "learningObjective")]
    pub learning_objective: String,
    pub explanation: String,
    #[serde(rename = "whyItMatters")]
    pub why_it_matters: String,
    pub example: String,
    pub code: String,
    #[serde(rename = "codeLanguage")]
    pub code_language: String,
    #[serde(rename = "codeExplanation")]
    pub code_explanation: String,
    #[serde(rename = "commonMistakes")]
    pub common_mistakes: String,
    #[serde(rename = "bestPractices")]
    pub best_practices: String,
    pub exercise: String,
    pub solution: String,
}

/// Quiz question as served to the frontend: no `answer` field.
#[derive(Debug, Clone, Serialize)]
pub struct QuizQuestionPublic {
    pub question: String,
    pub options: Vec<String>,
    pub explanation: String,
}

/// Quiz question with the answer (kept server-side for grading).
#[derive(Debug, Clone)]
pub struct QuizQuestion {
    pub question: String,
    pub options: Vec<String>,
    pub answer: usize,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Navigation {
    pub previous: Option<LessonSummary>,
    pub next: Option<LessonSummary>,
}

// ---------------------------------------------------------------------------
// Request DTOs (validated on the way in)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct QuizAttemptRequest {
    pub answers: Vec<i64>,
}

// ---------------------------------------------------------------------------
// Database rows
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub display_name: String,
    /// Argon2 hash. Kept on the model so auth flows can re-verify without
    /// a second query; not serialized (no `Serialize` impl on purpose).
    #[allow(dead_code)]
    pub password_hash: String,
    #[allow(dead_code)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProgressView {
    #[serde(rename = "completedCount")]
    pub completed_count: usize,
    #[serde(rename = "totalLessons")]
    pub total_lessons: usize,
    #[serde(rename = "completionPercentage")]
    pub completion_percentage: u32,
    #[serde(rename = "lastLessonSlug")]
    pub last_lesson_slug: Option<String>,
    #[serde(rename = "quizAttemptsCount")]
    pub quiz_attempts_count: usize,
    #[serde(rename = "quizAverageScore")]
    pub quiz_average_score: u32,
}
