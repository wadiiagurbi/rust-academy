---
slug: functions
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: control-flow
chapterTitle: "Chapter 3 — Control Flow"
chapterOrder: 3
title: "Functions"
difficulty: beginner
order: 6
estimatedMinutes: 10
tags: [rust, functions, parameters, return]
---

## Learning Objective

By the end of this lesson, you will define typed functions returning
via tail expressions.

## Explanation

A **function** is `fn name(params) -> Type { body }`. A **parameter**
is a typed input slot (`a: u32`); an **argument** is the passed value
(`gcd(143, 52)`). The **return type** after `->` states output; absent
means `()` (unit). The trailing expression without `;` (the **tail**)
returns; `return x;` exits early. Rust has no **overloading** (one
name, one signature) and uses **snake_case** names.

## Why It Matters

Typed signatures are checked docs: callers see inputs and the
compiler rejects mismatches before running.

## Example

`gcd` recurses with remainders until zero; `main` prints one call,
separating math from output.

## Code

```rust
fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 { gcd(b, a % b) } else { a }
}

fn main() {
    println!("gcd: {}", gcd(143, 52));
}
```

## Code Explanation

- Parameters need `: Type`; locals may infer.
- The `if` without `;` returns directly; output is `gcd: 13`.

## Common Mistakes

- **Tail with `;`**: `else { a };` returns `()` and mismatches.
- **Missing param types**: `fn f(a)` fails; annotate every param.

## Best Practices

- One job per function; print in `main`, compute in helpers.
- Use tails for results, `return` only for early exits.

## Exercise

Write `fn is_even(n: u32) -> bool` and print both calls:

```rust
fn main() {
    println!("4: {}", true);
}
```

## Solution

Tail returns the comparison:

```rust
fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn main() {
    println!("4: {}", is_even(4));
    println!("7: {}", is_even(7));
}
```

Prints `4: true` and `7: false`.

## Mini Quiz

```yaml
- question: "Where do param types go?"
  options:
    - "Nowhere"
    - "On every param like a: u32"
    - "Only returns"
    - "Cargo.toml"
  answer: 1
  explanation: "Params require types; locals infer."
- question: "How does a tail return?"
  options:
    - "Bare final expression"
    - "Only via return"
    - "Via print"
    - "Never"
  answer: 0
  explanation: "No semicolon means return the value."
- question: "Function overloading?"
  options:
    - "Unlimited"
    - "No, one signature per name"
    - "Only main"
    - "Only macros"
  answer: 1
  explanation: "Each name has one fixed list."
```
