---
slug: std-comparisons
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: std-traits
chapterTitle: "Chapter 12 — Standard Library Traits"
chapterOrder: 12
title: "Comparisons and Operators"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, traits, partialord, operators]
---

## Learning Objective

By the end of this lesson, you will derive comparison traits and overload `+` with `Add`.

## Explanation

**`PartialEq`** enables `==`; **`Eq`** marks it total. **`PartialOrd`** enables `<` that may be undefined (float `NaN`, "not a number"); **`Ord`** marks total ordering. **Operator overloading** teaches `+` via the **`Add`** trait (`fn add(self, rhs) -> Output`). **Deriving** generates these impls field by field.

## Why It Matters

Sorting, max-finding, and `+` on quantities all rest on these traits, derivable in one line.

## Example

Scores sort low to high; two sack weights add into a new total through `+`.

## Code

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Score(u32);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Sack(f64);

impl Add for Sack {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

fn main() {
    let mut scores = vec![Score(9), Score(4)];
    scores.sort();
    println!("{scores:?} {:?}", Sack(2.5) + Sack(1.5));
}
```

## Code Explanation

- `Score` derives full ordering by its `u32`; `sort()` orders ascending.
- `Add` sums inner weights; output is `[Score(4), Score(9)]` and `Sack(4.0)`.

## Common Mistakes

- **Sorting floats with `sort`**: `f64` lacks `Ord` via `NaN`. Fix: use `partial_cmp` or wrappers.
- **Missing `Output`**: `Add` requires it. Fix: set `type Output = Self;`.

## Best Practices

- Derive full comparisons on integer newtypes; keep `+` unsurprising and non-mutating.

## Exercise

Predict the output, then add `Score(7)` and describe the new order.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Score(u32);

fn main() {
    let mut scores = vec![Score(9), Score(4)];
    scores.sort();
    println!("{scores:?}");
}
```

## Solution

It prints `[Score(4), Score(9)]`. The newcomer slots mid-list:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Score(u32);

fn main() {
    let mut scores = vec![Score(9), Score(4), Score(7)];
    scores.sort();
    println!("{scores:?}");
}
```

Prints `[Score(4), Score(7), Score(9)]`.

## Mini Quiz

```yaml
- question: "Which trait enables `<`?"
  options:
    - "PartialOrd"
    - "Clone"
    - "Debug"
    - "Default"
  answer: 0
  explanation: "It powers sort and max."
- question: "Why does f64 lack Ord?"
  options:
    - "NaN leaves comparisons undefined"
    - "Floats are slow"
    - "Floats can't copy"
    - "Compiler dislike"
  answer: 0
  explanation: "NaN breaks total ordering."
- question: "How to teach `+`?"
  options:
    - "Implement Add with Output and add"
    - "Derive Copy"
    - "Write a macro"
    - "Implement Display"
  answer: 0
  explanation: "Add hooks `+` to your function."
```
