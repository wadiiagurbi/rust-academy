---
slug: trait-bounds
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: generics
chapterTitle: "Chapter 9 — Generics"
chapterOrder: 9
title: "Trait Bounds"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, generics, traits, bounds]
---

## Learning Objective

By the end of this lesson, you will restrict a type parameter with a trait bound.

## Explanation

A **trait bound** (`T: Display`) requires `T` to implement the trait. **`Display`** is the standard trait for user-facing `{}` text. Without bounds, generic `T` supports almost no operations; the bound unlocks the trait's methods, checked at compile time per call.

## Why It Matters

Bounds turn vague generics into contracts: printable-only input, rejected otherwise before running.

## Example

A banner wraps any displayable value in stars; the bound guarantees `{}` formatting works inside.

## Code

```rust
use std::fmt::Display;

fn banner<T: Display>(value: T) -> String {
    format!("*** {value} ***")
}

fn main() {
    println!("{}", banner(42));
    println!("{}", banner("launch"));
}
```

## Code Explanation

- Only `Display` types may call `banner`; `i32` and `&str` both qualify.
- Output is `*** 42 ***` and `*** launch ***`.

## Common Mistakes

- **Passing a non-Display type**: errors with `trait bound not satisfied`. Fix: implement `Display` first.
- **Missing the import**: `Display` must be in scope. Fix: add the `use` line.

## Best Practices

- Bound on exactly what the body uses; move long bounds to a `where` clause.

## Exercise

Predict the output, then decide: does `banner(vec![1, 2])` compile? Name the missing trait.

```rust
use std::fmt::Display;

fn banner<T: Display>(value: T) -> String {
    format!("*** {value} ***")
}

fn main() {
    println!("{}", banner(42));
}
```

## Solution

It prints `*** 42 ***`. Vectors fail the bound:

```rust
use std::fmt::Display;

fn banner<T: Display>(value: T) -> String {
    format!("*** {value} ***")
}

fn main() {
    println!("{}", banner(42));
    println!("{:?}", vec![1, 2]);
}
```

`Vec<i32>` has `Debug` but not `Display`, so the bound rejects it.

## Mini Quiz

```yaml
- question: "What does `T: Display` mean?"
  options:
    - "T must implement Display"
    - "T is exactly String"
    - "T is an integer"
    - "T is optional"
  answer: 0
  explanation: "Only Display types may call."
- question: "When are bound violations reported?"
  options:
    - "At compile time"
    - "In release only"
    - "At shutdown"
    - "Never"
  answer: 0
  explanation: "Each call site is checked during compilation."
- question: "Why do unbounded generics do little?"
  options:
    - "Almost no method is known to exist on T"
    - "They are slow"
    - "They need unsafe"
    - "They fit integers only"
  answer: 0
  explanation: "Bounds add knowledge; without them little applies."
```
