//! Integration tests: spin up the full Axum app in-process with a fresh
//! temp SQLite DB and a minimal fixture content dir per test.

use axum::http::StatusCode;
use axum_test::TestServer;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use ferris_academy_backend::{build_router, content_loader::ContentStore, db, state::AppState};

const TEST_JWT_SECRET: &str = "test-secret-0123456789abcdef";

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

const INTRO_RUST_MD: &str = r##"---
slug: intro-rust
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: getting-started
chapterTitle: "Getting Started"
chapterOrder: 1
title: "Intro to Rust"
difficulty: beginner
order: 1
estimatedMinutes: 8
tags: [rust, basics]
---

## Learning Objective

Learn what Rust is and why Ferris the crab loves systems programming.

## Explanation

Rust is a systems language focused on safety and speed without garbage collection.

## Why It Matters

Memory safety without runtime cost makes Rust ideal for reliable software.

## Example

A tiny hello world program shows the toolchain in action.

## Code

```rust
fn main() {
    println!("Hello, Ferris!");
}
```

## Code Explanation

The main function prints a greeting to standard output.

## Common Mistakes

Forgetting semicolons or misusing println formatting.

## Best Practices

Run cargo fmt and cargo clippy on every change.

## Exercise

Change the program to greet your own name.

## Solution

```rust
fn main() {
    println!("Hello, learner!");
}
```

## Mini Quiz

```yaml
- question: "What is Rust?"
  options:
    - "A garbage-collected scripting language"
    - "A systems language focused on safety and speed"
    - "A database"
    - "A text editor"
  answer: 1
  explanation: "Rust targets safety and speed without garbage collection."
- question: "Which command formats Rust code?"
  options:
    - "cargo fmt"
    - "cargo run"
    - "cargo test"
    - "cargo doc"
  answer: 0
  explanation: "cargo fmt runs the formatter."
```
"##;

const OWNERSHIP_MD: &str = r##"---
slug: ownership-basics
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: getting-started
chapterTitle: "Getting Started"
chapterOrder: 1
title: "Ownership Basics"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, ownership]
---

## Learning Objective

Understand how ownership moves values, with zebraquux as a memorable marker.

## Explanation

Every value has a single owner. When the owner goes out of scope the value is dropped. The zebraquux marker helps you spot moves in examples.

## Why It Matters

Ownership removes entire classes of memory bugs at compile time.

## Example

Moving a String invalidates the old binding.

## Code

```rust
fn main() {
    let s1 = String::from("zebraquux");
    let s2 = s1;
    println!("{s2}");
}
```

## Code Explanation

After `let s2 = s1`, `s1` can no longer be used because the String moved.

## Common Mistakes

Using a moved value or cloning when a borrow would do.

## Best Practices

Prefer borrows for read-only access and move only when transferring control.

## Exercise

Create a String, move it into a function, then try to use the original binding and read the compiler error.

## Solution

```rust
fn takes(s: String) {
    println!("{s}");
}
fn main() {
    let s = String::from("hi");
    takes(s);
}
```

## Mini Quiz

```yaml
- question: "What happens when a value moves?"
  options:
    - "The old binding can no longer be used"
    - "Nothing changes"
    - "The value is silently copied"
  answer: 0
  explanation: "Moves invalidate the old binding."
- question: "Which operator borrows immutably?"
  options:
    - "*"
    - "&"
    - "move"
  answer: 1
  explanation: "& creates a shared borrow."
```
"##;

const BORROWING_MD: &str = r##"---
slug: borrowing-intro
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: borrowing
chapterTitle: "Borrowing"
chapterOrder: 2
title: "Borrowing Intro"
difficulty: beginner
order: 1
estimatedMinutes: 7
tags: [rust, borrowing]
---

## Learning Objective

Learn how references let you use values without transferring control.

## Explanation

References with `&` let functions read data while the original binding stays valid.

## Why It Matters

Borrowing enables efficient zero-copy code that the compiler checks for safety.

