---
slug: patterns-intro
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: pattern-matching
chapterTitle: "Chapter 7 — Pattern Matching"
chapterOrder: 7
title: "Patterns Intro"
difficulty: beginner
order: 1
estimatedMinutes: 9
tags: [rust, patterns, match, enums]
---

## Learning Objective

By the end of this lesson, you will use a `match` expression to handle each variant of an enum.

## Explanation

A **pattern** is a shape compared against a value. An **enum** is a type with several named options called **variants**. A `match` lists patterns as **arms** and runs the first arm that fits. A `match` must be **exhaustive**, meaning every possible value has an arm, or the compiler rejects it. The wildcard `_` matches anything and works as a final catch-all.

## Why It Matters

Exhaustive matching keeps new variants visible: adding one breaks every incomplete `match` at compile time instead of hiding a bug.

## Example

A traffic light enum maps to driver instructions with three arms and no `if` chains.

## Code

```rust
enum Light { Red, Yellow, Green }

fn action(light: Light) -> &'static str {
    match light {
        Light::Red => "stop",
        Light::Yellow => "slow down",
        Light::Green => "go",
    }
}

fn main() {
    println!("{}", action(Light::Yellow));
}
```

## Code Explanation

- `enum Light` declares three variants: `Red`, `Yellow`, `Green`.
- `-> &'static str` returns a string slice living for the whole program.
- Each arm pairs a pattern with its result; `match light` moves the value in and only the fitting arm runs.

## Common Mistakes

- **Missing an arm**: omitting a variant errors with `non-exhaustive patterns`. Fix: add the arm or a `_` catch-all last.
- **Putting `_` first**: later arms become unreachable. Fix: keep `_` last.

## Best Practices

- Keep arm bodies short; list real variants before any `_` arm.

## Exercise

Predict the output, then add a `Flashing` variant returning `"caution"` so the code compiles again.

```rust
enum Shape { Circle, Square }

fn sides(shape: Shape) -> u32 {
    match shape {
        Shape::Circle => 0,
        Shape::Square => 4,
    }
}

fn main() {
    println!("{}", sides(Shape::Square));
}
```

## Solution

It prints `4`. The new variant needs its own arm:

```rust
enum Shape { Circle, Square, Flashing }

fn sides(shape: Shape) -> u32 {
    match shape {
        Shape::Circle | Shape::Flashing => 0,
        Shape::Square => 4,
    }
}

fn main() {
    println!("{}", sides(Shape::Square));
}
```

## Mini Quiz

```yaml
- question: "What does it mean for a `match` to be exhaustive?"
  options:
    - "It runs very quickly"
    - "Every possible value has a matching arm"
    - "It contains at least ten arms"
    - "It uses only `_`"
  answer: 1
  explanation: "The compiler requires every possible value to be covered."
- question: "What does the `_` pattern do?"
  options:
    - "Matches anything, used as a catch-all"
    - "Matches only zero"
    - "Ends the program"
    - "Declares a variable"
  answer: 0
  explanation: "`_` is the wildcard; placed last it catches the rest."
- question: "How does `match` pick an arm?"
  options:
    - "It runs all arms"
    - "It picks randomly"
    - "It runs the first fitting arm, top to bottom"
    - "It always runs the last arm"
  answer: 2
  explanation: "Arms are tested in order; only the first hit executes."
```
