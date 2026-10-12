---
slug: async-await
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: concurrency
chapterTitle: "Chapter 22 — Concurrency"
chapterOrder: 22
title: "Async and Await"
difficulty: intermediate
order: 6
estimatedMinutes: 12
tags: [rust, concurrency, async, await]
---

## Learning Objective

By the end of this lesson, you will write `async fn`, drive futures with `.await` on Tokio, and state why a runtime is required.

## Explanation

**Async Rust** juggles many waiting operations on few threads. An **`async fn`** returns a **`Future`** (pausable, resumable work) instead of the value — calling it runs nothing yet; futures are lazy. **`.await`** pauses the task until completion, freeing the thread meanwhile, and works only in `async` contexts. The **runtime** (Tokio) polls futures: `#[tokio::main]` (macro building the runtime around `async fn main`) boots it. `tokio::time::sleep` (non-blocking timer) stands in for real I/O. Install with `cargo add tokio --features full`.

> Needs-tokio-dependency: excerpts — run in a cargo project with `tokio` full features via `cargo run`, not bare `rustc`.

## Why It Matters

Servers handle thousands of connections on handfuls of threads: while one request waits on the database, others progress on the same core.

## Example

Fetch two "pages" (timer-simulated) sequentially and print both titles.

## Code

```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }
use tokio::time::{sleep, Duration};

async fn fetch_title(page: &str) -> String {
    sleep(Duration::from_millis(50)).await;
    format!("title of {page}")
}

#[tokio::main]
async fn main() {
    let first = fetch_title("home").await;
    let second = fetch_title("about").await;
    println!("{first} / {second}");
}
```

## Code Explanation

- The body runs only when `.await`ed; each await yields the thread during its 50 ms sleep (~100 ms total, sequential).
- `#[tokio::main]` polls both futures; output is `title of home / title of about`.

## Common Mistakes

- **`.await` in plain `main`**: `await is only allowed in async functions` — the entry must be `async` via `#[tokio::main]`.
- **Expecting overlap**: sequential awaits add up (~100 ms). Overlap needs tasks or `join!` (next lesson).
- **Unawaited futures**: called but never driven, they do nothing (compilers warn: unused `must_use` future).

## Best Practices

- Mark I/O-bound work `async`; number crunching belongs on threads or `spawn_blocking` (lesson 8).
- Borrow inputs (`&str`), return owned outputs (`String`).

## Exercise

Add a third fetch for `"contact"` and time all three with `std::time::Instant`. Sequential total near ~150 ms?

## Solution

```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }
use tokio::time::{sleep, Duration};

async fn fetch_title(page: &str) -> String {
    sleep(Duration::from_millis(50)).await;
    format!("title of {page}")
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let first = fetch_title("home").await;
    let second = fetch_title("about").await;
    let third = fetch_title("contact").await;
    println!("{first} / {second} / {third} in {:?}", start.elapsed());
}
```

Three sequential sleeps total around 150 ms or a little more — plain `.await` chains add up; `Instant`/`elapsed` makes the cost visible.

## Mini Quiz

```yaml
- question: "Calling an `async fn` produces?"
  options:
    - "The value, immediately"
    - "A lazy `Future`, run when awaited or spawned"
    - "A new OS thread"
    - "A compile error"
  answer: 1
  explanation: "Bodies execute as the runtime polls them after `.await`."
- question: "`.await` while waiting?"
  options:
    - "Blocks the thread"
    - "Parks the task; the thread runs others meanwhile"
    - "Spawns a runtime"
    - "Cancels the future"
  answer: 1
  explanation: "Cooperative yield — async's core win over blocking."
- question: "Why `#[tokio::main]`?"
  options:
    - "Output formatting"
    - "Builds the runtime polling futures; plain `main` cannot drive them"
    - "Multithreading only"
    - "Decoration"
  answer: 1
  explanation: "Futures need an executor; the macro wires one up."
```
