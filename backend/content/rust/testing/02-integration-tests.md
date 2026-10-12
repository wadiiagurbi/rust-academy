---
slug: integration-tests
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: testing
chapterTitle: "Chapter 19 — Testing"
chapterOrder: 19
title: "Integration Tests"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, testing, integration-tests]
---

## Learning Objective

By the end of this lesson, you will place integration tests in `tests/` and import your library crate with `use`.

## Explanation

An **integration test** checks your crate from the outside, as users see it. Convention: each `.rs` file in `tests/` (folder at the package root) becomes its own test binary linked against your **library crate** (`src/lib.rs`, imported by package name like `use word_tools::reverse;`). Being external clients, these tests touch only your `pub` API — private helpers stay invisible, so they verify real public behaviour. Each file compiles separately; helpers shared across files live in `tests/common/mod.rs` (via `mod common;`).

## Why It Matters

Unit tests prove internals; integration tests prove promises. A break here means users would feel the change.

## Example

A `reverse` helper tested as an outside crate would: import, call, assert — living in `tests/reverse.rs`.

## Code

```rust
// File: tests/reverse.rs — runs against the library via `cargo test`.
// (Helper defined locally so this sample compiles standalone.)
fn reverse(word: &str) -> String {
    word.chars().rev().collect()
}

fn main() {}

#[test]
fn reverses_hello() {
    assert_eq!(reverse("hello"), "olleh");
}

#[test]
fn reversing_twice_restores_input() {
    assert_eq!(reverse(&reverse("rust")), "rust");
}
```

## Code Explanation

- `chars().rev().collect()` reverses Unicode scalar values (byte reversal would corrupt multi-byte characters).
- `fn main() {}` keeps the file a valid standalone binary; under `cargo test` the harness supplies its own entry point.

## Common Mistakes

- **Testing a binary-only crate**: `tests/` needs a library target. Move shared logic to `src/lib.rs`.
- **Reaching for privates**: `error[E0603]: function is private` means you crossed the API boundary — test through `pub` fns.
- **One giant `tests/all.rs`**: split by feature (`reverse.rs`, `stats.rs`) for parallel, focused output.

## Best Practices

- Test public behaviour, not internals — refactor-proof by construction.
- Mirror feature names in file names; share fixtures via `tests/common/mod.rs`.

## Exercise

Go real: `cargo new word_tools --lib`, move `reverse` into `src/lib.rs` as `pub fn`, write `tests/reverse.rs` with `use word_tools::reverse;`. What changes versus the sample?

## Solution

`src/lib.rs` holds just the `pub fn reverse`; `tests/reverse.rs` becomes:

```rust
use word_tools::reverse;

#[test]
fn reverses_hello() {
    assert_eq!(reverse("hello"), "olleh");
}

#[test]
fn reversing_twice_restores_input() {
    assert_eq!(reverse(&reverse("rust")), "rust");
}

fn main() {}
```

The local helper is replaced by the `use` import; assertions untouched. `cargo test` shows separate `test result` lines per binary.

## Mini Quiz

```yaml
- question: "Where do integration tests live?"
  options:
    - "In `src/`"
    - "Each `tests/*.rs` file becomes a binary against the library crate"
    - "In doc comments"
    - "On a test server"
  answer: 1
  explanation: "Cargo compiles every `tests/*.rs` as its own external test binary."
- question: "Why only the `pub` API?"
  options:
    - "Speed"
    - "External clients verify exactly what users can touch"
    - "Privates are untestable"
    - "Historical accident"
  answer: 1
  explanation: "Failures there mean downstream users would feel the change."
- question: "Why split `tests/` into files?"
  options:
    - "Independent parallel binaries with focused output"
    - "Cargo requires three files"
    - "Files run faster"
    - "To skip `#[test]`"
  answer: 0
  explanation: "Per-file binaries isolate features with separate summaries."
```
