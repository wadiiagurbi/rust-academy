---
slug: error-conversions
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: error-handling
chapterTitle: "Chapter 20 — Error Handling"
chapterOrder: 20
title: "Custom Errors and Conversions"
difficulty: intermediate
order: 3
estimatedMinutes: 12
tags: [rust, errors, from, conversion]
---

## Learning Objective

By the end of this lesson, you will unify mixed failures under a custom error enum using `From` and `?`.

## Explanation

Libraries define a **custom error enum**: one variant per failure (`Parse(ParseIntError)` for bad numbers, `Empty` for missing input). The **`From` trait** (standard conversion interface: `From<X> for E` builds `E` from `X`) teaches `?` to translate automatically — with `impl From<ParseIntError> for CalcError`, `?` after `.parse()` converts failures without a manual `match`. **`ok_or`** (turns an `Option`'s `None` into your `Err`) bridges `Option` into `Result`: `.next().ok_or(CalcError::Empty)?` treats "no first field" as an error. One function, one error type, `?` everywhere.

## Why It Matters

Callers match meaningful variants instead of strings, and every call site stays a one-liner however many failures hide beneath.

## Example

Double the first of two comma-separated numbers — unifying "missing" and "not a number" under `CalcError`.

## Code

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum CalcError { Parse(ParseIntError), Empty }

impl From<ParseIntError> for CalcError {
    fn from(err: ParseIntError) -> Self { CalcError::Parse(err) }
}

fn double_first(input: &str) -> Result<i32, CalcError> {
    let first = input.split(',').next().ok_or(CalcError::Empty)?;
    let value: i32 = first.trim().parse()?;
    Ok(value * 2)
}

fn main() {
    match double_first("21, x") { Ok(n) => println!("{n}"), Err(err) => println!("error: {err:?}") }
}
```

## Code Explanation

- `#[derive(Debug)]` generates the `{:?}` formatting used in the error arm. `From` lets `?` auto-convert parse errors; `ok_or(?)` lifts the missing field. `"21, x"` prints `42`.

## Common Mistakes

- **Missing `From`**: `?` on `.parse()` then fails with `From<ParseIntError> is not implemented`. The impl wires mixed errors together.
- **`unwrap` in libraries**: panics on caller input. Convert (`ok_or`, `?`) so callers decide.
- **String errors**: `Result<i32, String>` forces text matching. Enums stay machine-readable.

## Best Practices

- One variant per cause; wrap source errors instead of discarding them. Derive `Debug` minimum; add `Display` + `std::error::Error` for public APIs.

## Exercise

Add a `Negative` variant rejecting negative values. What does `double_first("-5, x")` print?

## Solution

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum CalcError { Parse(ParseIntError), Empty, Negative }

impl From<ParseIntError> for CalcError {
    fn from(err: ParseIntError) -> Self { CalcError::Parse(err) }
}

fn double_first(input: &str) -> Result<i32, CalcError> {
    let first = input.split(',').next().ok_or(CalcError::Empty)?;
    let value: i32 = first.trim().parse()?;
    if value < 0 {
        return Err(CalcError::Negative);
    }
    Ok(value * 2)
}

fn main() {
    match double_first("-5, x") { Ok(n) => println!("{n}"), Err(err) => println!("error: {err:?}") }
}
```

Prints `error: Negative` — distinct and matchable.

## Mini Quiz

```yaml
- question: "What does the `From` impl enable?"
  options:
    - "Faster parsing"
    - "`?` auto-converting parse failures into `CalcError`"
    - "Parsing without Result"
    - "Ignoring errors"
  answer: 1
  explanation: "`?` translates via `From` into the declared error type."
- question: "`.ok_or(CalcError::Empty)?`?"
  options:
    - "Panics on None"
    - "`Some(v)` to `Ok(v)`, `None` to `Err`, then propagates"
    - "Converts back to Option"
    - "Filters empties"
  answer: 1
  explanation: "`ok_or` bridges Option into Result; `?` early-returns it."
- question: "Enum over `Result<T, String>`?"
  options:
    - "Faster compile"
    - "Callers match variants instead of parsing text"
    - "Strings cannot err"
    - "No advantage"
  answer: 1
  explanation: "Variants are machine-readable kinds."
```
