---
slug: unit-tests
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: testing
chapterTitle: "Chapter 19 — Testing"
chapterOrder: 19
title: "Unit Tests"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, testing, unit-tests]
---

## Learning Objective

By the end of this lesson, you will write unit tests with `#[test]` and `assert_eq!` and run them with `cargo test`.

## Explanation

A **unit test** checks one small piece of logic. Functions marked with the **`#[test]` attribute** (a `#`-annotation registering a test case) run under `cargo test` (builds the crate in test mode and executes each test). **`assert_eq!`** compares two values, panicking with a diff on mismatch; siblings are `assert!` (boolean must be true) and `assert_ne!` (must differ). Tests sit in `mod tests` gated by **`#[cfg(test)]`** (compile this module only when testing), so helpers never bloat release binaries. `use super::*;` imports the code under test.

## Why It Matters

Inline tests document intent and catch regressions where they happen — a safety net from day one, run in parallel with zero setup.

## Example

An `add` function plus two tests proving basic addition and the zero rule.

## Code

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() { println!("{}", add(2, 3)); }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() { assert_eq!(add(2, 3), 5); }

    #[test]
    fn adding_zero_changes_nothing() { assert_eq!(add(7, 0), 7); }
}
```

## Code Explanation

- `#[cfg(test)]` keeps `cargo run` clean (prints just `5`); `cargo test` runs both cases. Failures show left vs. right.

## Common Mistakes

- **Missing `#[test]`**: the function compiles but never runs — `0 tests`. The attribute registers it.
- **Testing `main`**: move logic into small pure functions and test those instead.
- **`assert!(a == b)`**: works, but `assert_eq!` prints both sides on failure.

## Best Practices

- One behaviour per test, named like a claim (`adding_zero_changes_nothing`); cover zeros, negatives, and empty inputs.

## Exercise

Add `adding_negatives` asserting `add(-4, -6) == -10`. How many tests run, and what does the summary say?

## Solution

Add the third test to the module (rest unchanged):

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() { println!("{}", add(2, 3)); }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() { assert_eq!(add(2, 3), 5); }

    #[test]
    fn adding_zero_changes_nothing() { assert_eq!(add(7, 0), 7); }

    #[test]
    fn adding_negatives() { assert_eq!(add(-4, -6), -10); }
}
```

Three tests run: `test result: ok. 3 passed; 0 failed`.

## Mini Quiz

```yaml
- question: "What does `#[test]` do?"
  options:
    - "Speeds up the function"
    - "Registers it as a test case for `cargo test`"
    - "Hides it from the compiler"
    - "Exports it from the crate"
  answer: 1
  explanation: "The harness discovers and runs `#[test]` functions."
- question: "Why gate with `#[cfg(test)]`?"
  options:
    - "Tests need Tuesdays"
    - "Test code then compiles only in test builds"
    - "It parallelizes tests"
    - "`assert_eq!` requires it"
  answer: 1
  explanation: "`cfg(test)` strips the module from normal and release builds."
- question: "`assert_eq!` vs `assert!(a == b)`?"
  options:
    - "Faster"
    - "Failures print both sides for quick diagnosis"
    - "More types"
    - "No difference"
  answer: 1
  explanation: "`assert_eq!` captures left vs. right in the panic message."
```
