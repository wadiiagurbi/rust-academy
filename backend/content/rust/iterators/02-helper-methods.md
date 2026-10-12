---
slug: iterator-methods
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: iterators
chapterTitle: "Chapter 17 — Iterators"
chapterOrder: 17
title: "Iterator Helper Methods"
difficulty: beginner
order: 2
estimatedMinutes: 12
tags: [rust, iterators, map, filter]
---

## Learning Objective

By the end of this lesson, you will chain `map`, `filter`, and `sum`, and separate lazy adapters from eager consumers.

## Explanation

Helper methods on `Iterator` split in two. **Adapters** (`map`, `filter`, `take`) are *lazy*: each returns a new iterator transforming items on demand, configured by a **closure** (an inline anonymous function like `|x| x * 2`). Nothing runs until consumed. **Consumers** (`sum`, `count`, `collect`) are *eager*: they pull the chain and produce a final value, taking ownership so the chain cannot be reused. `map` transforms each item, `filter` keeps items where the closure returns `true`, and `sum` adds all items (annotate the target, e.g. `let total: i32`, so the compiler knows the numeric type).

## Why It Matters

Chains replace index loops with pipelines the compiler optimizes to the same machine code — fewer off-by-one mistakes, same speed.

## Example

Keep exam scores `>= 10`, double them as bonus, and total the result in one expression.

## Code

```rust
fn main() {
    let scores = [4, 7, 12, 9, 20];
    let total: i32 = scores.iter().filter(|&&s| s >= 10).map(|&s| s * 2).sum();
    println!("total: {total}");
}
```

## Code Explanation

- `iter()` borrows each score; `filter(|&&s| ...)` unwraps the double reference to compare.
- `map(|&s| s * 2)` doubles survivors (12→24, 20→40); `sum()` totals 64.
- Without a consumer, the adapters would do nothing at all.

## Common Mistakes

- **Lazy chain, no consumer**: `.map(...)` alone never runs its closure. End with `sum`, `collect`, or a loop.
- **Reusing a consumed chain**: consumers move the iterator; build a fresh `iter()` chain instead.
- **Wrong closure pattern**: `|s| s >= 10` on `&&i32` will not compile; destructure with `|&&s|`.

## Best Practices

- Read chains as pipelines: source, filters, transforms, one consumer.
- Annotate consumer types (`let total: i32`) instead of converting inside closures.

## Exercise

Compute the printed total by hand. Then insert `.take(1)` before `.sum()` — which score survives, and what is the new total?

## Solution

Original keeps 12 and 20, doubles to 24 and 40, prints `total: 64`. With `take(1)`:

```rust
fn main() {
    let scores = [4, 7, 12, 9, 20];
    let total: i32 = scores.iter().filter(|&&s| s >= 10).map(|&s| s * 2).take(1).sum();
    println!("total: {total}");
}
```

Only 24 survives, printing `total: 24`.

## Mini Quiz

```yaml
- question: "What is true of adapters like `map`?"
  options:
    - "They return a Vec immediately"
    - "They are lazy; work starts only when a consumer pulls"
    - "They run once per program"
    - "They need `mut` on the collection"
  answer: 1
  explanation: "Adapters return iterators; consumers drive the work."
- question: "What does `filter` do?"
  options:
    - "Replaces failing items"
    - "Keeps only items where the closure returns true"
    - "Sorts the items"
    - "Panics on failure"
  answer: 1
  explanation: "filter passes downstream only items satisfying the predicate."
- question: "Why annotate `.sum()` with `let total: i32`?"
  options:
    - "sum only works on i32"
    - "The compiler must know which type to accumulate into"
    - "It speeds up the code"
    - "Otherwise sum returns a string"
  answer: 1
  explanation: "sum is generic over the result type, so Rust needs a constraint."
```
