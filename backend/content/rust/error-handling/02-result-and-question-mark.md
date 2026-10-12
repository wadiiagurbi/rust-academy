---
slug: question-mark
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: error-handling
chapterTitle: "Chapter 20 — Error Handling"
chapterOrder: 20
title: "Result and the Question Mark"
difficulty: beginner
order: 2
estimatedMinutes: 12
tags: [rust, errors, result]
---

## Learning Objective

By the end of this lesson, you will propagate errors with `Result` and `?`, handling the outcome with `match`.

## Explanation

**`Result<T, E>`** is the enum for fallible operations: `Ok(value)` success, `Err(error)` failure. The **`?` operator** means "return early on error": in a `Result`-returning function, `fs::read_to_string(path)?` yields the `String` on success or returns the error to the caller at once. Hence `?` needs a `Result`-returning function — the classic beginner compile error. At the boundary (often `main`), **`match`** inspects the outcome: greet on `Ok`, report on `Err` with `eprintln!` (like `println!` but to standard error, the diagnostics stream).

## Why It Matters

`?` straightens error plumbing into a top-to-bottom happy path; every failure takes the same early exit — no exceptions, no hidden jumps.

## Example

Read a username from a file, trim it, greet — I/O failure reaches `main`, which reports instead of crashing.

## Code

```rust
use std::fs;

fn read_username(path: &str) -> Result<String, std::io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(text.trim().to_string())
}

fn main() {
    match read_username("name.txt") {
        Ok(name) => println!("hello, {name}"),
        Err(err) => eprintln!("could not read file: {err}"),
    }
}
```

## Code Explanation

- `?` returns the I/O error immediately on failure — no mid-function `match` needed.
- `trim()` strips editor newlines; `to_string()` builds the owned `Ok` value.

## Common Mistakes

- **`?` in `()->()` main**: fails with `cannot use ?`. Fix: `match` here, or `fn main() -> Result<(), std::io::Error>`.
- **Swallowing `err`**: a bare `_ => {}` arm hides the reason. Bind and print it.
- **Dodging with `unwrap`**: it panics on missing files. Save `unwrap` for tests; propagate with `?`.

## Best Practices

- Bubble with `?` through helpers; handle once at the boundary.
- Print diagnostics with `eprintln!` so `stdout` stays pipeable.

## Exercise

Create `name.txt` with `  Ada  `, run it. Delete the file, run again — which arm fires, and what does `{err}` show?

## Solution

```rust
use std::fs;

fn read_username(path: &str) -> Result<String, std::io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(text.trim().to_string())
}

fn main() -> Result<(), std::io::Error> {
    let name = read_username("name.txt")?;
    println!("hello, {name}");
    Ok(())
}
```

Trimming prints `hello, Ada`. This variant shows the second boundary style: `main` returns `Result`, so `?` works directly and a missing file prints the default `Error: ...` line.

## Mini Quiz

```yaml
- question: "What does `?` do on `Err`?"
  options:
    - "Panics"
    - "Returns the error from the function at once"
    - "Retries"
    - "Converts to `None`"
  answer: 1
  explanation: "`?` unwraps `Ok`, early-returns `Err` — needs a `Result` function."
- question: "Why no `?` in `fn main()` returning `()`?"
  options:
    - "main never fails"
    - "No `Result` exists to carry the error out"
    - "Banned by the borrow checker"
    - "No restriction exists"
  answer: 1
  explanation: "Early return needs a compatible signature; `match` or return `Result`."
- question: "Why `eprintln!` for errors?"
  options:
    - "It prints red"
    - "Diagnostics go to stderr, keeping stdout pipeable"
    - "It is faster"
    - "println! cannot print errors"
  answer: 1
  explanation: "Separate streams let users redirect output and diagnostics independently."
```
