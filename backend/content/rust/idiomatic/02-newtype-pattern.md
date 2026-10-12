---
slug: newtype-pattern
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: idiomatic
chapterTitle: "Chapter 23 — Idiomatic Rust"
chapterOrder: 23
title: "The Newtype Pattern"
difficulty: intermediate
order: 2
estimatedMinutes: 10
tags: [rust, newtype, types]
---

## Learning Objective

By the end of this lesson, you will wrap a primitive in a **newtype** (a single-field struct giving a distinct type) to stop unit mix-ups at compile time.

## Explanation

A **newtype** is `struct Name(Inner);` — one field, zero runtime cost, but a brand-new type. `Meters(3.0)` and `Seconds(3.0)` are both `f64` (a 64-bit float) inside, yet cannot be added to each other. Construction goes through an associated function (called as `Type::name()`) that validates and returns `Result`. The field stays private, so every value in the program has passed the same gate.

## Why It Matters

Raw numbers let seconds mix with metres and compile fine, then fail in production. Newtypes move that check to the compiler for free.

## Example

Distance in metres: construction rejects negatives, and only metres add to metres.

## Code

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Meters(f64);
impl Meters {
    fn new(v: f64) -> Result<Self, String> {
        if v < 0.0 { return Err(format!("negative: {v}")); }
        Ok(Self(v))
    }
    fn add(self, o: Self) -> Self { Self(self.0 + o.0) }
}
fn main() {
    let total = Meters::new(3.0).expect("v").add(Meters::new(4.0).expect("v"));
    println!("{} {:?}", total.0, Meters::new(-1.0));
}
```

## Code Explanation

- `struct Meters(f64);` is a **tuple struct**: positional fields read as `self.0`, private outside the module.
- `Copy` is safe here since `f64` is `Copy`. Prints `7 Err("negative: -1")`.

## Common Mistakes

- **Public inner field**: `pub struct Meters(pub f64)` lets anyone skip validation. Keep it private.
- **Bypassing the gate**: constructing `Meters(x)` directly inside the module skips checks. Route all creation through `new()`.
- **Wrapping everything**: one-field wrappers suit truly distinct concepts only; wrapping every `bool` adds noise.

## Best Practices

- Validate once in `new()`; expose only safe operations (`add`, field reads) afterwards.
- Derive `Debug`, `Clone`, `Copy`, `PartialEq` when the inner type allows it.

## Exercise

Add `Seconds(f64)` with the same rules, plus `fn speed(d: Meters, t: Seconds) -> f64`. Print the speed of 100 m in 9.58 s.

## Solution

```rust
#[derive(Clone, Copy)]
struct Meters(f64);
#[derive(Clone, Copy)]
struct Seconds(f64);
impl Meters {
    fn new(v: f64) -> Result<Self, String> {
        if v < 0.0 { return Err(format!("negative: {v}")); }
        Ok(Self(v))
    }
}
impl Seconds {
    fn new(v: f64) -> Result<Self, String> {
        if v <= 0.0 { return Err(format!("non-positive: {v}")); }
        Ok(Self(v))
    }
}
fn speed(d: Meters, t: Seconds) -> f64 { d.0 / t.0 }
fn main() {
    let s = speed(Meters::new(100.0).expect("d"), Seconds::new(9.58).expect("t"));
    println!("{s:.2} m/s");
}
```

Prints `10.44 m/s`: swapped units cannot even be passed in the wrong order.

## Mini Quiz

```yaml
- question: "What does the newtype pattern buy you?"
  options:
    - "Faster floats"
    - "Distinct compile-time types over one inner type, zero cost"
    - "Automatic parsing"
    - "Heap allocation"
  answer: 1
  explanation: "Same layout as the inner value, but the compiler rejects mixing."
- question: "Why keep the inner field private?"
  options:
    - "Privacy is mandatory syntax"
    - "So every value passes the validating constructor"
    - "To save memory"
    - "To allow mutation"
  answer: 1
  explanation: "A private field forces all creation through the checked gate."
- question: "What is a tuple struct?"
  options:
    - "A struct with named fields"
    - "A struct with positional fields like Meters(f64)"
    - "A struct with no fields"
    - "An enum variant"
  answer: 1
  explanation: "Fields are accessed by index; ideal shape for newtypes."
```