## Example

Passing a reference keeps the caller usable afterwards.

## Code

```rust
fn len(s: &String) -> usize {
    s.len()
}
fn main() {
    let s = String::from("hello");
    println!("{}", len(&s));
    println!("{s}");
}
```

## Code Explanation

`&s` creates a shared reference; `s` remains valid after the call.

## Common Mistakes

Mixing mutable and shared references in ways the compiler rejects.

## Best Practices

Take `&str` over `&String` in function arguments for flexibility.

## Exercise

Write a function that takes `&String` and returns its length, then call it twice.

## Solution

```rust
fn len(s: &String) -> usize {
    s.len()
}
fn main() {
    let s = String::from("hey");
    assert_eq!(len(&s), 3);
}
```

## Mini Quiz

```yaml
- question: "What does &String mean in a function argument?"
  options:
    - "A shared reference to a String"
    - "A mutable copy"
    - "A heap allocation"
  answer: 0
  explanation: "&String is a shared borrow."
- question: "Can the caller still use s after len(&s)?"
  options:
    - "No, it moved"
    - "Yes, borrows keep the owner valid"
    - "Only with unsafe"
  answer: 1
  explanation: "Shared borrows do not move."
```
"##;

fn write_fixtures(content_dir: &std::path::Path) {
    std::fs::write(content_dir.join("01-intro-rust.md"), INTRO_RUST_MD).unwrap();
    std::fs::write(content_dir.join("02-ownership-basics.md"), OWNERSHIP_MD).unwrap();
    std::fs::write(content_dir.join("03-borrowing-intro.md"), BORROWING_MD).unwrap();
}

async fn setup_server() -> TestServer {
    let n = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let base: PathBuf =
        std::env::temp_dir().join(format!("ferris-api-test-{}-{n}", std::process::id()));
    let content_dir = base.join("content");
    std::fs::create_dir_all(&content_dir).unwrap();
    write_fixtures(&content_dir);

    let db_path = base.join("test.db");
    let db_url = format!(
        "sqlite:{}?mode=rwc",
        db_path.to_string_lossy().replace('\\', "/")
    );
    let pool = db::connect(&db_url).await.expect("db connect + migrate");

    let store = ContentStore::load(&content_dir);
    assert_eq!(store.order.len(), 3, "fixture lessons must load");
    assert!(
        store.malformed.is_empty(),
        "fixtures must not be malformed: {:?}",
        store.malformed
    );

    let state = AppState {
        db: pool,
        content: Arc::new(store),
        jwt_secret: TEST_JWT_SECRET.to_string(),
    };
    let router = build_router(state);
    TestServer::new(router).expect("test server builds")
}

async fn register_and_login(
    server: &TestServer,
    email: &str,
    password: &str,
    display_name: &str,
) -> String {
    let reg = server
        .post("/api/auth/register")
        .json(&json!({
            "email": email,
            "password": password,
            "displayName": display_name,
        }))
        .await;
    assert_eq!(
        reg.status_code(),
        StatusCode::CREATED,
        "register failed: {}",
        reg.text()
    );
    let login = server
        .post("/api/auth/login")
        .json(&json!({ "email": email, "password": password }))
        .await;
    assert_eq!(
        login.status_code(),
        StatusCode::OK,
        "login failed: {}",
        login.text()
    );
    let body: Value = login.json();
    body["token"]
        .as_str()
        .expect("login returns token")
        .to_string()
}

fn all_lesson_slugs(body: &Value) -> Vec<String> {
    body["tracks"]
        .as_array()
        .expect("tracks array")
        .iter()
        .flat_map(|t| t["chapters"].as_array().cloned().unwrap_or_default())
        .flat_map(|c| c["lessons"].as_array().cloned().unwrap_or_default())
        .filter_map(|l| l["slug"].as_str().map(String::from))
        .collect()
}

