---
slug: collect-trait
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: iterators
chapterTitle: "Chapter 17 — Iterators"
chapterOrder: 17
title: "Collect and IntoIterator"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, iterators, collect, collections]
---

## Learning Objective

By the end of this lesson, you will build collections with `collect()` and loop over borrows via `IntoIterator`.

## Explanation

`collect()` is the consumer that builds a collection from an iterator. It is **generic**: any type implementing **`FromIterator`** (the build-from-items interface: `Vec`, `String`, and more) can be the target, so annotate the binding (`let shout: Vec<String>`). The **`IntoIterator`** trait (a conversion into an iterator) powers `for` loops: `for x in collection` calls `into_iter()` on the value, the shared reference (`&collection`, borrows), or the exclusive reference (`&mut collection`, mutably borrows). So `for word in &shout` borrows and `shout` stays usable, while `for word in shout` moves it.

## Why It Matters

`collect()` bridges the iterator world and the collection world; `IntoIterator` makes `for` uniform over arrays, vectors, and maps.

## Example

Uppercase colour names into a vector, print each by borrow, then reuse the vector.

## Code

```rust
fn main() {
    let words = ["red", "green", "blue"];
    let shout: Vec<String> = words.iter().map(|w| w.to_uppercase()).collect();
    println!("{shout:?}");
    for word in &shout {
        println!("{word}");
    }
    println!("still have {} colours", shout.len());
}
```

## Code Explanation

- `map` uppercases each borrowed `&str` into an owned `String`; `collect()` gathers them.
- `{shout:?}` is **Debug formatting** (programmer-facing inspection output).
- Looping over `&shout` borrows, proven by the final `shout.len()`.

## Common Mistakes

- **Missing annotation**: `collect()` without a target fails with `type annotations needed`. Write `Vec<_>` to infer the element type.
- **Using a moved collection**: `for word in shout` moves it; later uses fail with `borrow of moved value`. Loop over `&shout` instead.
- **Collecting borrows by accident**: convert to owned data (`to_uppercase`, `to_string`) when the result must outlive the source.

## Best Practices

- Annotate `collect()` targets; default to looping over references.
- Consume (`for x in v`) only when you intend to destroy the collection.

## Exercise

Append `"!"` to each colour (`"RED!"`). Then consume with `for word in shout` — which old line stops compiling, and why?

## Solution

```rust
fn main() {
    let words = ["red", "green", "blue"];
    let shout: Vec<String> = words.iter().map(|w| format!("{}!", w.to_uppercase())).collect();
    for word in shout {
        println!("{word}");
    }
}
```

`format!` builds each owned string. Consuming `shout` moves it, so the old `shout.len()` line fails with `borrow of moved value`.

## Mini Quiz

```yaml
- question: "Why annotate `collect()`?"
  options:
    - "It is generic: many collection types could be built"
    - "collect only makes vectors"
    - "Annotations pick the items"
    - "Otherwise it returns an iterator"
  answer: 0
  explanation: "Any `FromIterator` type is valid, so Rust needs help choosing."
- question: "`for word in &shout` vs `for word in shout`?"
  options:
    - "The first is faster"
    - "Borrowing keeps `shout` usable; moving ends it"
    - "The first reverses order"
    - "No difference"
  answer: 1
  explanation: "`IntoIterator` on references yields borrows instead of moving."
- question: "What is `{shout:?}`?"
  options:
    - "JSON output"
    - "Debug formatting for inspection"
    - "A macro call"
    - "Per-element interpolation"
  answer: 1
  explanation: "`:?` uses the `Debug` trait, handy while developing."
```
