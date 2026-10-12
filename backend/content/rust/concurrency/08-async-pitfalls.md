---
slug: async-pitfalls
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: concurrency
chapterTitle: "Chapter 22 — Concurrency"
chapterOrder: 22
title: "Async Pitfalls"
difficulty: advanced
order: 8
estimatedMinutes: 12
tags: [rust, concurrency, async, pitfalls]
---

## Learning Objective

By the end of this lesson, you will avoid blocking the runtime, keep futures `Send` across `.await`, and offload sync work.

## Explanation

Three classic traps. **Blocking the runtime**: sync waits (`std::thread::sleep`, blocking I/O) inside a task stall the shared worker — *all* its tasks freeze. Use async twins (`tokio::time::sleep`). **Non-`Send` futures**: holding a non-thread-safe guard (e.g. std `MutexGuard`) across `.await` fails, since Tokio tasks migrate between workers and everything held across must be **`Send`** (movable between threads). Fix: drop guards before awaiting, or use `tokio::sync::Mutex` (whose guard is `Send`). **Heavy sync work** (hashing, compression) belongs on **`spawn_blocking`** (a side pool for blocking code, awaitable without stalling workers).

> Needs-tokio-dependency: excerpt — cargo project with `tokio` full features, `cargo run`, not bare `rustc`.

## Why It Matters

One blocking call can halve server throughput; one held guard can refuse to compile before release. Recognize the shapes early.

## Example

A slow sync checksum offloaded to `spawn_blocking` while a timer ticks beside it — neither stalls the other.

## Code

```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }
use tokio::time::{sleep, Duration};

fn slow_checksum(data: &str) -> u64 {
    std::thread::sleep(Duration::from_millis(50));
    data.bytes().map(|b| b as u64).sum()
}

#[tokio::main]
async fn main() {
    let ticker = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;
        "tick"
    });
    let sum = tokio::task::spawn_blocking(|| slow_checksum("abc")).await;
    println!("{} checksum {}", ticker.await.expect("tick"), sum.expect("sum"));
}
```

## Code Explanation

- `slow_checksum` blocks 50 ms (`"abc"` sums 97+98+99 = 294); direct task use would wedge a worker.
- `spawn_blocking` runs it on the side pool; the 10 ms ticker still fires on time. Prints `tick checksum 294`.

## Common Mistakes

- **`MutexGuard` across `.await`**: `future cannot be sent between threads safely`. Scope locks to end before awaits, or switch mutexes.
- **`thread::sleep` in async**: grep reviews for it inside `async`; replace with `tokio::time::sleep`.
- **`spawn_blocking` for async code**: it bridges *sync* work in — async I/O stays directly awaited.

## Best Practices

- No blocking calls in tasks; keep guard lifetimes off `.await` boundaries.
- Isolate CPU bursts and legacy sync APIs behind `spawn_blocking`, commented with the blocked duration.

## Exercise

Await `ticker` before the `spawn_blocking` handle. Why does overlap survive any await order? Then predict the wall time of a direct (non-offloaded) call.

## Solution

```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }
use tokio::time::{sleep, Duration};

fn slow_checksum(data: &str) -> u64 {
    data.bytes().map(|b| b as u64).sum()
}

#[tokio::main]
async fn main() {
    let ticker = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;
        "tick"
    });
    let tick = ticker.await.expect("tick");
    let sum = tokio::task::spawn_blocking(|| slow_checksum("abc")).await.expect("sum");
    println!("{tick} checksum {sum}");
}
```

Both futures were already spawned, so awaiting only *collects* — order cannot un-overlap them. Direct calls wedge the worker (fine at toy scale, fatal under load).

## Mini Quiz

```yaml
- question: "`thread::sleep` inside a task?"
  options:
    - "Panics"
    - "Blocks the shared worker, freezing its tasks"
    - "Sleeps forever"
    - "Deadlocks"
  answer: 1
  explanation: "Workers are shared; blocking waits stall everything queued."
- question: "std guard across `.await` fails why?"
  options:
    - "Mutexes banned in async"
    - "Guard is not `Send`; tasks migrate between workers"
    - "Guards expire"
    - "It compiles fine"
  answer: 1
  explanation: "Everything held across await must be Send-safe."
- question: "`spawn_blocking` is for?"
  options:
    - "Faster async"
    - "Blocking/sync work on a side pool, awaitable without stalling"
    - "Manual threads"
    - "Deliberate blocking"
  answer: 1
  explanation: "It bridges sync code into async contexts cheaply."
```
