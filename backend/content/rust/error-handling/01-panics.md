---
slug: panics
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: error-handling
chapterTitle: "Chapter 20 — Error Handling"
chapterOrder: 20
title: "Panics"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, errors, panic]
---

## Learning Objective

By the end of this lesson, you will trigger panics with `panic!` and `assert!` and state when they fit.

## Explanation

A **panic** is Rust's unrecoverable-error path: the thread **unwinds** (runs destructors while exiting stack frames), prints a message with file and line, and exits nonzero. Trigger it with **`panic!("message")`** (aborts with your text) or **`assert!(condition, "message")`** (panics when false). Reserve panics for **programmer errors** (broken invariants, preconditions a caller must satisfy) — continuing would be worse than crashing. For **expected failures** (missing files, bad input), return `Result` so callers recover (next lesson).

## Why It Matters

Panics separate "the program is wrong, stop now" from "the world is uncertain, handle it" — loud where required, graceful elsewhere.

## Example

A greeting refusing an empty username: a violated precondition, so crashing beats a broken greeting.

## Code

```rust
fn greet(username: &str) -> String {
    assert!(!username.is_empty(), "username must not be empty");
    format!("hello, {username}")
}

fn main() {
    println!("{}", greet("Ada"));
}
```

## Code Explanation

- The assertion checks the **precondition**; `"Ada"` passes silently, `""` panics naming file and line.
- `main` prints `hello, Ada`; with `""`, `cargo run` aborts with your message.

## Common Mistakes

- **Panicking on user input**: missing configs should be `Result`s so `main` exits politely, not crashes.
- **Bare `assert!`**: always add the `, "why"` message for future debuggers.
- **`catch_unwind` for control flow**: it exists for FFI/test boundaries, not ordinary errors. Use `Result`.

## Best Practices

- Panic for broken invariants; return `Result` for fallible operations.
- Libraries return `Result`; applications decide whether to panic (via `expect` with context).

## Exercise

Call `greet("")` and run it. Copy the panic's first two lines, then swap `assert!` for `if` + `panic!` — same custom text?

## Solution

```rust
fn greet(username: &str) -> String {
    if username.is_empty() {
        panic!("username must not be empty, got {username:?}");
    }
    format!("hello, {username}")
}

fn main() {
    let result = std::panic::catch_unwind(|| greet(""));
    println!("caught panic: {}", result.is_err());
}
```

`catch_unwind` (converting a panic into `Err` instead of crashing) prints `caught panic: true`. Use `assert!` for one-liners, `if` + `panic!` for richer context.

## Mini Quiz

```yaml
- question: "What happens on panic?"
  options:
    - "Returns null, continues"
    - "Thread unwinds, prints location, aborts"
    - "Retries the operation"
    - "Nothing; it is a warning"
  answer: 1
  explanation: "Unwind runs destructors, reports file/line, exits nonzero."
- question: "Panic vs `Result`?"
  options:
    - "Panic for missing files"
    - "Panic for programmer errors; `Result` for expected failures"
    - "Always panic"
    - "Never panic"
  answer: 1
  explanation: "Bugs crash loudly; environmental failures stay handleable."
- question: "What does `catch_unwind` do?"
  options:
    - "Prevents future panics"
    - "Turns a closure's panic into an `Err` value"
    - "Logs to a file"
    - "Replaces `Result`"
  answer: 1
  explanation: "An isolation boundary for tests and FFI, not control flow."
```
