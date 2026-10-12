---
slug: cli-tests
track: final-project
trackTitle: "Final Project"
trackOrder: 2
chapter: final-project
chapterTitle: "Chapter 27 — Final Project: CLI Task Manager"
chapterOrder: 27
title: "Tests for the Task Manager"
difficulty: intermediate
order: 7
estimatedMinutes: 11
tags: [rust, cli, final-project, testing]
---

## Learning Objective

By the end of this lesson, you will test task helpers and the store with `cargo test`.

## Explanation

A **test** (a function marked `#[test]` that panics when a check fails) locks in behavior. `assert_eq!` (a macro that panics when left and right differ) compares expected vs real values. `cargo test` (Cargo's runner that builds and runs every `#[test]`) reports passes and failures.

Fast checks live in `task.rs`: building with `Task::new` and flipping `mark_done`. Slower checks live in `store.rs` against a temporary database: an **in-memory database** (SQLite started with `:memory:`, holding rows only while the test runs) inserts then lists without touching real files.

## Why It Matters

Four commands share one store; a one-line SQL slip breaks `add`, `list`, and `done` together. Tests catch that before you demo the binary.

## Example

A model test builds `Task::new(1, "x")` and checks `done == false`; a store test opens `:memory:`, calls `init_db`, inserts one row, and asserts `list_all` returns exactly one title.

## Code

```rust
struct Task {
    id: i64,
    title: String,
    done: bool,
}

impl Task {
    fn new(id: i64, title: String) -> Self {
        Self { id, title, done: false }
    }
}

#[test]
fn new_task_starts_undone() {
    let t = Task::new(1, String::from("Read chapter"));
    assert_eq!(t.id, 1);
    assert_eq!(t.title, String::from("Read chapter"));
    assert_eq!(t.done, false);
}

fn main() {}
```

## Code Explanation

- `struct Task` plus `new` mirrors `src/task.rs` so the test checks the real shape.
- `#[test]` marks the function as a test; `cargo test` runs it and treats panic as failure.
- `Task::new(1, String::from(...))` builds the value under test with owned text.
- `assert_eq!(t.id, 1)` and `assert_eq!(t.title, ...)` pin the constructor inputs so swaps fail loudly.
- `assert_eq!(t.done, false)` panics with both sides shown if `new` ever sets `done` to `true`.
- `fn main() {}` is an empty entry point so plain `rustc --edition 2021` accepts the file; `cargo test` still runs the `#[test]` function and prints `ok`.

## Common Mistakes

- **Testing against `tasks.db`**: writing to the real file pollutes your todos; use `:memory:` for store tests.
- **Only happy paths**: add a bad-id case (`parse_id("abc")` is `Err`) or regressions slip back.
- **Skipping `cargo test` before push**: parsing edits silently break store arms; run the suite each lesson.

## Best Practices

- Name tests after behavior (`new_task_starts_undone`), not after functions, so failures read clearly.
- Keep model tests pure (no files) and put each SQL path (`insert`, `list_all`, `mark_done`, `remove`) in its own store test.
- Run `cargo test` plus `cargo run -- --help` together: logic green and CLI bootable.

## Exercise

Without running code, predict: does `new_task_starts_undone` pass, and what would make it fail?

Then add this test plus one `:memory:` store test (`insert` then `list_all` has length 1) and run `cargo test`.

## Solution

It passes because `new` sets `done: false`; changing `new` to `done: true` would fail the last `assert_eq!`.

```rust
#[test]
fn new_task_starts_undone() {
    let t = Task::new(1, String::from("Read chapter"));
    assert_eq!(t.id, 1);
    assert_eq!(t.title, String::from("Read chapter"));
    assert_eq!(t.done, false);
}

fn main() {}
```

The test pins the constructor so later edits cannot flip the default silently. Capstone step: `cargo test` is green on model plus store; only final polish remains.

## Mini Quiz

```yaml
- question: "What does #[test] do?"
  options:
    - "Makes the function run on every CLI start"
    - "Marks the function for cargo test to run, with panic as failure"
    - "Creates a database table"
    - "Skips the function during testing"
  answer: 1
  explanation: "The test runner finds marked functions and reports panics as failures."
- question: "Why use :memory: for store tests?"
  options:
    - "Rows live only during the test, leaving real files untouched"
    - "Memory databases run without Rust"
    - "It makes SQL unneeded"
    - "It skips init_db"
  answer: 0
  explanation: "In-memory SQLite isolates tests from the user's tasks.db file."
- question: "What does assert_eq! check?"
  options:
    - "That code compiles faster"
    - "That a file exists"
    - "That two values are equal, panicking with both sides if not"
    - "That a command prints help"
  answer: 2
  explanation: "assert_eq compares expected vs real and shows both on mismatch."
```
