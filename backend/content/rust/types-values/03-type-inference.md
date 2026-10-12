---
slug: type-inference
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: types-values
chapterTitle: "Chapter 2 — Types and Values"
chapterOrder: 2
title: "Type Inference"
difficulty: beginner
order: 3
estimatedMinutes: 8
tags: [rust, types, inference]
---

## Learning Objective

By the end of this lesson, you will rely on inference and read
`{integer}` in error messages.

## Explanation

**Type inference** means the compiler deduces types from literals and
uses, so `let x = 10;` needs no annotation yet still has one fixed
**static type**. Constraints flow from calls: passing `x` to `fn f(v:
u32)` pins `x` to `u32`. Defaults apply with no constraints: integers
become `i32`, floats become `f64`. Errors show `{integer}` or
`{float}` for still-ambiguous literals. An **annotation** like `let y:
u32 = 20;` overrides inference when the compiler asks for help.

## Why It Matters

Inference keeps code short without dynamic typing: machine code
matches annotated code exactly, with safety intact.

## Example

Helpers taking `u32` and `i8` pin two plain `let` bindings to those
types. Swapping arguments then fails, proving types already fixed.

## Code

```rust
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    takes_u32(10);
    takes_i8(20);
}
```

## Code Explanation

- Parameter types (`: u32`, `: i8`) are mandatory and constrain calls.
- Literals `10` and `20` adapt to each signature at the call site.
- Passing an `i8` variable to `takes_u32` would fail statically.

## Common Mistakes

- **Treating inference as dynamic**: `x = "hi"` after `let x = 10`
  fails; one static type sticks.
- **Over-annotating locals**: annotate signatures, let locals infer.

## Best Practices

- Write full signatures; let bodies infer from them.
- Add the narrowest annotation at the creating `let` when asked.

## Exercise

Why does this fail? Give two fixes:

```rust
fn main() {
    let a = 3.14;
    let b = 20;
    assert_eq!(a, b);
}
```

## Solution

`a` is `f64`, `b` is `i32`; `==` needs one type:

```rust
fn main() {
    let a: f64 = 3.14;
    let b: f64 = 20.0;
    assert_eq!(a, b - 16.86);
}
```

Pinning both to `f64` lets the comparison compile.

## Mini Quiz

```yaml
- question: "What does let x = 10 create?"
  options:
    - "Dynamic any-type"
    - "One fixed inferred type"
    - "No type until run"
    - "A string"
  answer: 1
  explanation: "Inference fills one static type permanently."
- question: "Default integer literal type?"
  options:
    - "u8"
    - "i32"
    - "usize"
    - "f64"
  answer: 1
  explanation: "Integers default to i32; floats to f64."
- question: "{integer} in errors means?"
  options:
    - "A runtime value"
    - "An unconstrained literal"
    - "A format slot"
    - "An import"
  answer: 1
  explanation: "Compiler lacks constraints to pin the literal."
```
