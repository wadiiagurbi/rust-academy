//! Course content loader: Markdown + YAML frontmatter → structured lessons.
//!
//! One file per lesson under `content/`:
//!
//! ````markdown
//! ---
//! slug: what-is-rust
//! track: rust
//! trackTitle: "Rust"
//! trackOrder: 1
//! chapter: getting-started
//! chapterTitle: "Chapter 1 — Getting Started"
//! chapterOrder: 1
//! title: "What is Rust?"
//! difficulty: beginner
//! order: 1
//! estimatedMinutes: 8
//! tags: [rust, basics]
//! ---
//!
//! ## Learning Objective
//! ...
//! ## Mini Quiz
//! ```yaml
//! - question: "..."
//!   options: [...]
//!   answer: 1
//!   explanation: "..."
//! ```
//! ````
//!
//!
//! Malformed files are collected (not fatal) and served as 404, so one bad
//! file can never take the whole course down.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::models::{
    Chapter, LessonContent, LessonSections, LessonSummary, Navigation, QuizQuestion,
    QuizQuestionPublic, Track,
};

/// Frontmatter fields every lesson must declare.
#[derive(Debug, serde::Deserialize)]
struct Frontmatter {
    slug: String,
    track: String,
    #[serde(rename = "trackTitle")]
    track_title: String,
    #[serde(rename = "trackOrder")]
    track_order: i64,
    chapter: String,
    #[serde(rename = "chapterTitle")]
    chapter_title: String,
    #[serde(rename = "chapterOrder")]
    chapter_order: i64,
    title: String,
    difficulty: String,
    order: i64,
    #[serde(rename = "estimatedMinutes")]
    estimated_minutes: Option<i64>,
    tags: Option<Vec<String>>,
}

/// Required `##` sections, mapped to their JSON key.
const REQUIRED_SECTIONS: [(&str, &str); 11] = [
    ("Learning Objective", "learningObjective"),
    ("Explanation", "explanation"),
    ("Why It Matters", "whyItMatters"),
    ("Example", "example"),
    ("Code", "code"),
    ("Code Explanation", "codeExplanation"),
    ("Common Mistakes", "commonMistakes"),
    ("Best Practices", "bestPractices"),
    ("Exercise", "exercise"),
    ("Solution", "solution"),
    ("Mini Quiz", "miniQuiz"),
];

#[derive(Debug)]
pub struct LoadedLesson {
    pub content: LessonContent,
    /// Server-side copy including quiz answers (for grading).
    pub quiz_answers: Vec<QuizQuestion>,
}

pub struct ContentStore {
    /// Slug → lesson, in global course order.
    pub lessons: HashMap<String, LoadedLesson>,
    pub order: Vec<String>,
    pub tracks: Vec<Track>,
    /// Paths that failed to parse (for logging/tests).
    pub malformed: Vec<String>,
}

