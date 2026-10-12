---
slug: tokio-tasks
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: concurrency
chapterTitle: "Chapter 22 — Concurrency"
chapterOrder: 22
title: "Tokio Tasks"
difficulty: intermediate
order: 7
estimatedMinutes: 12
tags: [rust, concurrency, tokio, tasks]
---

## Learning Objective

By the end of this lesson, you will overlap futures with `tokio::spawn` and `tokio::join!` and collect each result.

## Explanation

A Tokio **task** is lightweight work on the runtime's thread pool — thousands share a few OS threads. **`tokio::spawn(future)`** schedules one, returning a **`JoinHandle`** (`.await` it for `Result<T>`); the future must be `'static` (own its data, outliving the spawner). **`tokio::join!`** (macro polling several futures together, returning all results) suits fixed branch sets — two 50 ms timers finish in ~50 ms total. Handles report panics as `Err`, so `.expect("...")` with branch names keeps failures loud.

> Needs-tokio-dependency: excerpt — cargo project with `tokio` full features, `cargo run`, not bare `rustc`.

## Why It Matters

Spawn-plus-join is async's thread pattern at a fraction of the cost: concurrent requests, parallel reads, fan-out queries.

## Example

Two page fetches racing via `spawn`, joined and printed together.

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
    let first = tokio::spawn(fetch_title("home"));
    let second = tokio::spawn(fetch_title("about"));
    let (a, b) = tokio::join!(first, second);
    println!("{} / {}", a.expect("first failed"), b.expect("second failed"));
}
```

## Code Explanation

- `"home"` is `'static`, satisfying spawn's ownership rule (borrowed locals would be rejected).
- `join!` overlaps both timers: ~50 ms, not ~100 ms. Same output, twice as fast.

## Common Mistakes

- **Spawning borrows**: `spawn(fetch(local))` fails on lifetimes. Move owned/`'static` data in.
- **Dropping handles**: tasks run on detached — bind and join what you care about.
- **Blocking in tasks**: `std::thread::sleep` stalls the shared worker (lesson 8). Use async twins inside tasks.

## Best Practices

- `join!` for fixed branch sets; `spawn` + handle collection for dynamic counts.
- Pass owned `String`s and `Arc`s into spawned futures, never borrowed locals.

## Exercise

Spawn three fetches and join all three. Wall time at 50 ms each — still near ~50 ms? Verify with `Instant`.

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
    let (a, b, c) = tokio::join!(
        tokio::spawn(fetch_title("home")),
        tokio::spawn(fetch_title("about")),
        tokio::spawn(fetch_title("contact"))
    );
    println!("{} / {} / {} in {:?}", a.expect("a"), b.expect("b"), c.expect("c"), start.elapsed());
}
```

All three timers overlap (~50 ms vs ~150 ms sequential). `join!` takes handles directly — no intermediate bindings.

## Mini Quiz

```yaml
- question: "`tokio::spawn` returns? Get the value how?"
  options:
    - "Value directly"
    - "`JoinHandle`; `.await` it for `Result<T, JoinError>`"
    - "A thread ID"
    - "A channel"
  answer: 1
  explanation: "Spawn schedules; awaiting the handle retrieves the outcome."
- question: "Why must spawned futures be `'static`?"
  options:
    - "Speed"
    - "They may outlive the spawner, so no borrows of it allowed"
    - "Less memory"
    - "All futures are static"
  answer: 1
  explanation: "Detached scheduling plus borrowed locals could dangle."
- question: "Prefer `join!` over sequential `.await` when?"
  options:
    - "Always"
    - "Branches are independent and should overlap"
    - "Strict order needed"
    - "Never"
  answer: 1
  explanation: "`join!` polls together; sequential await sums times."
```
