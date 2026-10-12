---
slug: cli-errors
track: final-project
trackTitle: "Final Project"
trackOrder: 2
chapter: final-project
chapterTitle: "Chapter 27 — Final Project: CLI Task Manager"
chapterOrder: 27
title: "Error Handling with anyhow"
difficulty: intermediate
order: 5
estimatedMinutes: 10
tags: [rust, cli, final-project, errors]
---

## Learning Objective

By the end of this lesson, you will return helpful errors with `anyhow` instead of crashing the CLI.

## Explanation

An **error** (a failure value like missing file or bad id) should print a clear message and exit cleanly. `anyhow` (a crate giving one flexible `Result` error type plus helpers) makes that short.

A **Result** (the enum `Ok(value)` or `Err(error)` showing success or failure) threads through our code: `store.rs` returns `anyhow::Result<Vec<Task>>` (either the task list or a failure). The `?` **operator** (placed after a `Result`, returns early with `Err` on failure) forwards database faults upward. `.context()` (an anyhow helper that adds a plain-words note to an error) says which step failed.

Project excerpt from `src/store.rs` (project file, verify with cargo build):

```rust
// Project file src/store.rs — needs anyhow in Cargo.toml; verify with `cargo build`.
use anyhow::{Context, Result};
impl Store { pub fn mark_done(&self, id: i64) -> Result<()> { let changed = self.conn.execute("UPDATE tasks SET status = 'done' WHERE id = ?1", params![id]).with_context(|| format!("failed to mark task {id} as done"))?; if changed == 0 { anyhow::bail!("no task found with id {id}"); } Ok(()) } }
```

`mark_done` matches the real app: same function name, same SQL, same context and bail messages.

## Why It Matters

`unwrap()` (a method that crashes on `Err`) turns a typo like `done abc` into a panic trace. `anyhow` turns it into `Error: no task found with id 99` plus the step that failed.

## Example

Typing `tasks done 99` for a missing row returns `no task found with id 99` with context `failed to mark task 99 as done`; `main` prints it and exits with a non-zero **status code** (a number the terminal reads: 0 means success).

## Code

```rust
fn parse_id(text: &str) -> Result<i64, String> {
    text.trim().parse::<i64>().map_err(|e| format!("invalid id {text:?}: {e}"))
}
fn main() {
    match parse_id("3") {
        Ok(id) => println!("parsed {id}"),
        Err(err) => println!("error: {err}"),
    }
    match parse_id("abc") {
        Ok(id) => println!("parsed {id}"),
        Err(err) => println!("error: {err}"),
    }
}
```

## Code Explanation

- `fn parse_id(text: &str) -> Result<i64, String>` returns either the number or a printable failure, like `anyhow::Result` does.
- `text.trim().parse::<i64>()` tries to convert text to `i64`; the `::<i64>` (called **turbofish**) picks the target type.
- `.map_err(|e| format!(...))` adds an `invalid id "abc"` note while keeping the cause, like `.context()` does.
- `match` in `main` prints `parsed 3` then `error: invalid id "abc"...` with no panic.

## Common Mistakes

- **Using `unwrap` in commands**: one bad id crashes test runs; return `Result` and let `main` print it.
- **Forgetting `Context` import**: `.context()` only works when the `Context` trait is in scope.
- **Swallowing causes**: writing only `eprintln!("failed")` hides which id or file failed; attach the value.

## Best Practices

- Return `anyhow::Result<T>` from every fallible helper (`add_task`, `list_tasks`, `mark_done`) for one error style.
- Add context at each layer: store says the SQL step with `with_context`, `main` prints the chain.
- Let `main` return `Result<()>` so `Err` prints via `Debug` and exits non-zero with no extra code.

## Exercise

Without running `rustc`, read the Code sample and predict: what does it print for `"3"` and `"abc"`?

Then add `anyhow = "1"` under `[dependencies]`, add `mark_done` with its context to your package, and run `cargo build`.

## Solution

`"3"` prints `parsed 3`; `"abc"` prints an `error:` line with `invalid id "abc"`.

```rust
fn parse_id(text: &str) -> Result<i64, String> {
    text.parse::<i64>().map_err(|e| format!("invalid id {text:?}: {e}"))
}
fn main() {
    println!("{:?}", parse_id("3"));
    println!("{:?}", parse_id("abc"));
}
```

`?` and `map_err` forward failures, so `main` prints instead of crashing. Capstone step: every store helper now returns `anyhow::Result`; the clap lesson can call them directly.

## Mini Quiz

```yaml
- question: "What does the ? operator do after a Result?"
  options:
    - "Always crashes the program"
    - "Ignores the error silently"
    - "Returns early with Err on failure, unwraps on success"
    - "Converts success into failure"
  answer: 2
  explanation: "? forwards errors to the caller, keeping command code short."
- question: "What does .context() add?"
  options:
    - "A new database table"
    - "A plain-words note about which step failed, keeping the cause"
    - "A CLI subcommand"
    - "Faster SQL queries"
  answer: 1
  explanation: "Context layers notes like invalid id over the low-level parse error."
- question: "Why prefer anyhow::Result over unwrap in the CLI?"
  options:
    - "Bad input prints a clear message instead of a panic trace"
    - "unwrap does not compile"
    - "anyhow makes code run faster"
    - "Result uses less disk space"
  answer: 0
  explanation: "Returning errors lets main print them and exit with a status code."
```
