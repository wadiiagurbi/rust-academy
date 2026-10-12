---
slug: scalar-types
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: types-values
chapterTitle: "Chapter 2 — Types and Values"
chapterOrder: 2
title: "Literals and Scalar Types"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, types, integers, floats]
---

## Learning Objective

By the end of this lesson, you will write literals for integers,
floats, booleans, and characters.

## Explanation

A **scalar type** holds one value. A **literal** is a value written
directly, like `42`. Integers split into signed (`i8`–`i128`,
`isize`) and unsigned (`u8`–`u128`, `usize`); the number is the bit
width. **Floats** (`f32`, `f64`) handle decimals. A **char** (`'a'`,
`'∞'`) is a 32-bit Unicode value in single quotes. A **bool** is
`true` or `false`. Underscores group digits (`1_000`); a **suffix**
pins width (`123_u16`, `2_f32`).

## Why It Matters

Choosing `u32` for counts and `f64` for measures prevents overflow
and encoding bugs while documenting intent.

## Example

Seats as `u32`, temperature as `f64`, section as `char`, and a flag
as `bool` show each scalar once in a booking record.

## Code

```rust
fn main() {
    let seats: u32 = 1_000;
    let temp: f64 = 21.5;
    let section: char = 'B';
    let open: bool = true;
    println!("{seats} {temp} {section} {open}");
}
```

## Code Explanation

- `: u32` and `: f64` are type annotations; `char` needs single
  quotes (double quotes make `&str`).
- `bool` holds flags; one `println!` shows all four values.

## Common Mistakes

- **Double quotes for char**: `"a"` is `&str`. Use `'a'`.
- **`True` capitalization**: only lowercase `true` compiles.

## Best Practices

- Annotate at boundaries; let locals infer.
- Use underscores (`86_400`) and suffixes when width matters.

## Exercise

Name each type, then annotate and print:

```rust
fn main() {
    let a = 42;
    let b = 3.14;
    let c = 'z';
    let d = false;
}
```

## Solution

Integers default to `i32`, floats to `f64`:

```rust
fn main() {
    let a: i32 = 42;
    let b: f64 = 3.14;
    let c: char = 'z';
    let d: bool = false;
    println!("{a} {b} {c} {d}");
}
```

## Mini Quiz

```yaml
- question: "Which is a char literal?"
  options:
    - "\"a\""
    - "'a'"
    - "97"
    - "true"
  answer: 1
  explanation: "Single quotes build char; double quotes build &str."
- question: "What does 1_000 mean?"
  options:
    - "A float"
    - "1000; underscores aid reading"
    - "A string"
    - "An array"
  answer: 1
  explanation: "Underscores are ignored by the compiler."
- question: "Default integer type?"
  options:
    - "u8"
    - "i32"
    - "usize"
    - "Untyped"
  answer: 1
  explanation: "Unconstrained integers default to i32."
```