impl ContentStore {
    pub fn load(dir: &Path) -> Self {
        let mut raw: Vec<RawLesson> = Vec::new();
        let mut malformed: Vec<String> = Vec::new();

        if dir.is_dir() {
            let mut paths: Vec<PathBuf> = Vec::new();
            for entry in WalkDir::new(dir) {
                match entry {
                    Ok(e) => {
                        let p = e.into_path();
                        if p.extension().is_some_and(|ext| ext == "md") {
                            paths.push(p);
                        }
                    }
                    Err(e) => {
                        tracing::warn!(?e, "error walking content directory");
                    }
                }
            }
            paths.sort();

            let mut seen: HashSet<String> = HashSet::new();
            for path in paths {
                match parse_lesson_file(&path) {
                    Ok(lesson) => {
                        if seen.insert(lesson.fm.slug.clone()) {
                            raw.push(lesson);
                        } else {
                            // Duplicate slug: first file wins, later ones are malformed.
                            malformed.push(format!(
                                "{}: duplicate slug \"{}\"",
                                path.display(),
                                lesson.fm.slug
                            ));
                        }
                    }
                    Err(reason) => malformed.push(format!("{}: {reason}", path.display())),
                }
            }
        }

        raw.sort_by(|a, b| {
            (a.track_order, a.chapter_order, a.order).cmp(&(
                b.track_order,
                b.chapter_order,
                b.order,
            ))
        });

        // Summaries in global order (1-based).
        let mut summaries: HashMap<String, LessonSummary> = HashMap::new();
        let mut order: Vec<String> = Vec::new();
        for (i, r) in raw.iter().enumerate() {
            order.push(r.fm.slug.clone());
            summaries.insert(
                r.fm.slug.clone(),
                LessonSummary {
                    slug: r.fm.slug.clone(),
                    title: r.fm.title.clone(),
                    track: r.fm.track.clone(),
                    chapter: r.fm.chapter.clone(),
                    chapter_title: r.fm.chapter_title.clone(),
                    difficulty: r.fm.difficulty.clone(),
                    order: r.fm.order,
                    global_order: (i + 1) as i64,
                },
            );
        }

        let tracks = build_tree(&raw, &summaries);

        let mut lessons: HashMap<String, LoadedLesson> = HashMap::new();
        for (i, r) in raw.into_iter().enumerate() {
            let prev = if i > 0 {
                order
                    .get(i - 1)
                    .and_then(|slug| summaries.get(slug))
                    .cloned()
            } else {
                None
            };
            let next = order
                .get(i + 1)
                .and_then(|slug| summaries.get(slug))
                .cloned();
            let public_quiz: Vec<QuizQuestionPublic> = r
                .quiz
                .iter()
                .map(|q| QuizQuestionPublic {
                    question: q.question.clone(),
                    options: q.options.clone(),
                    explanation: q.explanation.clone(),
                })
                .collect();
            let content = LessonContent {
                slug: r.fm.slug.clone(),
                track: r.fm.track.clone(),
                track_title: r.fm.track_title.clone(),
                chapter: r.fm.chapter.clone(),
                chapter_title: r.fm.chapter_title.clone(),
                title: r.fm.title.clone(),
                difficulty: r.fm.difficulty.clone(),
                order: r.fm.order,
                estimated_minutes: r.fm.estimated_minutes.filter(|v| *v > 0).unwrap_or(10),
                tags: r.fm.tags.clone().unwrap_or_default(),
                sections: r.sections,
                quiz: public_quiz,
                navigation: Navigation {
                    previous: prev,
                    next,
                },
            };
            lessons.insert(
                r.fm.slug.clone(),
                LoadedLesson {
                    content,
                    quiz_answers: r.quiz,
                },
            );
        }

        Self {
            lessons,
            order,
            tracks,
            malformed,
        }
    }

