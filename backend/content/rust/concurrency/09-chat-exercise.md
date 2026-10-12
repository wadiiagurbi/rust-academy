---
slug: broadcast-chat
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: concurrency
chapterTitle: "Chapter 22 — Concurrency"
chapterOrder: 22
title: "Exercise: Broadcast Chat"
difficulty: advanced
order: 9
estimatedMinutes: 20
tags: [rust, concurrency, exercise]
---

## Learning Objective

By the end of this lesson, you will fan out chat messages with a Tokio broadcast channel, verified by `cargo test`.

## Explanation

This is a **cargo task**: `cargo new chat_lab`, `cargo add tokio --features full`, implement the relay in `src/main.rs`. A **broadcast channel** (`tokio::sync::broadcast`: every active **receiver** — an independent subscription — gets its own copy of each message) fits chat: one sender, each client reading the same line. `send` returns the receiver count (errors with zero receivers, mapped to 0 here); `recv().await` collects one copy per subscriber. `todo!()` compiles while the test fails loudly.

> Needs-tokio-dependency: `cargo test` in the Tokio project, not bare `rustc`.

## Why It Matters

Publish-subscribe fan-out drives chat servers, dashboards, event buses — one primitive replacing per-client queues.

## Example

```text
test both_clients_receive ... ok
delivered to 0
```

## Code

```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }
use tokio::sync::broadcast;

async fn relay(chat: &broadcast::Sender<&'static str>, line: &'static str) -> usize { todo!() }

#[tokio::main]
async fn main() {
    let (chat, rx) = broadcast::channel(8);
    drop(rx);
    println!("delivered to {}", relay(&chat, "hello").await);
}

#[tokio::test]
async fn both_clients_receive() {
    let (chat, rx) = broadcast::channel(8);
    drop(rx);
    let mut ada = chat.subscribe();
    let mut grace = chat.subscribe();
    relay(&chat, "hello").await;
    assert_eq!(ada.recv().await.expect("ada missed it"), "hello");
    assert_eq!(grace.recv().await.expect("grace missed it"), "hello");
}
```

## Code Explanation

- `broadcast::channel(8)` buses up to 8 buffered messages; `subscribe()` mints independent handles. `drop(rx)` releases the original handle so counts reflect exactly the named subscribers (2 in the test, 0 in `main`).

## Common Mistakes

- **Zero receivers**: `send` errors with none subscribed. Map to 0 or keep a guard handle alive.
- **Cross-client order asserts**: each receiver is ordered, but clients advance independently — assert per-client sequences only.
- **Lagged receivers**: past capacity, slow readers get `Lagged` errors — re-subscribe or count drops, never panic blindly.

## Best Practices

- Size capacity for bursts; return the `send` count as a live-client census for logging. Keep relay socket-free (sender in, count out) so `#[tokio::test]` covers it directly.

## Exercise

Implement `relay` so the test passes and `cargo run` prints `delivered to 0`. Why 0 in `main` but 2 in the test?

## Solution

```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }
use tokio::sync::broadcast;

async fn relay(chat: &broadcast::Sender<&'static str>, line: &'static str) -> usize {
    chat.send(line).unwrap_or(0)
}

#[tokio::main]
async fn main() {
    let (chat, rx) = broadcast::channel(8);
    drop(rx);
    println!("delivered to {}", relay(&chat, "hello").await);
}
```

`send` fans out to every live receiver and reports the count: 0 in `main` (original dropped, none subscribed — `unwrap_or(0)` maps the error), 2 in the test (ada + grace).

## Mini Quiz

```yaml
- question: "Broadcast vs `mpsc`?"
  options:
    - "No senders"
    - "Every receiver gets a copy; mpsc delivers each message once"
    - "Sync only"
    - "Identical"
  answer: 1
  explanation: "Broadcast is publish-subscribe; mpsc is single delivery."
- question: "`broadcast::Sender::send` returns?"
  options:
    - "Nothing"
    - "Receiver count (errors when zero)"
    - "The message"
    - "A receiver"
  answer: 1
  explanation: "The count doubles as a live-subscriber census."
- question: "Why `drop(rx)` in `main`?"
  options:
    - "Slows the sender"
    - "So the count reflects only real subscribers (here: none)"
    - "Faster tests"
    - "Required syntax"
  answer: 1
  explanation: "Every live handle counts — including the original."
```
