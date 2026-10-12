---
slug: channels
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: concurrency
chapterTitle: "Chapter 22 — Concurrency"
chapterOrder: 22
title: "Channels"
difficulty: intermediate
order: 2
estimatedMinutes: 12
tags: [rust, concurrency, channels]
---

## Learning Objective

By the end of this lesson, you will pass values between threads with an `mpsc` channel and receive via `for` or `recv()`.

## Explanation

A **channel** is a typed message queue between threads. `mpsc::channel()` (**mpsc** = multiple-producer, single-consumer) creates a `(Sender, Receiver)` pair. **`Sender`** (transmitting end; `Clone` so many threads hold one) offers `send(value)`, which moves the value in. **`Receiver`** (single collecting end) offers `recv()` (blocks until a message or disconnect) and iterates: `for msg in rx` yields until every `Sender` drops — the **disconnect signal** ending the loop. Sending *moves* values, so each message has exactly one owner: race-free by construction.

## Why It Matters

Channels mean "share by communicating": hand off ownership instead of guarding shared memory. Most Rust designs start here.

## Example

Two workers send partial sums; `main` collects both through the receiver.

## Code

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for part in [1..=5, 6..=10] {
        let worker = tx.clone();
        thread::spawn(move || {
            let sum: i32 = part.sum();
            worker.send(sum).expect("receiver gone");
        });
    }
    drop(tx);
    let mut total = 0;
    for received in rx {
        total += received;
    }
    println!("total: {total}");
}
```

## Code Explanation

- Each worker gets a cloned `Sender` moved into its thread; `drop(tx)` closes the original so the `for` loop ends.
- Partials 15 + 40 arrive in any order; printed `total: 55` is fixed since addition commutes.

## Common Mistakes

- **Keeping the original sender**: the receive loop blocks forever. `drop(tx)` explicitly.
- **Moving `tx`**: second `spawn(move)` fails with `use of moved value`. Clone per producer.
- **Assuming order**: arrivals are nondeterministic — design so order is irrelevant (or add sequence numbers).

## Best Practices

- Let sender-drop end receive loops; no sentinel poison messages.
- One channel per message kind; distinct types keep protocols obvious.

## Exercise

Add a third worker for `11..=15`. New total? Then swap the `for` loop for two `rx.recv()` calls — what breaks with three workers?

## Solution

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    for part in [1..=5, 6..=10, 11..=15] {
        let worker = tx.clone();
        thread::spawn(move || {
            worker.send(part.sum()).expect("receiver gone");
        });
    }
    drop(tx);
    let mut total = 0;
    for received in rx {
        total += received;
    }
    println!("total: {total}");
}
```

15 + 40 + 65 = `total: 120`. Fixed `recv()` counts cannot adapt to producer counts; the `for` loop can.

## Mini Quiz

```yaml
- question: "`mpsc` implies?"
  options:
    - "Many senders clone `Sender`; one `Receiver` collects"
    - "Priority-ordered delivery"
    - "Many receivers share load"
    - "Pre-shared memory"
  answer: 0
  explanation: "Multiple-producer, single-consumer: Sender is Clone, Receiver unique."
- question: "How does `for msg in rx` end?"
  options:
    - "Timeout"
    - "Every `Sender` dropped (disconnect)"
    - "Never"
    - "Momentary empty queue"
  answer: 1
  explanation: "All-senders-dropped disconnects the channel — hence `drop(tx)`."
- question: "Why race-free?"
  options:
    - "Internal locks only"
    - "`send` moves ownership: one owner at a time"
    - "Messages copy"
    - "Same-thread receivers"
  answer: 1
  explanation: "Ownership transfer replaces shared mutable state."
```