    /// Case-insensitive search over title, chapter, tags and body text.
    pub fn search(&self, query: &str) -> Vec<(LessonSummary, String)> {
        let needle = query.trim().to_lowercase();
        if needle.is_empty() {
            return Vec::new();
        }
        let mut results = Vec::new();
        for slug in &self.order {
            let Some(lesson) = self.lessons.get(slug) else {
                continue;
            };
            let c = &lesson.content;
            let body = format!(
                "{} {} {} {} {} {} {} {} {} {} {}",
                c.sections.learning_objective,
                c.sections.explanation,
                c.sections.why_it_matters,
                c.sections.example,
                c.sections.code,
                c.sections.code_explanation,
                c.sections.common_mistakes,
                c.sections.best_practices,
                c.sections.exercise,
                c.sections.solution,
                c.tags.join(" ")
            );
            let haystacks = [
                c.title.to_lowercase(),
                c.chapter_title.to_lowercase(),
                body.to_lowercase(),
            ];
            let matched = haystacks.iter().find(|h| h.contains(&needle));
            if let Some(text) = matched {
                let pos = text.find(&needle).unwrap_or(0);
                let start = pos.saturating_sub(40);
                let end = (pos + 100).min(text.len());
                // Keep excerpt on char boundaries.
                let mut s = start;
                while s < pos && !text.is_char_boundary(s) {
                    s += 1;
                }
                let mut e = end;
                while e > pos && !text.is_char_boundary(e) {
                    e -= 1;
                }
                let prefix = if s > 0 { "…" } else { "" };
                // Only mark truncation when we actually cut the tail off.
                let truncated = e < text.len();
                let suffix = if truncated { "…" } else { "" };
                let slice = text.get(s..e).or_else(|| text.get(s..)).unwrap_or("");
                let excerpt = format!("{}{}{}", prefix, slice.trim(), suffix);
                if let Some(summary) = self
                    .tracks
                    .iter()
                    .flat_map(|t| &t.chapters)
                    .flat_map(|ch| &ch.lessons)
                    .find(|l| l.slug == *slug)
                    .cloned()
                    .or_else(|| {
                        // Fallback: rebuild from content (same data).
                        Some(LessonSummary {
                            slug: c.slug.clone(),
                            title: c.title.clone(),
                            track: c.track.clone(),
                            chapter: c.chapter.clone(),
                            chapter_title: c.chapter_title.clone(),
                            difficulty: c.difficulty.clone(),
                            order: c.order,
                            global_order: 0,
                        })
                    })
                {
                    results.push((summary, excerpt));
                }
            }
            if results.len() > 20 {
                results.truncate(20);
            }
        }
        results
    }
}

struct RawLesson {
    fm: Frontmatter,
    track_order: i64,
    chapter_order: i64,
    order: i64,
    sections: LessonSections,
    quiz: Vec<QuizQuestion>,
}

fn build_tree(raw: &[RawLesson], summaries: &HashMap<String, LessonSummary>) -> Vec<Track> {
    // `raw` is already globally sorted; grouping preserves that order.
    let mut track_pos: HashMap<String, usize> = HashMap::new();
    let mut chapter_pos: HashMap<(String, String), usize> = HashMap::new();
    let mut tracks: Vec<Track> = Vec::new();

    for r in raw {
        let Some(summary) = summaries.get(&r.fm.slug) else {
            continue;
        };
        let ti = *track_pos.entry(r.fm.track.clone()).or_insert_with(|| {
            tracks.push(Track {
                key: r.fm.track.clone(),
                title: r.fm.track_title.clone(),
                order: r.track_order,
                chapters: Vec::new(),
            });
            tracks.len() - 1
        });
        let key = (r.fm.track.clone(), r.fm.chapter.clone());
        let ci = *chapter_pos.entry(key).or_insert_with(|| {
            tracks[ti].chapters.push(Chapter {
                key: r.fm.chapter.clone(),
                title: r.fm.chapter_title.clone(),
                order: r.chapter_order,
                lessons: Vec::new(),
            });
            tracks[ti].chapters.len() - 1
        });
        tracks[ti].chapters[ci].lessons.push(summary.clone());
    }

    tracks
}

/// Split a Markdown body on `## Heading` lines into (heading, content) pairs.
fn split_sections(body: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut current_body = String::new();

    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some(h) = current_heading.take() {
                out.push((h, current_body.trim().to_string()));
                current_body = String::new();
            }
            current_heading = Some(heading.trim().to_string());
        } else if current_heading.is_some() {
            current_body.push_str(line);
            current_body.push('\n');
        }
    }
    if let Some(h) = current_heading {
        out.push((h, current_body.trim().to_string()));
    }
    out
}

/// Extract `(language, code)` from a single fenced block, else plain text.
fn extract_code_block(text: &str) -> (String, String) {
    let trimmed = text.trim();
    if let Some(rest) = trimmed.strip_prefix("```") {
        let mut lines = rest.lines();
        let lang = lines.next().unwrap_or("").trim().to_string();
        let mut code_lines: Vec<&str> = lines.collect();
        // Drop closing fence.
        if code_lines.last().is_some_and(|l| l.trim() == "```") {
            code_lines.pop();
        }
        let language = if lang.is_empty() { "text" } else { &lang };
        return (language.to_string(), code_lines.join("\n"));
    }
    ("text".to_string(), trimmed.to_string())
}

