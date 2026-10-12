---
slug: result-intro
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: std-types
chapterTitle: "Chapter 11 — Standard Library Types"
chapterOrder: 11
title: "Result Intro"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, result, errors]
---

## Learning Objective

By the end of this lesson, you will return `Ok`/`Err` and propagate failures with `?`.

## Explanation

**`Result<T, E>`** models fallible work: `Ok(value)` success, `Err(error)` failure details. The **`?` operator** unwraps `Ok` or returns `Err` from the whole function early. Functions using `?` must themselves return `Result` (or `Option`) to receive the early return.

## Why It Matters

Errors travel upward with context instead of crashing, and chained `?` calls read top to bottom.

## Example

Registration validates age, chaining checks so the first failure wins.

## Code

```rust
fn check_age(age: u32) -> Result<u32, String> {
    if age >= 13 { Ok(age) } else { Err(String::from("too young")) }
}

fn register(age: u32) -> Result<String, String> {
    let valid = check_age(age)?;
    Ok(format!("registered {valid}"))
}

fn main() {
    println!("{:?}", register(20));
    println!("{:?}", register(9));
}
```

## Code Explanation

- `?` unwraps valid ages or returns the error at once.
- Output is `Ok("registered 20")` then `Err("too young")`.

## Common Mistakes

- **`?` in void `main`**: needs a `Result` return. Fix: return `Result<(), E>` or match instead.
- **Unwrapping user input**: panics on bad data. Fix: propagate with `?`.

## Best Practices

- Return `Result` for fallible work; use `?` instead of nested matches.

## Exercise

Predict both lines, then add `check_name` rejecting empty names and chain it with `?`.

```rust
fn check_age(age: u32) -> Result<u32, String> {
    if age >= 13 { Ok(age) } else { Err(String::from("too young")) }
}

fn register(age: u32) -> Result<String, String> { Ok(format!("registered {}", check_age(age)?)) }

fn main() {
    println!("{:?}", register(20));
}
```

## Solution

It prints the `Ok` line. Chaining two checks:

```rust
fn check_age(age: u32) -> Result<u32, String> {
    if age >= 13 { Ok(age) } else { Err(String::from("too young")) }
}

fn check_name(name: &str) -> Result<String, String> {
    if name.is_empty() { Err(String::from("name required")) } else { Ok(name.to_string()) }
}

fn register(age: u32, name: &str) -> Result<String, String> {
    Ok(format!("registered {} ({})", check_name(name)?, check_age(age)?))
}

fn main() { println!("{:?} {:?}", register(20, "ana"), register(20, "")); }
```

## Mini Quiz

```yaml
- question: "What are Result's variants?"
  options:
    - "Ok(value) and Err(error)"
    - "Some and None"
    - "True and False"
    - "Yes and No"
  answer: 0
  explanation: "Ok succeeds; Err carries details."
- question: "What does `?` do on Err?"
  options:
    - "Returns it from the function at once"
    - "Panics always"
    - "Ignores it"
    - "Retries"
  answer: 0
  explanation: "Failures propagate upward immediately."
- question: "What must `?` users return?"
  options:
    - "Result or Option"
    - "Unit always"
    - "String always"
    - "Anything"
  answer: 0
  explanation: "The early return needs a carrier."
```
