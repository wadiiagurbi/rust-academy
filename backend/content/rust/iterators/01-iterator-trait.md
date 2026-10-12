---
slug: iterator-trait
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: iterators
chapterTitle: "Chapter 17 — Iterators"
chapterOrder: 17
title: "The Iterator Trait"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, iterators, traits]
---

## Learning Objective

By the end of this lesson, you will implement `Iterator` for your own type and consume it with `next()`.

## Explanation

An **iterator** is a value that hands out a sequence one item at a time. The **`Iterator` trait** (a shared interface any type can implement) defines it with one method, `next()`, plus an **associated type** `Item` (a placeholder each implementor fills in, like `type Item = u32;`). `next(&mut self)` returns an **`Option`**: `Some(item)` while items remain, `None` once exhausted (permanently). It takes `&mut self` (a mutable borrow) because yielding advances internal state. A `for` loop just calls `next()` until `None`.

## Why It Matters

Filtering, parsing, and streaming all build on this one trait. Learning it first shows there is no magic: every helper ahead is just code calling `next()`.

## Example

A counter yielding 1, 2, 3, then stopping — usable in `for` loops and every adapter for free.

## Code

```rust
struct Counter { current: u32, stop: u32 }

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current < self.stop {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

fn main() {
    for value in (Counter { current: 0, stop: 3 }) {
        println!("got {value}");
    }
}
```

## Code Explanation

- `Counter` is a tiny state machine: position (`current`) plus finish line (`stop`).
- `next` returns `Some` until `current == stop`, then `None`, ending the `for` loop. Output: `got 1`, `got 2`, `got 3`.

## Common Mistakes

- **Missing `mut`**: `next()` needs `&mut self`, so a non-`mut` binding fails with `cannot borrow as mutable`.
- **Reusing after `None`**: exhaustion is permanent; the sequence never restarts.
- **Wrong borrow kind**: `iter()` borrows items (`&T`), `into_iter()` moves them (`T`).

## Best Practices

- Implement `Iterator` for custom sequences instead of exposing index access.

## Exercise

Predict the exact output above, then make the counter yield 5, 6, 7 (start at 4, stop at 7).

## Solution

Original prints `got 1` through `got 3`. Adjusted version:

```rust
struct Counter { current: u32, stop: u32 }

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current < self.stop {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

fn main() {
    for value in (Counter { current: 4, stop: 7 }) {
        println!("got {value}");
    }
}
```

## Mini Quiz

```yaml
- question: "What does `next()` return once exhausted?"
  options:
    - "`None`, permanently"
    - "It panics"
    - "An empty iterator"
    - "`Some` with a default"
  answer: 0
  explanation: "Exhaustion is `None`, and further calls stay `None`."
- question: "What is the associated type `Item`?"
  options:
    - "The error type"
    - "The type of each yielded value"
    - "The items remaining"
    - "The source collection"
  answer: 1
  explanation: "Each implementor sets `type Item` to declare what `next()` produces."
- question: "Why does `next()` take `&mut self`?"
  options:
    - "To prevent reuse"
    - "Advancing the position mutates iterator state"
    - "To move out each item"
    - "It takes `&self`"
  answer: 1
  explanation: "Yielding updates the cursor, which needs a mutable borrow."
```
