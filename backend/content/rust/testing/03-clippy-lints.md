---
slug: clippy
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: testing
chapterTitle: "Chapter 19 — Testing"
chapterOrder: 19
title: "Clippy Lints"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, testing, clippy, lints]
---

## Learning Objective

By the end of this lesson, you will run `cargo clippy`, read a lint, and apply its suggested fix.

## Explanation

**Clippy** is Rust's official **linter** (a tool reading code without running it, flagging buggy, slow, or unidiomatic patterns). Run `cargo clippy` after tests pass; each finding (a **lint**) names the pattern, points at lines, and usually suggests a rewrite. A **warning** still compiles — but idiomatic Rust treats a clean Clippy run as part of "done", often enforced with `cargo clippy -- -D warnings` (turning warnings into denied errors).

## Why It Matters

Clippy is a senior reviewer on call: it teaches idioms at the exact moment you violate them.

## Example

A manual loop totalling an array — correct, but Clippy rewrites it as an iterator sum.

## Code

```rust
fn main() {
    let numbers = [1, 2, 3, 4];
    let mut total = 0;
    for n in numbers.iter() {
        total += *n;
    }
    println!("{total}");
}
```

## Code Explanation

- `iter()` borrows (`&i32`), so `total += *n` must **dereference** (follow the reference) first.
- Prints `10`, warning-free under `rustc` — yet Clippy flags the dance: `numbers.iter().sum()` says the same thing.

## Common Mistakes

- **Ignoring lints**: the `help:` suggestion is the lesson; read it before dismissing anything.
- **Piling up warnings**: one hides the next. Fix or scoped-`#[allow(...)]` (attribute silencing one spot) immediately.
- **Clippy only at the end**: per-session runs keep each fix tiny and educational.

## Best Practices

- Order: `cargo test`, `cargo clippy`, `cargo fmt` (formatter) — green, idiomatic, tidy.
- Gate merges on `cargo clippy -- -D warnings` so new lints fail CI instead of lingering.

## Exercise

Run `cargo clippy` on the program (via `cargo new clippy_lab`). Name the lint, apply its suggestion — does the output change?

## Solution

```rust
fn main() {
    let numbers = [1, 2, 3, 4];
    let total: i32 = numbers.iter().sum();
    println!("{total}");
}
```

Clippy suggests `.iter().sum()`; output stays `10` — same behaviour minus the accumulator and dereference.

## Mini Quiz

```yaml
- question: "What is Clippy?"
  options:
    - "A formatter"
    - "The official linter: flags unidiomatic patterns without running code"
    - "A test runner"
    - "A registry"
  answer: 1
  explanation: "`cargo clippy` statically analyses code and suggests rewrites."
- question: "`cargo clippy -- -D warnings`?"
  options:
    - "Disables warnings"
    - "Denies warnings: any lint fails the build"
    - "Shows errors only"
    - "Deletes history"
  answer: 1
  explanation: "`-D warnings` elevates lints to errors, commonly gating CI."
- question: "Why prefer `.iter().sum()`?"
  options:
    - "Different result"
    - "Same result minus `mut`, indexing, and dereference"
    - "Loops are deprecated"
    - "sum() needs empty input"
  answer: 1
  explanation: "The iterator form removes the accumulator and off-by-one surface."
```
