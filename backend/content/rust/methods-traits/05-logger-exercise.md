---
slug: logger-trait
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: methods-traits
chapterTitle: "Chapter 8 — Methods and Traits"
chapterOrder: 8
title: "Logger Exercise"
difficulty: beginner
order: 5
estimatedMinutes: 18
tags: [rust, traits, exercise, testing]
---

## Learning Objective

By the end of this lesson, you will build a `Logger` trait with two implementors and test it.

## Explanation

A **logger** records messages. **Unit tests** (functions marked `#[test]`) check behavior with `assert_eq!`, failing on mismatch. `cargo test` runs them all. The plan: `Logger` with `fn log(&mut self, msg: &str)`, a console printer, and a memory logger holding `Vec<String>` (growable owned-string list) for assertions.

## Why It Matters

Trait plus tests is the daily loop: shared behavior, two implementors, automated contract checks.

## Example

Both loggers take the same calls; only memory keeps history, so tests assert stored lines without reading consoles.

## Code

```rust
trait Logger { fn log(&mut self, msg: &str); }
struct MemoryLogger { lines: Vec<String> }
impl Logger for MemoryLogger {
    fn log(&mut self, msg: &str) { self.lines.push(msg.to_string()); }
}

fn main() {
    let mut logger = MemoryLogger { lines: Vec::new() };
    logger.log("boot");
    logger.log("ready");
    println!("{}", logger.lines.len());
}
```

## Code Explanation

- `&mut self` allows pushing; `to_string()` owns the message.
- Two logs store two lines, printing `2`.

## Common Mistakes

- **Using `&self`**: pushing needs mutable borrow. Fix: keep `&mut self`.
- **Storing `&str`**: borrows need lifetimes. Fix: store owned `String`.

## Best Practices

- Keep traits minimal; ship a memory implementor beside console/file ones for tests.

## Exercise

Create a cargo project `logger_task` with `cargo new logger_task`. Add the trait plus both loggers so the criteria pass.

```rust
// src/main.rs starter — replace todo!() with your code:
trait Logger { fn log(&mut self, msg: &str); }
struct MemoryLogger { lines: Vec<String> }
impl Logger for MemoryLogger {
    fn log(&mut self, msg: &str) { todo!() }
}
fn main() { todo!() }
```

Your `cargo test` criteria: two logs store both in order; fresh logger starts empty.

## Solution

```rust
trait Logger { fn log(&mut self, msg: &str); }
struct MemoryLogger { lines: Vec<String> }
impl Logger for MemoryLogger {
    fn log(&mut self, msg: &str) { self.lines.push(msg.to_string()); }
}
fn main() {
    let mut logger = MemoryLogger { lines: Vec::new() };
    logger.log("boot");
    println!("{}", logger.lines.len());
}
#[test]
fn stores_in_order() {
    let mut logger = MemoryLogger { lines: Vec::new() };
    logger.log("boot");
    logger.log("ready");
    assert_eq!(logger.lines, [String::from("boot"), String::from("ready")]);
}
#[test]
fn starts_empty() { assert_eq!(MemoryLogger { lines: Vec::new() }.lines.len(), 0); }
```

## Mini Quiz

```yaml
- question: "Why does `log` take `&mut self`?"
  options:
    - "It changes logger state"
    - "It borrows faster"
    - "It blocks logging"
    - "Every trait needs it"
  answer: 0
  explanation: "Storing a line mutates the logger."
- question: "Why store `String` not `&str`?"
  options:
    - "Owned values avoid lifetime bookkeeping"
    - "Strings print louder"
    - "Slices can't log"
    - "No difference"
  answer: 0
  explanation: "Owned strings live with the logger."
- question: "What does `cargo test` do?"
  options:
    - "Runs all `#[test]` functions"
    - "Formats code"
    - "Publishes the crate"
    - "Cleans builds"
  answer: 0
  explanation: "It builds the harness and reports results."
```
