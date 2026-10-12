---
slug: derive
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: methods-traits
chapterTitle: "Chapter 8 — Methods and Traits"
chapterOrder: 8
title: "Derive"
difficulty: beginner
order: 4
estimatedMinutes: 8
tags: [rust, derive, debug, clone]
---

## Learning Objective

By the end of this lesson, you will derive `Debug`, `Clone`, and `PartialEq` for your types.

## Explanation

**`#[derive]`** is an **attribute** (a `#`-marker above an item) generating a trait impl automatically. **`Debug`** enables programmer-facing `{:?}` output; **`Clone`** enables explicit `.clone()` duplicates; **`PartialEq`** enables `==` comparison. Generation follows fields, so plain structs get all three in one line.

## Why It Matters

One attribute replaces hand-written debug, copy, and equality code across configs and test values.

## Example

A server config derives all three so it logs, duplicates for workers, and asserts in tests.

## Code

```rust
#[derive(Debug, Clone, PartialEq)]
struct Config {
    host: String,
    port: u16,
}

fn main() {
    let first = Config { host: String::from("local"), port: 8080 };
    let second = first.clone();
    println!("{first:?}");
    println!("{}", first == second);
}
```

## Code Explanation

- `derive` writes the three impls at compile time.
- `clone()` duplicates the owned struct; `{:?}` prints developers' output.
- `==` compares field by field; output ends with `true`.

## Common Mistakes

- **Field lacking the trait**: a non-`Clone` field blocks deriving `Clone`. Fix: derive it there too.
- **Using `{}` for Debug-only types**: `{}` needs `Display`. Fix: use `{:?}` for derived output.

## Best Practices

- Derive `Debug` on nearly every type; add `Clone` and `PartialEq` for data and tests.

## Exercise

Predict the output, then add a `retries: u32` field set to `3` at both construction sites.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Config {
    host: String,
    port: u16,
}

fn main() {
    let first = Config { host: String::from("local"), port: 8080 };
    println!("{first:?}");
}
```

## Solution

It prints the struct fields. With the new field:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Config {
    host: String,
    port: u16,
    retries: u32,
}

fn main() {
    let first = Config { host: String::from("local"), port: 8080, retries: 3 };
    println!("{first:?}");
}
```

## Mini Quiz

```yaml
- question: "What does `#[derive(Debug)]` do?"
  options:
    - "Generates Debug automatically"
    - "Runs debug mode"
    - "Adds a debug field"
    - "Disables opts"
  answer: 0
  explanation: "The compiler writes the impl for you."
- question: "Which placeholder uses Debug?"
  options:
    - "`{:?}`"
    - "`{}`"
    - "`{x}` bare"
    - "`{d}`"
  answer: 0
  explanation: "`{:?}` prints Debug; `{}` needs Display."
- question: "What does PartialEq allow?"
  options:
    - "Comparing with `==`"
    - "Cloning"
    - "Printing always"
    - "Sorting"
  answer: 0
  explanation: "Derived impl compares each field."
```