// 1. GET /api/health → 200 {status:"ok"}
#[tokio::test]
async fn health_returns_ok() {
    let server = setup_server().await;
    let resp = server.get("/api/health").await;
    assert_eq!(resp.status_code(), StatusCode::OK);
    let body: Value = resp.json();
    assert_eq!(body["status"], json!("ok"));
}

// 2. GET /api/courses → tree with fixture lessons, completedLessonSlugs=[]
#[tokio::test]
async fn courses_returns_tree_without_completions_when_anonymous() {
    let server = setup_server().await;
    let resp = server.get("/api/courses").await;
    assert_eq!(resp.status_code(), StatusCode::OK);
    let body: Value = resp.json();
    let slugs = all_lesson_slugs(&body);
    assert!(
        slugs.contains(&"intro-rust".to_string()),
        "slugs: {slugs:?}"
    );
    assert!(
        slugs.contains(&"ownership-basics".to_string()),
        "slugs: {slugs:?}"
    );
    assert!(
        slugs.contains(&"borrowing-intro".to_string()),
        "slugs: {slugs:?}"
    );
    assert_eq!(slugs.len(), 3);
    let completed = body["completedLessonSlugs"]
        .as_array()
        .expect("completedLessonSlugs array");
    assert!(completed.is_empty());
    assert!(!body["tracks"].as_array().unwrap().is_empty());
}

// 3. GET /api/lessons/:slug → full content, quiz WITHOUT answer field
#[tokio::test]
async fn lesson_detail_has_content_and_no_quiz_answers() {
    let server = setup_server().await;
    let resp = server.get("/api/lessons/ownership-basics").await;
    assert_eq!(resp.status_code(), StatusCode::OK);
    let body: Value = resp.json();
    assert_eq!(body["slug"], json!("ownership-basics"));
    assert_eq!(body["title"], json!("Ownership Basics"));
    assert!(body["sections"]["explanation"].is_string());
    assert!(body["navigation"].is_object());
    let quiz = body["quiz"].as_array().expect("quiz array");
    assert_eq!(quiz.len(), 2);
    for q in quiz {
        assert!(q["question"].is_string());
        assert!(q["options"].is_array());
        assert!(q["explanation"].is_string());
        assert!(
            q.get("answer").is_none(),
            "quiz must not expose answer: {q}"
        );
    }
    // Raw text must not leak the answer key either.
    let raw = resp.text();
    assert!(
        !raw.contains("\"answer\""),
        "response must not contain answer field"
    );
}

// 4. GET /api/lessons/nope → 404 JSON {error:{message}}
#[tokio::test]
async fn lesson_unknown_slug_returns_404_error_json() {
    let server = setup_server().await;
    let resp = server.get("/api/lessons/nope").await;
    assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
    let body: Value = resp.json();
    assert!(body["error"]["message"].is_string());
}

// 5a. GET /api/search?q= → finds fixture
#[tokio::test]
async fn search_finds_fixture_lesson() {
    let server = setup_server().await;
    let resp = server.get("/api/search?q=zebraquux").await;
    assert_eq!(resp.status_code(), StatusCode::OK);
    let body: Value = resp.json();
    let results = body["results"].as_array().expect("results array");
    assert!(!results.is_empty(), "search must find fixture");
    let slugs: Vec<String> = results
        .iter()
        .filter_map(|r| r["lesson"]["slug"].as_str().map(String::from))
        .collect();
    assert!(slugs.contains(&"ownership-basics".to_string()));
}

// 5b. empty q → empty results
#[tokio::test]
async fn search_empty_query_returns_empty_results() {
    let server = setup_server().await;
    let resp = server.get("/api/search?q=").await;
    assert_eq!(resp.status_code(), StatusCode::OK);
    let body: Value = resp.json();
    assert_eq!(body["results"], json!([]));

    let resp2 = server.get("/api/search").await;
    assert_eq!(resp2.status_code(), StatusCode::OK);
    let body2: Value = resp2.json();
    assert_eq!(body2["results"], json!([]));
}

