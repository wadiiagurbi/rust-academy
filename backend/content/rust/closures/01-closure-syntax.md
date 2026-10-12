---
slug: closures
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: closures
chapterTitle: "Chapter 10 — Closures"
chapterOrder: 10
title: "Closure Syntax"
difficulty: beginner
order: 1
estimatedMinutes: 9
tags: [rust, closures, iterators]
---

## Learning Objective

By the end of this lesson, you will write `|args|` closures and pass them to iterator adapters.

## Explanation

A **closure** is an anonymous function value: no name, syntax `|params| expression` like `|x| x + 1`. **Anonymous** means unnamed and stored in variables or passed inline. An **iterator adapter** like `map` transforms each element with a closure. **Type annotations** (`|x: i32|`) are usually unneeded because the compiler infers them.

## Why It Matters

Closures turn loops into pipelines: `map` and `filter` state intent while types stay checked.

## Example

Doubling quiz scores and adding two numbers each take a one-line closure instead of a manual loop.

## Code

```rust
fn main() {
    let scores = vec![40, 55, 70];
    let doubled: Vec<i32> = scores.iter().map(|s| s * 2).collect();
    println!("{doubled:?}");
    let add = |a: i32, b: i32| a + b;
    println!("{}", add(2, 3));
}
```

## Code Explanation

- `|s| s * 2` doubles each borrowed score; `map` plus `collect()` builds the new vector.
- Output is `[80, 110, 140]` then `5`.

## Common Mistakes

- **Missing `collect` type**: the compiler cannot infer the container. Fix: annotate `Vec<i32>`.
- **Wrong iterator kind**: `iter()` yields `&i32`, `into_iter()` yields `i32`. Fix: match the closure body.

## Best Practices

- Keep closures to one expression; prefer pipelines over index loops for transforms.

## Exercise

Predict the output, then keep doubled values above `100` with `filter` and print them.

```rust
fn main() {
    let scores = vec![40, 55, 70];
    let doubled: Vec<i32> = scores.iter().map(|s| s * 2).collect();
    println!("{doubled:?}");
}
```

## Solution

It prints `[80, 110, 140]`. Filtering keeps two:

```rust
fn main() {
    let scores = vec![40, 55, 70];
    let big: Vec<i32> = scores.iter().map(|s| s * 2).filter(|s| *s > 100).collect();
    println!("{big:?}");
}
```

## Mini Quiz

```yaml
- question: "What is a closure?"
  options:
    - "An anonymous function value"
    - "A named module"
    - "A struct kind"
    - "A test harness"
  answer: 0
  explanation: "Closures use |args| syntax like values."
- question: "What does `map(|s| s * 2)` do?"
  options:
    - "Doubles each element"
    - "Sorts input"
    - "Drops elements"
    - "Counts them"
  answer: 0
  explanation: "`map` transforms each item with the closure."
- question: "Why call `collect()`?"
  options:
    - "Adapters are lazy; collect builds results"
    - "It sorts data"
    - "It spawns threads"
    - "It deletes input"
  answer: 0
  explanation: "Chains run only when consumed."
```
