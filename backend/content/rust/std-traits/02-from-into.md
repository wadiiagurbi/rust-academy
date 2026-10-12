---
slug: from-into
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: std-traits
chapterTitle: "Chapter 12 — Standard Library Traits"
chapterOrder: 12
title: "From and Into"
difficulty: beginner
order: 2
estimatedMinutes: 9
tags: [rust, traits, from, conversions]
---

## Learning Objective

By the end of this lesson, you will convert with `From`/`into` and parse strings with `parse`.

## Explanation

**`From<T>`** is the standard infallible conversion: `From<T> for U` teaches `U::from` plus reverse `.into()`. **`into()`** infers its target from context. **`parse()`** converts text into an annotated type, returning `Result` because input may be invalid. `Into` comes free with every `From`.

## Why It Matters

Conversions glue APIs: raw numbers become ids, CLI text becomes typed config, through one idiom.

## Example

A user id wraps `u32`; a port parses from text with a fallback default.

## Code

```rust
#[derive(Debug, PartialEq)]
struct UserId(u32);

impl From<u32> for UserId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

fn main() {
    let id: UserId = 7u32.into();
    println!("{id:?}");
    println!("{}", "8080".parse::<u16>().unwrap_or(80));
}
```

## Code Explanation

- Target types come from annotations (`UserId`, `u16`); bad parses fall back to `80`.
- Output is `UserId(7)` then `8080`.

## Common Mistakes

- **Missing annotation**: target ambiguous. Fix: annotate the binding.
- **Fallible `From`**: `From` cannot fail. Fix: use `TryFrom` or `parse` for validation.

## Best Practices

- Implement `From` for clean wrappings; parse once at the boundary, carry typed values inward.

## Exercise

Predict the output, then parse `"not-a-port"` with `unwrap_or(80)` and state the result.

```rust
#[derive(Debug, PartialEq)]
struct UserId(u32);

impl From<u32> for UserId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

fn main() {
    let id: UserId = 7u32.into();
    println!("{id:?}");
}
```

## Solution

It prints `UserId(7)`. Invalid text falls back:

```rust
fn main() {
    println!("{}", "not-a-port".parse::<u16>().unwrap_or(80));
}
```

This prints `80` because parsing fails.

## Mini Quiz

```yaml
- question: "What comes with `From<T> for U`?"
  options:
    - "Both From and reverse Into"
    - "Only Debug"
    - "A module"
    - "Nothing"
  answer: 0
  explanation: "Into is automatic from each From."
- question: "Why does `parse` return Result?"
  options:
    - "Text may be invalid"
    - "Parsing is slow"
    - "Strings valid always"
    - "Results print nicer"
  answer: 0
  explanation: "Bad input becomes Err, not a crash."
- question: "When use TryFrom over From?"
  options:
    - "Conversion itself can fail"
    - "Always"
    - "Never"
    - "Strings only"
  answer: 0
  explanation: "From is infallible by contract."
```
