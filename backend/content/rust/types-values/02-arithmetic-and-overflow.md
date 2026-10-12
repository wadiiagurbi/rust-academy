---
slug: arithmetic-overflow
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: types-values
chapterTitle: "Chapter 2 — Types and Values"
chapterOrder: 2
title: "Arithmetic and Overflow"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, arithmetic, overflow]
---

## Learning Objective

By the end of this lesson, you will predict overflow behavior and
pick checked, wrapping, or saturating math.

## Explanation

**Arithmetic** uses `+ - * / %` with usual precedence. **Overflow**
means a result no longer fits its type. Debug builds **panic** (halt
with an error); release builds **wrap** (cycle modulo 2^N). Three
methods stay explicit: `checked_add` returns an **Option** (`Some` or
`None`), `wrapping_add` always wraps, and `saturating_add` clamps at
the limit. Constants overflowing fail at compile time, so pass values
through functions to see runtime behavior.

## Why It Matters

Overflow corrupts balances and indexes. Rust forces the choice at the
call site instead of leaving it undefined.

## Example

`interproduct(120, 100, 248)` fits `i32` but would overflow `i16`.
`saturating_add` on `u8` clamps instead of wrapping.

## Code

```rust
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    a * b + b * c + c * a
}

fn main() {
    println!("result: {}", interproduct(120, 100, 248));
    println!("sat: {}", 250_u8.saturating_add(20));
}
```

## Code Explanation

- `-> i32` is the return type; the bare `a * b + ...` returns it.
- `*` binds tighter than `+`, giving `66560`.
- `saturating_add` clamps `250 + 20` to `255`, not `14`.

## Common Mistakes

- **Expecting release panics**: release wraps by default.
- **`%` on negatives**: `-5 % 2` is `-1`. Use `rem_euclid()`.

## Best Practices

- Keep money in wide types and use `checked_*` near limits.
- Name intentional wraps with `wrapping_*` for reviewers.

## Exercise

What prints in debug versus release? Make it consistent:

```rust
fn main() {
    let a: u8 = 200;
    let b: u8 = 100;
    println!("{}", a + b);
}
```

## Solution

Debug panics; release prints `44` (`300 % 256`):

```rust
fn main() {
    let a: u8 = 200;
    let b: u8 = 100;
    let sum = a.checked_add(b).expect("overflow");
    println!("sum: {sum}");
}
```

`checked_add` plus `expect()` panics with your message everywhere.

## Mini Quiz

```yaml
- question: "Debug overflow does what?"
  options:
    - "Wraps silently"
    - "Panics with a message"
    - "Formats code"
    - "Is impossible"
  answer: 1
  explanation: "Debug checks overflow and halts."
- question: "checked_add on overflow returns?"
  options:
    - "Wrapped value"
    - "None as an Option"
    - "MAX value"
    - "An exception"
  answer: 1
  explanation: "checked_* gives Some(v) or None."
- question: "250_u8.saturating_add(20)?"
  options:
    - "14"
    - "255"
    - "Error"
    - "None"
  answer: 1
  explanation: "Saturation clamps at the type limit."
```
