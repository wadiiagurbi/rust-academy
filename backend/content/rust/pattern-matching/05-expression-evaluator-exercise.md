---
slug: expression-evaluator
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: pattern-matching
chapterTitle: "Chapter 7 — Pattern Matching"
chapterOrder: 7
title: "Expression Evaluator Exercise"
difficulty: intermediate
order: 5
estimatedMinutes: 20
tags: [rust, match, enums, exercise]
---

## Learning Objective

By the end of this lesson, you will evaluate a recursive expression tree with `match` and guards.

## Explanation

A **recursive enum** holds itself inside a variant, via a **`Box`** (an owning heap pointer) so the type has known size. An **expression tree** nests math nodes like `Add(Number(2), Mul(Number(3), Number(4)))`. A **recursive function** calls itself on smaller inputs; each call matches one node and recurses into children.

## Why It Matters

Compilers and parsers are trees plus `match`. This exercise proves you can model nesting and cover every shape.

## Example

`2 + 3 * 4` is an `Add` node holding `2` and a `Mul` node; evaluation recurses down, then combines upward.

## Code

```rust
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(l, r) => eval(l) + eval(r),
        Expr::Mul(l, r) => eval(l) * eval(r),
    }
}

fn main() {
    let tree = Expr::Add(Box::new(Expr::Number(2)), Box::new(Expr::Number(6)));
    println!("{}", eval(&tree));
}
```

## Code Explanation

- `Box<Expr>` stores children on the heap for a fixed enum size; `eval` borrows `&Expr` and `*n` copies out.
- The tree is `2 + 6`, printing `8`.

## Common Mistakes

- **Skipping `Box`**: plain recursion errors with `infinite size`. Fix: wrap children in `Box`.
- **Returning `n` not `*n`**: that yields `&i64` where `i64` is needed. Fix: dereference to copy.

## Best Practices

- Borrow (`&Expr`) for read-only passes; add guards for invalid input like division by zero.

## Exercise

Create a cargo project `expr_eval` with `cargo new expr_eval`. Add a `Div` variant and `eval_checked` returning `Option<i64>` (`None` on divide-by-zero).

```rust
// src/main.rs starter — replace todo!() with your code:
enum Expr { Number(i64), Add(Box<Expr>, Box<Expr>), Div(Box<Expr>, Box<Expr>) }
fn eval_checked(expr: &Expr) -> Option<i64> { todo!() }
fn main() { todo!() }
```

Your `cargo test` criteria: `2 + 3 * 4 == Some(14)`, `10 / 2 == Some(5)`, `1 / 0 == None`.

## Solution

```rust
enum Expr { Number(i64), Add(Box<Expr>, Box<Expr>), Div(Box<Expr>, Box<Expr>) }
fn eval_checked(expr: &Expr) -> Option<i64> {
    match expr {
        Expr::Number(n) => Some(*n),
        Expr::Add(l, r) => Some(eval_checked(l)? + eval_checked(r)?),
        Expr::Div(l, r) => { let d = eval_checked(r)?; if d == 0 { None } else { Some(eval_checked(l)? / d) } }
    }
}
fn main() { println!("{:?}", eval_checked(&Expr::Div(Box::new(Expr::Number(10)), Box::new(Expr::Number(2))))); }
#[test]
fn checks_criteria() {
    let t = Expr::Add(Box::new(Expr::Number(2)), Box::new(Expr::Number(6)));
    assert_eq!(eval_checked(&t), Some(8));
    assert_eq!(eval_checked(&Expr::Div(Box::new(Expr::Number(10)), Box::new(Expr::Number(2)))), Some(5));
    assert_eq!(eval_checked(&Expr::Div(Box::new(Expr::Number(1)), Box::new(Expr::Number(0)))), None);
}
```

## Mini Quiz

```yaml
- question: "Why does a recursive enum need `Box`?"
  options:
    - "So the type has a known, finite size"
    - "To run faster"
    - "To allow more variants"
    - "It doesn't"
  answer: 0
  explanation: "`Box` puts children on the heap behind a fixed-size pointer."
- question: "What does `?` do on Option here?"
  options:
    - "Panics on None"
    - "Returns None from the function early"
    - "Converts None to zero"
    - "Ignores the value"
  answer: 1
  explanation: "`?` unwraps Some or exits with None at once."
- question: "What shape does `eval` recurse over?"
  options:
    - "A flat integer"
    - "A tree of nested Add/Mul/Div nodes"
    - "Source text"
    - "A hash map"
  answer: 1
  explanation: "Each arm handles one node and recurses into children."
```
