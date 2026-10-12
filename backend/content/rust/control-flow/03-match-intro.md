---
slug: match-intro
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: control-flow
chapterTitle: "Chapter 3 — Control Flow"
chapterOrder: 3
title: "Match Intro"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, match, patterns, control-flow]
---

## Learning Objective

By the end of this lesson, you will write an exhaustive `match` that
returns one shared type.

## Explanation

A **`match` expression** tests arms top-down; the first matching
**pattern** wins with no fallthrough. `_` (called **wildcard**)
matches anything as the default. **Exhaustive** means covering every
input, or compilation fails. All arm bodies must share one type since
`match` returns the taken value. Bodies use `=> value,` or braces for
longer work.

## Why It Matters

`match` replaces long `else-if` chains with checked tables, catching
missing cases before they run.

## Example

Dice faces map `1` to 100, `10` to 50, else 5. A `bool` maps to 1/0,
showing `match` as an expression.

## Code

```rust
fn main() {
    let val = 10;
    let points = match val {
        1 => 100,
        10 => 50,
        _ => 5,
    };
    println!("points: {points}");
}
```

## Code Explanation

- `10 => 50` fires here, so `points` is `50`.
- `_` keeps the match exhaustive for other numbers.

## Common Mistakes

- **Missing `_`**: integers without `_` fail as non-exhaustive.
- **Expecting fallthrough**: only one arm runs; use `1 | 2 =>`.

## Best Practices

- Order specific arms first, `_` last.
- Return from `match` into one `let` instead of assigning inside.

## Exercise

What prints? Add `100 => 1000` and test `100`:

```rust
fn main() {
    let flag = false;
    let val = match flag {
        true => 1,
        false => 0,
    };
    println!("val: {val}");
}
```

## Solution

Prints `val: 0`:

```rust
fn main() {
    let val = 100;
    let points = match val {
        1 => 100,
        100 => 1000,
        _ => 5,
    };
    println!("points: {points}");
}
```

Prints `points: 1000`. Dropping `false` breaks exhaustiveness.

## Mini Quiz

```yaml
- question: "How do match arms run?"
  options:
    - "All in parallel"
    - "First match wins, no fallthrough"
    - "Last match wins"
    - "Randomly"
  answer: 1
  explanation: "Arms test top-down, one body runs."
- question: "What does _ do?"
  options:
    - "Skips match"
    - "Matches anything as default"
    - "Matches zero"
    - "Exits"
  answer: 1
  explanation: "Wildcard covers remaining values."
- question: "Arm body type rule?"
  options:
    - "Any type each"
    - "All share one type"
    - "Must be ()"
    - "First only matters"
  answer: 1
  explanation: "match has one result type."
```
