---
slug: generic-functions
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: generics
chapterTitle: "Chapter 9 — Generics"
chapterOrder: 9
title: "Generic Functions"
difficulty: beginner
order: 1
estimatedMinutes: 9
tags: [rust, generics, functions]
---

## Learning Objective

By the end of this lesson, you will write a generic function working for many types via a type parameter.

## Explanation

A **generic function** uses a **type parameter** like `T` in `fn first<T>(...)`, a placeholder replaced by each **concrete type** (`i32`, `String`) at calls. **Monomorphization** generates a specialized copy per concrete type at compile time: one body, fast code, no duplication.

## Why It Matters

One tested helper serves numbers, names, and structs while the compiler specializes each call site for free.

## Example

A roster tool takes the first of numbers or names with a single generic `first` instead of three copies.

## Code

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

fn main() {
    println!("{:?}", first(&[10, 20, 30]));
    println!("{:?}", first(&["ana", "ben"]));
}
```

## Code Explanation

- `&[T]` borrows a slice of any element type; `Option<&T>` is `Some` borrow or `None` when empty.
- Calls set `T` to `i32` then `&str`; output is `Some(10)`, `Some("ana")`.

## Common Mistakes

- **Mixing types per call**: `[1, "hi"]` has no single `T`. Fix: keep calls homogeneous.
- **Returning `T` from a slice**: moving out is forbidden. Fix: return `&T` borrows.

## Best Practices

- Take slices (`&[T]`) over `Vec<T>`; return `Option<&T>` for possibly empty lookups.

## Exercise

Predict the output, then print `first` of an empty `[i32; 0]` and explain the result.

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

fn main() {
    println!("{:?}", first(&[10, 20, 30]));
}
```

## Solution

It prints `Some(10)`. Empty input has nothing to borrow:

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

fn main() {
    let empty: [i32; 0] = [];
    println!("{:?}", first(&empty));
}
```

This prints `None`.

## Mini Quiz

```yaml
- question: "What is a type parameter `T`?"
  options:
    - "A placeholder filled per call"
    - "An integer variable"
    - "A module name"
    - "A test flag"
  answer: 0
  explanation: "Each call picks the concrete type."
- question: "What is monomorphization?"
  options:
    - "A specialized copy per type at compile time"
    - "Merging types dynamically"
    - "Running twice"
    - "Formatting"
  answer: 0
  explanation: "Each `T` gets its own fast copy."
- question: "Why return `Option<&T>`?"
  options:
    - "Slices may be empty, so None is needed"
    - "References need Option"
    - "It is faster"
    - "Slices are never empty"
  answer: 0
  explanation: "`None` models the empty case safely."
```
