---
slug: send-sync
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: concurrency
chapterTitle: "Chapter 22 — Concurrency"
chapterOrder: 22
title: "Send and Sync"
difficulty: intermediate
order: 3
estimatedMinutes: 12
tags: [rust, concurrency, send, sync]
---

## Learning Objective

By the end of this lesson, you will bound generics with `Send`/`Sync` and fix an `Rc`-across-threads error with `Arc`.

## Explanation

**`Send`** is a marker trait (no methods, bound-only) promising a value may *move* to another thread; **`Sync`** promises safe *sharing by `&T`* across threads. Most types are both; **`Rc`** (single-threaded reference counting, non-atomic) and `RefCell` are not. `thread::spawn` demands `Send` closures; sharing `&T` demands `T: Sync`. Generics can require it too: `T: Send + 'static` (where **`'static`** means owning all data, no stack borrows) accepts only thread-movable values. **`Arc`** (atomic reference counting: `Rc`'s thread-safe sibling) is the fix — `Arc<T>` is `Send + Sync` exactly when `T` is.

## Why It Matters

`Send`/`Sync` turn "safe to share?" from folklore into compiler errors. Reading them — and reaching for `Arc` — is fearless concurrency's core skill.

## Example

A generic move-to-thread helper, plus `Arc` sharing a value with one worker.

## Code

```rust
use std::sync::Arc;
use std::thread;

fn run_in_thread<T>(value: T) -> T
where
    T: Send + 'static,
{
    thread::spawn(move || value).join().expect("thread panicked")
}

fn main() {
    println!("{}", run_in_thread(String::from("hello")));
    let shared = Arc::new(21);
    let worker = Arc::clone(&shared);
    let handle = thread::spawn(move || *worker * 2);
    println!("shared: {} doubled: {}", shared, handle.join().expect("panicked"));
}
```

## Code Explanation

- `String` passes the bound (owned, sendable); an `Rc` or stack borrow would fail naming the trait.
- `Arc::clone` shares ownership; the worker dereferences to 42 while `main` still reads 21. Swap `Arc` for `Rc` and compilation stops you.

## Common Mistakes

- **Shipping `Rc`**: the canonical `Send` error. Fix: `Arc` (plus `Mutex` if mutating).
- **Borrowing into `spawn`**: `&local` fails (`does not live long enough`). Move owned/`Arc` data.
- **Confusing the traits**: `Send` (movable) without `Sync` (sharable), e.g. `Cell<T>` — read which one the error names.

## Best Practices

- Bound thread-taking generics with `Send + 'static` so errors point at callers.
- Default shared ownership to `Arc`; `Rc` only for proven single-threaded code.

## Exercise

Swap `Arc::new(21)` for `Rc::new(21)` (import `std::rc::Rc`). Copy the trait named in the error, then revert.

## Solution

```rust
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    let single = Rc::new(21);
    println!("single-threaded Rc: {single}");
    let shared = Arc::new(21);
    let worker = Arc::clone(&shared);
    let handle = std::thread::spawn(move || *worker * 2);
    println!("shared: {} doubled: {}", shared, handle.join().expect("panicked"));
}
```

`Rc` fails with `` `Rc<i32>` cannot be sent between threads safely `` (a `Send` error); `Arc` prints both lines. Same shape, different atomicity, different verdict.

## Mini Quiz

```yaml
- question: "`Send` and `Sync` promise?"
  options:
    - "Movable across threads / sharable by reference"
    - "Faster clone / synchronous run"
    - "Both single-threaded"
    - "Network sending"
  answer: 0
  explanation: "Send governs transfer; Sync governs shared `&T` access."
- question: "Why is `Arc` thread-safe but `Rc` not?"
  options:
    - "Smaller"
    - "Atomic counting vs racy plain counting"
    - "Rc cannot clone"
    - "Arc skips counting"
  answer: 1
  explanation: "Atomic updates are race-free, earning Send+Sync."
- question: "`'static` on thread data means?"
  options:
    - "Lives whole run"
    - "Owns its data; no borrows of the spawning frame"
    - "Runs forever"
    - "Speed hint"
  answer: 1
  explanation: "Borrowed locals could vanish mid-task; owned data prevents that."
```
