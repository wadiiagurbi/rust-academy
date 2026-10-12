---
slug: rot13
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: std-traits
chapterTitle: "Chapter 12 — Standard Library Traits"
chapterOrder: 12
title: "ROT13 Exercise"
difficulty: beginner
order: 4
estimatedMinutes: 18
tags: [rust, traits, exercise, testing]
---

## Learning Objective

By the end of this lesson, you will implement ROT13 with iterator adapters and test round-trips.

## Explanation

**ROT13** rotates each ASCII letter 13 places (a to n). **ASCII** is the 128-character basic set with `a`-`z`, `A`-`Z` ranges. An **iterator adapter** lazily transforms items; **`collect`** gathers them into a `String`. A **round-trip** encodes twice to restore the original, since 13 + 13 spans the 26-letter alphabet.

## Why It Matters

One cipher chain exercises chars, ranges, mapping, and collecting with traits doing the work.

## Example

`"Hello"` encodes to `"Uryyb"`; encoding again restores `"Hello"` while digits pass through.

## Code

```rust
fn rot13(input: &str) -> String {
    input.chars().map(|c| match c {
        'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
        'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
        other => other,
    }).collect()
}

fn main() {
    println!("{} {}", rot13("Hello"), rot13(&rot13("Hello")));
}
```

## Code Explanation

- `chars()` iterates characters; `as u8`/`as char` shifts byte values within each half-range.
- `other` passes spaces and digits through; output is `Uryyb Hello`.

## Common Mistakes

- **Shifting past `z`**: adding 13 to `n` overshoots. Fix: split ranges at `m`/`M`.
- **Dropping non-letters**: digits need passthrough. Fix: keep the `other` arm.

## Best Practices

- Match range halves explicitly; test empty input, punctuation, and double-encode trips.

## Exercise

Create a cargo project `rot13_task` with `cargo new rot13_task`. Implement `rot13` for the criteria.

```rust
// src/main.rs starter — replace todo!() with your code:
fn rot13(input: &str) -> String {
    todo!()
}

fn main() {
    todo!()
}
```

Your `cargo test` criteria: `"Hello"` gives `"Uryyb"`, `"a1b2"` gives `"n1o2"`, double-encode restores input.

## Solution

```rust
fn rot13(input: &str) -> String {
    input.chars().map(|c| match c {
        'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
        'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
        other => other,
    }).collect()
}

fn main() {
    println!("{}", rot13("Hello"));
}

#[test]
fn encodes_and_round_trips() {
    assert_eq!(rot13("Hello"), "Uryyb");
    assert_eq!(rot13("a1b2"), "n1o2");
    assert_eq!(rot13(&rot13("Rust 123!")), "Rust 123!");
}
```

## Mini Quiz

```yaml
- question: "What does ROT13 do per letter?"
  options:
    - "Rotates 13 places in its case range"
    - "Deletes it"
    - "Makes a digit"
    - "Uppercases always"
  answer: 0
  explanation: "Half-alphabets swap with wraparound."
- question: "Why does double ROT13 restore text?"
  options:
    - "13 + 13 spans all 26 letters"
    - "It caches input"
    - "Chars copy twice"
    - "Collect reverses"
  answer: 0
  explanation: "Two halves complete one cycle."
- question: "What happens to digits?"
  options:
    - "Pass through via catch-all"
    - "Become letters"
    - "Deleted"
    - "Panic"
  answer: 0
  explanation: "`other` returns them untouched."
```