// 5c. q > 200 chars → 400
#[tokio::test]
async fn search_too_long_query_returns_400() {
    let server = setup_server().await;
    let long: String = "a".repeat(201);
    let resp = server.get("/api/search").add_query_param("q", long).await;
    assert_eq!(resp.status_code(), StatusCode::BAD_REQUEST);
    let body: Value = resp.json();
    assert!(body["error"]["message"].is_string());
}

// 6a. POST /api/auth/register → 201
#[tokio::test]
async fn register_success_returns_201() {
    let server = setup_server().await;
    let resp = server
        .post("/api/auth/register")
        .json(&json!({
            "email": "alice@example.com",
            "password": "correct-horse-123",
            "displayName": "Alice",
        }))
        .await;
    assert_eq!(resp.status_code(), StatusCode::CREATED);
    let body: Value = resp.json();
    assert_eq!(body["email"], json!("alice@example.com"));
    assert_eq!(body["displayName"], json!("Alice"));
    assert!(body["id"].is_number());
}

// 6b. duplicate → 409
#[tokio::test]
async fn register_duplicate_returns_409() {
    let server = setup_server().await;
    let payload = json!({
        "email": "bob@example.com",
        "password": "correct-horse-123",
        "displayName": "Bob",
    });
    let first = server.post("/api/auth/register").json(&payload).await;
    assert_eq!(first.status_code(), StatusCode::CREATED);
    let second = server.post("/api/auth/register").json(&payload).await;
    assert_eq!(second.status_code(), StatusCode::CONFLICT);
    let body: Value = second.json();
    assert!(body["error"]["message"].is_string());
}

// 6c. bad email → 400
#[tokio::test]
async fn register_bad_email_returns_400() {
    let server = setup_server().await;
    let resp = server
        .post("/api/auth/register")
        .json(&json!({
            "email": "not-an-email",
            "password": "correct-horse-123",
            "displayName": "Nope",
        }))
        .await;
    assert_eq!(resp.status_code(), StatusCode::BAD_REQUEST);
    let body: Value = resp.json();
    assert!(body["error"]["message"].is_string());
}

// 6d. short password → 400
#[tokio::test]
async fn register_short_password_returns_400() {
    let server = setup_server().await;
    let resp = server
        .post("/api/auth/register")
        .json(&json!({
            "email": "short@example.com",
            "password": "short",
            "displayName": "Short",
        }))
        .await;
    assert_eq!(resp.status_code(), StatusCode::BAD_REQUEST);
    let body: Value = resp.json();
    assert!(body["error"]["message"].is_string());
}

// 6e. overlong fields → 400
#[tokio::test]
async fn register_overlong_fields_return_400() {
    let server = setup_server().await;

    let overlong_email = format!("{}@b.cd", "a".repeat(250));
    assert!(overlong_email.len() > 254);
    let resp = server
        .post("/api/auth/register")
        .json(&json!({
            "email": overlong_email,
            "password": "correct-horse-123",
            "displayName": "Long",
        }))
        .await;
    assert_eq!(resp.status_code(), StatusCode::BAD_REQUEST);

    let resp = server
        .post("/api/auth/register")
        .json(&json!({
            "email": "longpass@example.com",
            "password": "x".repeat(129),
            "displayName": "Long",
        }))
        .await;
    assert_eq!(resp.status_code(), StatusCode::BAD_REQUEST);

    let resp = server
        .post("/api/auth/register")
        .json(&json!({
            "email": "longname@example.com",
            "password": "correct-horse-123",
            "displayName": "y".repeat(65),
        }))
        .await;
    assert_eq!(resp.status_code(), StatusCode::BAD_REQUEST);
}

// 7a. POST /api/auth/login → 200 token
#[tokio::test]
async fn login_success_returns_token() {
    let server = setup_server().await;
    let token =
        register_and_login(&server, "carol@example.com", "correct-horse-123", "Carol").await;
    assert!(!token.is_empty());
    assert!(token.split('.').count() == 3, "expected JWT, got {token}");
}

