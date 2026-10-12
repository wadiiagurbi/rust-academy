---
slug: closure-capture
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: closures
chapterTitle: "Chapter 10 — Closures"
chapterOrder: 10
title: "Capturing Environment"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, closures, borrowing, move]
---

## Learning Objective

By the end of this lesson, you will predict borrow vs move captures and use the `move` keyword.

## Explanation

**Capturing** is a closure's use of surrounding variables. Rust takes the weakest access: shared borrow for reading, mutable borrow for mutating, **move** (ownership transfer) for consuming. The **`move` keyword** (`move || ...`) forces ownership, fitting threads and returned closures where borrows would expire.

## Why It Matters

Borrowed closures stay local; `move` closures safely travel to threads and callers.

## Example

A discount closure borrows a rate; a greeting closure owns a cloned message so the original stays usable.

## Code

```rust
fn main() {
    let rate = 0.1;
    let discount = |price: f64| price * (1.0 - rate);
    println!("{:.1}", discount(100.0));
    let message = String::from("hello");
    let owned = message.clone();
    let greet = move || owned.clone() + "!";
    println!("{}", greet());
    println!("{message}");
}
```

## Code Explanation

- `discount` borrows `rate` and prints `90.0`; `greet` moves the clone and prints `hello!`.
- `message` itself never moved, so the last line prints `hello`.

## Common Mistakes

- **Needless `move`**: blocks later use of the original. Fix: omit it for local closures.
- **Missing `move` for returns**: borrowing closures fail lifetime checks. Fix: add `move`.

## Best Practices

- Let the compiler pick captures; clone before `move` when the outer scope still needs data.

## Exercise

Predict the output and explain why the last line compiles despite `move`.

```rust
fn main() {
    let label = String::from("sale");
    let owned = label.clone();
    let show = move || owned.clone() + "?";
    println!("{}", show());
    println!("{label}");
}
```

## Solution

It prints `sale?` then `sale`: only the clone moved into `show`, while `label` stayed put.

```rust
fn main() {
    let label = String::from("sale");
    println!("{} {label}", String::from("sale?"));
}
```

## Mini Quiz

```yaml
- question: "How are captures chosen?"
  options:
    - "Weakest access that works"
    - "Always by move"
    - "Always mutably"
    - "Randomly"
  answer: 0
  explanation: "Read, mutate, or consume is inferred."
- question: "What does `move` do?"
  options:
    - "Forces ownership into the closure"
    - "Moves files"
    - "Deletes variables"
    - "Speeds code"
  answer: 0
  explanation: "Owned captures outlive the scope."
- question: "When is `move` required?"
  options:
    - "Threads or returns outliving borrows"
    - "Every closure"
    - "Math only"
    - "Never"
  answer: 0
  explanation: "Ownership satisfies those lifetimes."
```
