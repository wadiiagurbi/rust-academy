---
slug: option-type
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: std-types
chapterTitle: "Chapter 11 — Standard Library Types"
chapterOrder: 11
title: "Option Type"
difficulty: beginner
order: 1
estimatedMinutes: 9
tags: [rust, option, std]
---

## Learning Objective

By the end of this lesson, you will handle `Some` and `None` with `unwrap_or` and `map`.

## Explanation

**`Option<T>`** models presence or absence: `Some(value)` holds one, `None` holds nothing, replacing null. **`unwrap_or`** substitutes a fallback for `None`. **`map`** transforms the inner value when present and passes `None` through. The missing case is always visible to the compiler.

## Why It Matters

Lookups and configs are often absent; `Option` forces the empty path into the type instead of a crash.

## Example

A nickname lookup falls back to Guest and uppercases only when a name exists.

## Code

```rust
fn nickname(user: &str) -> Option<String> {
    if user == "ana" { Some(String::from("Ace")) } else { None }
}

fn main() {
    println!("{}", nickname("ana").unwrap_or(String::from("Guest")));
    println!("{:?}", nickname("ana").map(|n| n.to_uppercase()));
    println!("{:?}", nickname("zoe").map(|n| n.to_uppercase()));
}
```

## Code Explanation

- `unwrap_or` prints `Ace`; `map` with `to_uppercase` prints `Some("ACE")`.
- Unknown user maps to `None` without running the closure.

## Common Mistakes

- **Blind `unwrap`**: panics on `None`. Fix: prefer `unwrap_or` or `match`.
- **Missing `None` arm**: `match` without it fails exhaustiveness. Fix: always include `None`.

## Best Practices

- Default to `unwrap_or` and `map`; reserve message-bearing `expect` for impossible `None`.

## Exercise

Predict the first line, then rewrite it with a `match` printing `"Ace"` or `"Guest"`.

```rust
fn nickname(user: &str) -> Option<String> {
    if user == "ana" { Some(String::from("Ace")) } else { None }
}

fn main() {
    println!("{}", nickname("ana").unwrap_or(String::from("Guest")));
}
```

## Solution

It prints `Ace`. The explicit version:

```rust
fn nickname(user: &str) -> Option<String> {
    if user == "ana" { Some(String::from("Ace")) } else { None }
}

fn main() {
    let name = match nickname("ana") {
        Some(nick) => nick,
        None => String::from("Guest"),
    };
    println!("{name}");
}
```

## Mini Quiz

```yaml
- question: "What are Option's variants?"
  options:
    - "Some(value) and None"
    - "Ok and Err"
    - "True and False"
    - "Start and End"
  answer: 0
  explanation: "Some carries; None marks absence."
- question: "What does `unwrap_or` do?"
  options:
    - "Value or fallback on None"
    - "Always panics"
    - "Deletes input"
    - "Makes a vector"
  answer: 0
  explanation: "Safe default instead of crashing."
- question: "What does `map` do on None?"
  options:
    - "Stays None, skipping the closure"
    - "Panics"
    - "Returns empty Some"
    - "Calls with zero"
  answer: 0
  explanation: "Only Some runs the closure."
```