// 7b. wrong password → 401
#[tokio::test]
async fn login_wrong_password_returns_401() {
    let server = setup_server().await;
    let reg = server
        .post("/api/auth/register")
        .json(&json!({
            "email": "dave@example.com",
            "password": "correct-horse-123",
            "displayName": "Dave",
        }))
        .await;
    assert_eq!(reg.status_code(), StatusCode::CREATED);
    let resp = server
        .post("/api/auth/login")
        .json(&json!({ "email": "dave@example.com", "password": "wrong-password-xyz" }))
        .await;
    assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    let body: Value = resp.json();
    assert!(body["error"]["message"].is_string());
}

// 7c. unknown user → 401
#[tokio::test]
async fn login_unknown_user_returns_401() {
    let server = setup_server().await;
    let resp = server
        .post("/api/auth/login")
        .json(&json!({ "email": "ghost@example.com", "password": "whatever-12345" }))
        .await;
    assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    let body: Value = resp.json();
    assert!(body["error"]["message"].is_string());
}

// 8a. GET /api/me without token → 401
#[tokio::test]
async fn me_without_token_returns_401() {
    let server = setup_server().await;
    let resp = server.get("/api/me").await;
    assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    let body: Value = resp.json();
    assert!(body["error"]["message"].is_string());
}

// 8b. with token → user
#[tokio::test]
async fn me_with_token_returns_user() {
    let server = setup_server().await;
    let token = register_and_login(&server, "erin@example.com", "correct-horse-123", "Erin").await;
    let resp = server.get("/api/me").authorization_bearer(&token).await;
    assert_eq!(resp.status_code(), StatusCode::OK);
    let body: Value = resp.json();
    assert_eq!(body["email"], json!("erin@example.com"));
    assert_eq!(body["displayName"], json!("Erin"));
    assert!(body["id"].is_number());
}

// 9. GET /api/progress without token → 401
#[tokio::test]
async fn progress_without_token_returns_401() {
    let server = setup_server().await;
    let resp = server.get("/api/progress").await;
    assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    let body: Value = resp.json();
    assert!(body["error"]["message"].is_string());
}

// 10. POST /lessons/:slug/complete → 200; progress shows 1; DELETE undoes; unknown → 404
#[tokio::test]
async fn complete_progress_and_uncomplete_flow() {
    let server = setup_server().await;
    let token =
        register_and_login(&server, "frank@example.com", "correct-horse-123", "Frank").await;

    let resp = server
        .post("/api/lessons/intro-rust/complete")
        .authorization_bearer(&token)
        .await;
    assert_eq!(resp.status_code(), StatusCode::OK);
    let body: Value = resp.json();
    assert_eq!(body["slug"], json!("intro-rust"));
    assert_eq!(body["completed"], json!(true));

    let progress = server
        .get("/api/progress")
        .authorization_bearer(&token)
        .await;
    assert_eq!(progress.status_code(), StatusCode::OK);
    let pbody: Value = progress.json();
    assert_eq!(pbody["completedCount"], json!(1));
    let slugs = pbody["completedLessonSlugs"]
        .as_array()
        .expect("completedLessonSlugs array");
    assert_eq!(slugs.len(), 1);
    assert!(slugs.contains(&json!("intro-rust")));

    let del = server
        .delete("/api/lessons/intro-rust/complete")
        .authorization_bearer(&token)
        .await;
    assert_eq!(del.status_code(), StatusCode::OK);
    let dbody: Value = del.json();
    assert_eq!(dbody["completed"], json!(false));

    let progress2 = server
        .get("/api/progress")
        .authorization_bearer(&token)
        .await;
    assert_eq!(progress2.status_code(), StatusCode::OK);
    let pbody2: Value = progress2.json();
    assert_eq!(pbody2["completedCount"], json!(0));
    assert_eq!(pbody2["completedLessonSlugs"], json!([]));
}

