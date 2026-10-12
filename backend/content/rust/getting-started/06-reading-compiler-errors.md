---
slug: compiler-errors
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: getting-started
chapterTitle: "Chapter 1 — Getting Started"
chapterOrder: 1
title: "Reading Compiler Errors"
difficulty: beginner
order: 6
estimatedMinutes: 10
tags: [rust, compiler, errors, debugging]
---

## Learning Objective

By the end of this lesson, you will read the first `rustc` error and
apply its `help:` suggestion.

## Explanation

A **compiler error** reports a broken rule before code runs. It has a
code (`E0308`), a message (`mismatched types`), a location
(`src/main.rs:2:9`), and often a `help:` fix. Fix the first error
first; later ones are often side effects. A **warning** (like `unused
variable`) still builds but flags likely bugs. **rust-analyzer** is an
editor plugin showing the same diagnostics inline. The loop: run
`cargo check`, read the top error fully, apply `help:`, re-check.

## Why It Matters

Rust rejects risky programs early to prevent crashes and data races.
Reading errors calmly turns minutes of fear into fast fixes.

## Example

Passing `&str` where `i32` is expected yields `E0308` with `expected
i32, found &str` plus a suggestion to parse or change the type.

## Code

```rust
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("double: {}", double(21));
}
```

## Code Explanation

- `fn double(n: i32) -> i32` takes a parameter `n` and returns `i32`.
- `n * 2` without `;` is the return expression.
- `double(21)` passes the literal `21`; output is `double: 42`.

## Common Mistakes

- **Fixing the last error first**: start at the top each time.
- **Ignoring `help:`**: suggestions resolve most beginner errors.

## Best Practices

- Use `cargo check` in a tight loop for sub-second feedback.
- Read location, message, snippet, then `help:`, fixing one per cycle.

## Exercise

Find the bug and the expected versus found types:

```rust
fn main() {
    let total = "42";
    let doubled: i32 = total;
    println!("{doubled}");
}
```

What helper converts text (`parse` plus `expect`)?

## Solution

Expected `i32`, found `&str`. Parse explicitly:

```rust
fn main() {
    let total = "42";
    let doubled: i32 = total.parse().expect("number");
    println!("doubled: {}", doubled * 2);
}
```

It prints `doubled: 84`. `expect()` handles failure with a message.

## Mini Quiz

```yaml
- question: "Where do you start with several errors?"
  options:
    - "The last error"
    - "The first error"
    - "A random warning"
    - "Delete target/"
  answer: 1
  explanation: "The first error is usually the root cause."
- question: "What does `help:` do?"
  options:
    - "Publishes your crate"
    - "Suggests a concrete fix"
    - "Formats the file"
    - "Runs tests"
  answer: 1
  explanation: "rustc suggestions fix most beginner errors."
- question: "Errors versus warnings?"
  options:
    - "No difference"
    - "Errors block builds; warnings build but flag bugs"
    - "Warnings block builds"
    - "Only clippy warns"
  answer: 1
  explanation: "Fix errors to build; fix warnings to stay clear."
```
