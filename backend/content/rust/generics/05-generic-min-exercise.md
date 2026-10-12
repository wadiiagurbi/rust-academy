---
slug: generic-min
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: generics
chapterTitle: "Chapter 9 — Generics"
chapterOrder: 9
title: "Generic Min Exercise"
difficulty: beginner
order: 5
estimatedMinutes: 18
tags: [rust, generics, exercise, testing]
---

## Learning Objective

By the end of this lesson, you will write a bounded generic `minimum` and prove it with `cargo test`.

## Explanation

A **slice** (`&[T]`) borrows consecutive elements. **`PartialOrd`** provides `<` comparison; **`Clone`** makes explicit duplicates. Returning `Option<T>` models both outcomes: `Some(min)` or `None` for the **empty slice** (zero elements). Bounds `T: PartialOrd + Clone` accept every comparable, clonable type.

## Why It Matters

One `minimum` serves scores, prices, and names without rewrites or runtime checks.

## Example

A classroom finds the lowest score; the same call finds the first name alphabetically.

## Code

```rust
fn minimum<T: PartialOrd + Clone>(items: &[T]) -> Option<T> {
    let mut best = items.first()?.clone();
    for item in &items[1..] {
        if *item < best { best = item.clone(); }
    }
    Some(best)
}

fn main() {
    println!("{:?}", minimum(&[8, 3, 5]));
    println!("{:?}", minimum(&["pear", "apple"]));
}
```

## Code Explanation

- `first()?` exits with `None` on empty input; otherwise clones the leader.
- The loop clones smaller values in; output is `Some(3)`, `Some("apple")`.

## Common Mistakes

- **Indexing empties**: `items[0]` panics. Fix: start from `first()?`.
- **Using `Ord` for floats**: `f64` lacks it. Fix: bound on `PartialOrd`.

## Best Practices

- Return `Option<T>` for partial ops; test empty, single, numeric, and string inputs.

## Exercise

Create a cargo project `min_task` with `cargo new min_task`. Implement `minimum` for the criteria.

```rust
// src/main.rs starter — replace todo!() with your code:
fn minimum<T: PartialOrd + Clone>(items: &[T]) -> Option<T> { todo!() }

fn main() { todo!() }
```

Your `cargo test` criteria: `Some(3)` for `[8, 3, 5]`, `None` for empty, `Some("apple")` for `["pear", "apple"]`.

## Solution

```rust
fn minimum<T: PartialOrd + Clone>(items: &[T]) -> Option<T> {
    let mut best = items.first()?.clone();
    for item in &items[1..] {
        if *item < best { best = item.clone(); }
    }
    Some(best)
}

fn main() { println!("{:?}", minimum(&[8, 3, 5])); }

#[test]
fn finds_numbers_and_strings() {
    assert_eq!(minimum(&[8, 3, 5]), Some(3));
    assert_eq!(minimum(&["pear", "apple"]), Some("apple"));
}

#[test]
fn empty_gives_none() { assert_eq!(minimum::<i32>(&[]), None); }
```

## Mini Quiz

```yaml
- question: "Why return `Option<T>`?"
  options:
    - "Empty input has no minimum (None)"
    - "Options run faster"
    - "Slices need Option"
    - "To skip generics"
  answer: 0
  explanation: "`None` covers emptiness without panicking."
- question: "What does `PartialOrd + Clone` require?"
  options:
    - "`<` comparison plus cloning"
    - "Integer type"
    - "String type"
    - "Copy"
  answer: 0
  explanation: "Compare, then return owned values."
- question: "What does `?` do here?"
  options:
    - "Returns None early on empty input"
    - "Panics"
    - "Skips first"
    - "Clones slice"
  answer: 0
  explanation: "`?` propagates None from Option functions."
```
