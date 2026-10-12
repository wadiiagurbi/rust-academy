---
slug: counter-exercise
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: std-types
chapterTitle: "Chapter 11 — Standard Library Types"
chapterOrder: 11
title: "Counter Exercise"
difficulty: beginner
order: 5
estimatedMinutes: 18
tags: [rust, hashmap, exercise, testing]
---

## Learning Objective

By the end of this lesson, you will count words with a `HashMap` and test the counts.

## Explanation

**Tokenizing** splits text into words. **`split_whitespace`** yields borrowed `&str` views over spaces. A **frequency map** (`HashMap<String, u32>`) records occurrences per owned word. The `entry` API increments each word in one lookup.

## Why It Matters

Word counts power search and analytics, combining strings, slices, options, and maps in one pipeline.

## Example

`"red fish red fish blue"` counts to red 2, fish 2, blue 1 via one folding loop.

## Code

```rust
use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

fn main() { let c = count_words("red fish red fish blue"); println!("{} {}", c.get("red").unwrap_or(&0), c.len()); }
```

## Code Explanation

- Borrows the sentence; owns each key; creates-at-zero then counts. Prints `2` and `3`.

## Common Mistakes

- **Case sensitivity**: `"Red"` differs from `"red"`. Fix: normalize with `to_lowercase` when wanted.
- **Missing star**: `or_insert` gives `&mut u32`. Fix: write `*slot += 1`.

## Best Practices

- Borrow input, own keys; read counts through `get` so missing words stay safe.

## Exercise

Create a cargo project `counter_task` with `cargo new counter_task`. Implement `count_words` for the criteria.

```rust
// src/main.rs starter — replace todo!() with your code:
use std::collections::HashMap;
fn count_words(text: &str) -> HashMap<String, u32> { todo!() }
fn main() { todo!() }
```

Your `cargo test` criteria: `"red fish red"` gives red 2, fish 1; empty gives empty; `"a a a"` gives a 3.

## Solution

```rust
use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

fn main() { println!("{:?}", count_words("red fish red")); }

#[test]
fn counts_repeated_words() {
    let counts = count_words("red fish red");
    assert_eq!((counts.get("red"), counts.get("fish")), (Some(&2), Some(&1)));
}

#[test]
fn handles_empty_and_triples() {
    assert!(count_words("").is_empty());
    assert_eq!(count_words("a a a").get("a"), Some(&3));
}
```

## Mini Quiz

```yaml
- question: "What does `split_whitespace` yield?"
  options:
    - "Borrowed word views"
    - "Owned Strings"
    - "A HashMap"
    - "Chars"
  answer: 0
  explanation: "It lazily yields &str slices."
- question: "Why `to_string` keys?"
  options:
    - "Owned keys outlive input borrows"
    - "Faster hashing"
    - "Borrows banned"
    - "It sorts"
  answer: 0
  explanation: "Ownership frees map lifetimes."
- question: "What does `or_insert(0) += 1` do?"
  options:
    - "Init at zero, count one"
    - "Delete entry"
    - "Reset all"
    - "Return length"
  answer: 0
  explanation: "One lookup initializes and prepares increment."
```
