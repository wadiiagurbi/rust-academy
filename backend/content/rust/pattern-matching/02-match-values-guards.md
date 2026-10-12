---
slug: match-guards
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: pattern-matching
chapterTitle: "Chapter 7 — Pattern Matching"
chapterOrder: 7
title: "Match Values and Guards"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, match, guards, patterns]
---

## Learning Objective

By the end of this lesson, you will bind values in patterns and filter arms with a guard condition.

## Explanation

**Binding** names part of a matched value for use in the arm body. A **match guard** is an extra `if` condition after a pattern; the arm runs only when both fit. The `|` symbol means "or" in patterns, and `1..=12` is an **inclusive range pattern** matching 1 through 12.

## Why It Matters

Guards keep classification in one place: the pattern names the shape, the guard states the rule, and exhaustiveness is still checked.

## Example

A ticket desk prices sections 1-2 and 3-5, with youth and senior discounts applied through guards.

## Code

```rust
fn ticket_price(section: u32, age: u32) -> u32 {
    match section {
        1 | 2 if age < 12 => 10,
        1 | 2 => 25,
        3..=5 if age >= 65 => 15,
        3..=5 => 20,
        _ => 30,
    }
}

fn main() {
    println!("{} {} {}", ticket_price(2, 8), ticket_price(4, 70), ticket_price(9, 30));
}
```

## Code Explanation

- `1 | 2 if age < 12` matches sections 1-2 only for children; the guard filters after the pattern fits.
- `3..=5` covers 3, 4, 5 inclusive; `_` catches every other section.

## Common Mistakes

- **Broad arm hiding a guarded one**: order matters top to bottom. Fix: put specific guarded arms first.
- **Confusing `..=` with `..`**: only `..=` includes the end. Fix: use `..=` for closed ranges.

## Best Practices

- Group related cases with `|` and ranges; keep guards small enough to read aloud.

## Exercise

Predict the three numbers, then add a `0 => 0` arm for preview seats.

```rust
fn ticket_price(section: u32, age: u32) -> u32 {
    match section {
        1 | 2 if age < 12 => 10,
        1 | 2 => 25,
        _ => 30,
    }
}

fn main() {
    println!("{}", ticket_price(2, 8));
}
```

## Solution

It prints `10`. Section 0 goes first so nothing hides it:

```rust
fn ticket_price(section: u32, age: u32) -> u32 {
    match section {
        0 => 0,
        1 | 2 if age < 12 => 10,
        1 | 2 => 25,
        _ => 30,
    }
}

fn main() {
    println!("{}", ticket_price(0, 40));
}
```

## Mini Quiz

```yaml
- question: "What is a match guard?"
  options:
    - "An extra `if` on an arm that must also be true"
    - "A thread lock"
    - "A comment above an arm"
    - "A nested match"
  answer: 0
  explanation: "Guards filter an arm after its pattern matches."
- question: "What does `1 | 2` mean in a pattern?"
  options:
    - "The number 12"
    - "Either 1 or 2"
    - "1 divided by 2"
    - "An exclusive range"
  answer: 1
  explanation: "`|` lets one arm cover several shapes."
- question: "What does `3..=5` match?"
  options:
    - "Only 3"
    - "3, 4, and 5 inclusive"
    - "Everything except 3-5"
    - "Only above 5"
  answer: 1
  explanation: "`..=` includes both endpoints."
```
