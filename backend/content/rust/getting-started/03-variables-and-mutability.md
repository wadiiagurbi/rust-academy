---
slug: variables-and-mutability
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: getting-started
chapterTitle: "Chapter 1 — Getting Started"
chapterOrder: 1
title: "Variables and Mutability"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, variables, mutability, types]
---

## Learning Objective

You will declare `let` bindings, control mutation with `mut`, and read compiler errors.

## Explanation

A **binding** (`let name = value;`) has a fixed **type** (`i32` for `5`).

1. **Immutable by default.** `let x = 5; x = 6;` fails; opt in with `let mut x = 5;`.
2. **Static types.** Inferred but fixed — `let x = 5;` stays `i32`.

Immutability makes mutation visible, preventing bug families. **Shadowing**
(`let x = 5; let x = x + 1;`) re-binds anew, even changing type.

## Why It Matters

Mutability discipline leads to ownership and borrowing. `mut` friction trains you to see mutation.

## Example

Mutable `total` accumulates; immutable `tax_rate` shows fixed intent.

## Code

```rust
fn main() {
    let tax_rate = 0.2;
    let mut total = 0.0;

    for price in [10.0, 20.0, 30.0] {
        total += price * (1.0 + tax_rate);
    }

    println!("Total with tax: {total}");
}
```

## Code Explanation

- `let tax_rate = 0.2;` — immutable `f64`; the compiler forbids changes.
- `let mut total = 0.0;` — mutable `f64` accumulator.
- `for price in [...]` — `price` is a fresh immutable binding each round.
- `total += ...` — legal only because `total` is `mut`.
- `{total}` interpolates directly into the string.

## Common Mistakes

- Assigning to immutable needs `mut`; unused `mut` should be removed.
- Shadowing re-`let`s (may change type); `mut` updates in place.

## Best Practices

- Default to immutable; fix every `unused_mut` while learning.
- Prefer shadowing pipelines (`let s = s.trim();`) for stepwise values.

## Exercise

Fix this so it compiles and prints `count: 3` — (a) with `mut`, (b) with shadowing only.

```rust
fn main() {
    let count = 0;
    count += 1;
    count += 1;
    count += 1;
    println!("count: {count}");
}
```

## Solution

(a) With `mut`:

```rust
fn main() {
    let mut count = 0;
    count += 1;
    count += 1;
    count += 1;
    println!("count: {count}");
}
```

(b) With shadowing:

```rust
fn main() {
    let count = 0;
    let count = count + 1;
    let count = count + 1;
    let count = count + 1;
    println!("count: {count}");
}
```

Both print `count: 3` — (a) mutates state, (b) pipelines transformations.

## Mini Quiz

```yaml
- question: "What happens with `let x = 5; x = 6;`?"
  options:
    - "x becomes 6, no problem"
    - "Compile error: cannot assign twice to immutable variable"
    - "Runtime panic"
    - "x becomes 6 but with a warning"
  answer: 1
  explanation: "Bindings are immutable by default; reassignment needs `let mut x`."
- question: "How is shadowing different from mutation?"
  options:
    - "There is no difference"
    - "Shadowing creates a new binding and can change the type; mutation updates the same binding in place"
    - "Shadowing only works on strings"
    - "Mutation is faster than shadowing"
  answer: 1
  explanation: "`let x = x + 1` re-binds (possibly a new type); `mut` updates the existing binding."
- question: "Why does Rust default to immutable bindings?"
  options:
    - "To save memory"
    - "To make mutation visible and deliberate, preventing bug families"
    - "Because the compiler is slower with mut"
    - "It doesn't — everything is mutable"
  answer: 1
  explanation: "Visible mutation is the foundation of Rust's safety story, leading into ownership and borrowing."
```
