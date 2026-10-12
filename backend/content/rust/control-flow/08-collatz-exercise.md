---
slug: collatz
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: control-flow
chapterTitle: "Chapter 3 — Control Flow"
chapterOrder: 3
title: "Exercise: Collatz Sequence"
difficulty: beginner
order: 8
estimatedMinutes: 15
tags: [rust, exercise, loops, collatz]
---

## Learning Objective

By the end of this lesson, you will implement `collatz_length` in
Cargo and verify it with asserts.

## Explanation

The **Collatz sequence** from positive `n`: 1 ends it; even goes `n /
2`; odd goes `3 * n + 1`. **Length** counts terms including ends (3
gives 8 terms). `mut n: i32` means the local copy is mutable. An
**`if` expression** picks the next value in one line. A **Cargo task**
is `cargo new`, editing `src/main.rs`, proving with `cargo run`.
`todo!()` stubs panic until replaced.

## Why It Matters

Collatz joins `while`, `mut`, and `if`-as-value — the core loop idiom
for simulations and retries.

## Example

From 11 the chain has 15 terms ending at 1. Evens halve, odds go
`3n+1`, and a counter tracks steps.

## Code

```rust
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

fn main() {
    println!("length: {}", collatz_length(11));
}
```

## Code Explanation

- `mut n` allows updating the local copy; callers pass by value.
- `while n > 1` runs until 1; bare `len` returns the count (15).

## Common Mistakes

- **Missing `mut`**: assigning `n` then fails as immutable.
- **Starting `len` at 0**: length includes the start, so use 1.

## Best Practices

- Scope to positive inputs with `while n > 1` documented.
- Assign via `if`-expression to avoid duplicating `n =`.

## Exercise

Implement the stub so asserts pass:

```bash
cargo new collatz_exercise
cd collatz_exercise
cargo run
```

```rust
fn collatz_length(mut n: i32) -> u32 {
    todo!("implement this")
}

fn main() {
    assert_eq!(collatz_length(1), 1);
    assert_eq!(collatz_length(11), 15);
    println!("length: {}", collatz_length(11));
}
```

## Solution

Count terms while stepping:

```rust
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

fn main() {
    assert_eq!(collatz_length(11), 15);
    println!("length: {}", collatz_length(11));
}
```

Prints `length: 15`. Integers are `Copy`, so `mut n` never affects
callers.

## Mini Quiz

```yaml
- question: "Collatz length of 1?"
  options:
    - "0"
    - "1 counting the start"
    - "15"
    - "Infinite"
  answer: 1
  explanation: "Length includes the starting term."
- question: "Why mut n?"
  options:
    - "Change caller var"
    - "Update the local copy"
    - "Make a string"
    - "Required always"
  answer: 1
  explanation: "i32 is Copy; mut affects the local binding."
- question: "n = if c { a } else { b } shows?"
  options:
    - "Statement"
    - "if as value-producing expression"
    - "A macro"
    - "Endless loop"
  answer: 1
  explanation: "if yields the branch value into n."
```
