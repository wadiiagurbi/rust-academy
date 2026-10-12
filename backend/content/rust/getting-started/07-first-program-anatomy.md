---
slug: program-anatomy
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: getting-started
chapterTitle: "Chapter 1 — Getting Started"
chapterOrder: 1
title: "First Program Anatomy"
difficulty: beginner
order: 7
estimatedMinutes: 10
tags: [rust, basics, main, macros]
---

## Learning Objective

By the end of this lesson, you will name what `fn main`,
`println!`, statements, and expressions do.

## Explanation

A **binary crate** builds an executable. Its **entry point** is `fn
main()`, called first by the system. `fn` starts a **function**;
`main` with `()` takes no arguments. A **statement** ends with `;`
(like `let x = 3;`). An **expression** produces a value (like `3 +
4`). `println!` is a **macro** (compile-time expander, marked by `!`)
printing a line. `{name}` inside a **format string** fills from a
variable. `()` (called **unit**) means "no useful value".

## Why It Matters

Main plus macros plus statement-versus-expression repeats in every
file. Spotting those parts lets you focus on each lesson's new idea.

## Example

Two bindings plus two `println!` calls greet a visitor and show a
count, filling `{name}` from a `&str` (borrowed slice).

## Code

```rust
fn main() {
    let name = "Aisha";
    let visits = 3;
    println!("Hello, {name}!");
    println!("Visits: {visits}");
}
```

## Code Explanation

- `fn main() { ... }` is the entry point; braces delimit the block.
- `let name = "Aisha";` binds a `&str`; `let visits = 3;` infers `i32`.
- Each `println!` interpolates one variable into output.

## Common Mistakes

- **Missing `!`**: `println("hi")` finds no function. Macros need `!`.
- **Missing `;` after `let`**: bindings are statements needing `;`.

## Best Practices

- Keep `main` small; compute in helpers, print at the end.
- Run `cargo fmt` so style matches community norms.

## Exercise

Predict the output, then also print year `2026`:

```rust
fn main() {
    let city = "Lagos";
    println!("City: {city}");
}
```

Which lines are statements?

## Solution

It prints `City: Lagos`. Extended version:

```rust
fn main() {
    let city = "Lagos";
    let year = 2026;
    println!("City: {city}");
    println!("Year: {year}");
}
```

Each `let` and `println!` line is a statement ending with `;`.

## Mini Quiz

```yaml
- question: "What starts a Rust binary?"
  options:
    - "cargo fmt"
    - "fn main called by the system"
    - "rustup install"
    - "A println! macro"
  answer: 1
  explanation: "main is the entry point the OS invokes."
- question: "How do you spot a macro?"
  options:
    - "It ends with ;"
    - "Its name ends with !"
    - "It returns ()"
    - "It takes no args"
  answer: 1
  explanation: "! marks compile-time expansion."
- question: "Statement versus expression?"
  options:
    - "Statements end with ; and act; expressions produce values"
    - "They are identical"
    - "Expressions never nest"
    - "Statements return values"
  answer: 0
  explanation: "Statements run for effect; expressions evaluate to values."
```
