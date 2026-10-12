---
slug: result-rewrite
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: error-handling
chapterTitle: "Chapter 20 — Error Handling"
chapterOrder: 20
title: "Exercise: Rewriting with Result"
difficulty: beginner
order: 5
estimatedMinutes: 15
tags: [rust, errors, exercise]
---

## Learning Objective

By the end of this lesson, you will convert a panicking parser into `Result` code with `?`, verified by `cargo test`.

## Explanation

This is a **cargo task**: `cargo new parse_lab`, paste the stub into `src/main.rs`, replace `todo!()` with a typed `Result` pipeline. The stub parses `"num:..."` lines; each `#[test]` is one criterion. `ParseError` needs **`PartialEq`** (the `==` trait — derive it) so tests compare `Result` values directly with `assert_eq!`. `From<ParseIntError>` is pre-wired so `?` converts digit failures automatically.

## Why It Matters

Panics dictate crashes; `Result`s offer choices the caller — and your tests — can exercise.

## Example

```text
test doubles_valid_line ... ok
test rejects_missing_prefix ... ok
```

## Code

```rust
#[derive(Debug, PartialEq)]
enum ParseError { MissingPrefix, BadNumber(std::num::ParseIntError) }

impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self { ParseError::BadNumber(err) }
}

fn parse_and_double(line: &str) -> Result<i32, ParseError> { todo!() }

fn main() {
    match parse_and_double("num:21") { Ok(n) => println!("{n}"), Err(err) => eprintln!("error: {err:?}") }
}

#[test]
fn doubles_valid_line() { assert_eq!(parse_and_double("num:21"), Ok(42)); }

#[test]
fn rejects_missing_prefix() { assert_eq!(parse_and_double("21"), Err(ParseError::MissingPrefix)); }
```

## Code Explanation

- `todo!()` compiles but fails both tests; `main` already handles either outcome (`42` once implemented).

## Common Mistakes

- **Slicing the prefix**: `&line[4..]` panics on short input. Use `line.strip_prefix("num:")` (returns `Option`) with `ok_or(...)?`.
- **Hand-matching parse**: the given `From` makes one `?` enough — no manual conversion.

## Best Practices

- Convert `Option` to `Result` at the boundary with `ok_or`, then `?` all the way down. Derive `PartialEq` on error types you assert on.

## Exercise

Implement `parse_and_double` with `strip_prefix` + `ok_or` + `?`. Add a third test: `parse_and_double("num:4x")` is `Err` (use `.is_err()`).

## Solution

```rust
#[derive(Debug, PartialEq)]
enum ParseError { MissingPrefix, BadNumber(std::num::ParseIntError) }

impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self { ParseError::BadNumber(err) }
}

fn parse_and_double(line: &str) -> Result<i32, ParseError> {
    let digits = line.strip_prefix("num:").ok_or(ParseError::MissingPrefix)?;
    Ok(digits.trim().parse::<i32>()? * 2)
}

fn main() {
    match parse_and_double("num:21") { Ok(n) => println!("{n}"), Err(err) => eprintln!("error: {err:?}") }
}

#[test]
fn rejects_bad_digits() { assert!(parse_and_double("num:4x").is_err()); }
```

Stub tests carry over unchanged; the new `rejects_bad_digits` plus `cargo run` printing `42` confirm the fix.

## Mini Quiz

```yaml
- question: "`line.strip_prefix(\"num:\")` returns?"
  options:
    - "Uppercase line"
    - "`Some(rest)` if present, `None` otherwise"
    - "A boolean"
    - "It panics on mismatch"
  answer: 1
  explanation: "Safe `Option` instead of panicking slicing."
- question: "Why derive `PartialEq`?"
  options:
    - "Required for enums"
    - "Tests compare `Result`s directly with `assert_eq!`"
    - "Pretty printing"
    - "Enables `?`"
  answer: 1
  explanation: "`assert_eq!` needs `==`, including across variants."
- question: "Core benefit over panicking?"
  options:
    - "Faster"
    - "Callers handle each failure instead of crashing"
    - "Less memory"
    - "Panics removed"
  answer: 1
  explanation: "The retry/default/abort decision moves to the caller."
```