#[tokio::test]
async fn complete_unknown_slug_returns_404() {
    let server = setup_server().await;
    let token = register_and_login(&server, "gina@example.com", "correct-horse-123", "Gina").await;
    let resp = server
        .post("/api/lessons/no-such-lesson/complete")
        .authorization_bearer(&token)
        .await;
    assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
    let body: Value = resp.json();
    assert!(body["error"]["message"].is_string());
}

// 11a. quiz correct → full score
#[tokio::test]
async fn quiz_attempt_correct_answers_full_score() {
    let server = setup_server().await;
    let token =
        register_and_login(&server, "heidi@example.com", "correct-horse-123", "Heidi").await;
    // Fixture answers for ownership-basics are [0, 1].
    let resp = server
        .post("/api/quizzes/ownership-basics/attempt")
        .authorization_bearer(&token)
        .json(&json!({ "answers": [0, 1] }))
        .await;
    assert_eq!(resp.status_code(), StatusCode::CREATED);
    let body: Value = resp.json();
    assert_eq!(body["slug"], json!("ownership-basics"));
    assert_eq!(body["score"], json!(2));
    assert_eq!(body["totalQuestions"], json!(2));
    assert_eq!(body["percentage"], json!(100));
}

// 11b. wrong → partial
#[tokio::test]
async fn quiz_attempt_wrong_answers_partial_score() {
    let server = setup_server().await;
    let token = register_and_login(&server, "ivan@example.com", "correct-horse-123", "Ivan").await;
    let resp = server
        .post("/api/quizzes/ownership-basics/attempt")
        .authorization_bearer(&token)
        .json(&json!({ "answers": [0, 0] }))
        .await;
    assert_eq!(resp.status_code(), StatusCode::CREATED);
    let body: Value = resp.json();
    assert_eq!(body["score"], json!(1));
    assert_eq!(body["totalQuestions"], json!(2));
    assert_eq!(body["percentage"], json!(50));
}

// 11c. malformed body → 4xx
#[tokio::test]
async fn quiz_attempt_malformed_body_returns_4xx() {
    let server = setup_server().await;
    let token = register_and_login(&server, "judy@example.com", "correct-horse-123", "Judy").await;
    let resp = server
        .post("/api/quizzes/ownership-basics/attempt")
        .authorization_bearer(&token)
        .json(&json!({}))
        .await;
    assert!(
        resp.status_code().is_client_error(),
        "expected 4xx, got {}",
        resp.status_code()
    );

    let resp2 = server
        .post("/api/quizzes/ownership-basics/attempt")
        .authorization_bearer(&token)
        .json(&json!({ "answers": "not-an-array" }))
        .await;
    assert!(
        resp2.status_code().is_client_error(),
        "expected 4xx, got {}",
        resp2.status_code()
    );
}

// 12. GET /api/courses WITH token after completing → completedLessonSlugs populated
#[tokio::test]
async fn courses_with_token_shows_completions() {
    let server = setup_server().await;
    let token = register_and_login(&server, "karl@example.com", "correct-horse-123", "Karl").await;

    let before = server
        .get("/api/courses")
        .authorization_bearer(&token)
        .await;
    assert_eq!(before.status_code(), StatusCode::OK);
    let bbody: Value = before.json();
    assert_eq!(bbody["completedLessonSlugs"], json!([]));

    let done = server
        .post("/api/lessons/ownership-basics/complete")
        .authorization_bearer(&token)
        .await;
    assert_eq!(done.status_code(), StatusCode::OK);

    let after = server
        .get("/api/courses")
        .authorization_bearer(&token)
        .await;
    assert_eq!(after.status_code(), StatusCode::OK);
    let abody: Value = after.json();
    let slugs = abody["completedLessonSlugs"]
        .as_array()
        .expect("completedLessonSlugs array");
    assert!(slugs.contains(&json!("ownership-basics")), "got {slugs:?}");
}