fn parse_lesson_file(path: &Path) -> Result<RawLesson, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| format!("cannot read: {e}"))?;

    // Frontmatter: ---\n<yaml>\n---\n<body>
    let mut parts = raw.splitn(3, "---");
    let _ = parts.next(); // empty before first fence
    let fm_raw = parts.next().ok_or("missing YAML frontmatter block")?;
    let body = parts.next().ok_or("missing YAML frontmatter block")?;

    let fm: Frontmatter =
        serde_yaml::from_str(fm_raw).map_err(|e| format!("invalid YAML frontmatter: {e}"))?;
    if fm.slug.trim().is_empty()
        || fm.track.trim().is_empty()
        || fm.chapter.trim().is_empty()
        || fm.title.trim().is_empty()
    {
        return Err("missing required frontmatter field".to_string());
    }
    if !matches!(
        fm.difficulty.as_str(),
        "beginner" | "intermediate" | "advanced"
    ) {
        return Err(format!(
            "invalid difficulty \"{}\": must be beginner|intermediate|advanced",
            fm.difficulty
        ));
    }
    if fm.track_order < 0 || fm.chapter_order < 0 || fm.order < 0 {
        return Err("trackOrder, chapterOrder and order must be >= 0".to_string());
    }
    // estimatedMinutes <= 0 falls back to 10 at build time (see below).

    let sections = split_sections(body);
    let mut by_key: HashMap<&str, String> = HashMap::new();
    for (heading, content) in &sections {
        if let Some((_, key)) = REQUIRED_SECTIONS.iter().find(|(h, _)| h == heading) {
            by_key.insert(key, content.clone());
        }
    }
    let missing: Vec<&str> = REQUIRED_SECTIONS
        .iter()
        .filter(|(_, key)| !by_key.contains_key(*key))
        .map(|(_, key)| *key)
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "missing required section(s): {}",
            missing.join(", ")
        ));
    }

    let get_section = |key: &str| -> Result<String, String> {
        by_key
            .get(key)
            .cloned()
            .ok_or_else(|| format!("missing required section: {key}"))
    };

    let (code_language, code) = extract_code_block(&get_section("code")?);
    let quiz = parse_quiz(&get_section("miniQuiz")?)?;

    Ok(RawLesson {
        track_order: fm.track_order,
        chapter_order: fm.chapter_order,
        order: fm.order,
        sections: LessonSections {
            learning_objective: get_section("learningObjective")?,
            explanation: get_section("explanation")?,
            why_it_matters: get_section("whyItMatters")?,
            example: get_section("example")?,
            code,
            code_language,
            code_explanation: get_section("codeExplanation")?,
            common_mistakes: get_section("commonMistakes")?,
            best_practices: get_section("bestPractices")?,
            exercise: get_section("exercise")?,
            solution: get_section("solution")?,
        },
        quiz,
        fm,
    })
}

#[derive(Debug, serde::Deserialize)]
struct QuizItem {
    question: String,
    options: Vec<String>,
    answer: usize,
    explanation: Option<String>,
}

fn parse_quiz(section: &str) -> Result<Vec<QuizQuestion>, String> {
    let (_, yaml_text) = extract_code_block(section);
    let items: Vec<QuizItem> =
        serde_yaml::from_str(&yaml_text).map_err(|e| format!("invalid quiz YAML: {e}"))?;
    if items.is_empty() {
        return Err("quiz must contain at least one question".to_string());
    }
    let mut out = Vec::new();
    for item in items {
        if item.options.len() < 2 {
            return Err("each quiz question needs at least 2 options".to_string());
        }
        if item.answer >= item.options.len() {
            return Err("quiz answer index out of range".to_string());
        }
        out.push(QuizQuestion {
            question: item.question,
            options: item.options,
            answer: item.answer,
            explanation: item.explanation.unwrap_or_default(),
        });
    }
    Ok(out)
}
