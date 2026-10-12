---
slug: newtype
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: user-types
chapterTitle: "Chapter 6 — User-Defined Types"
chapterOrder: 6
title: "Tuple Structs and Newtype"
difficulty: beginner
order: 2
estimatedMinutes: 8
tags: [rust, structs, newtype, types]
---

## Learning Objective

By the end of this lesson, you will wrap primitives to separate units
by type.

## Explanation

A **tuple struct** has positional fields (`struct Point(i32, i32);`)
read as `p.0`, built as `Point(17, 23)`. A **newtype** wraps one value
(`struct Newtons(f64);`) giving `f64` a distinct identity, so mixing
`Newtons` with `PoundsOfForce` fails to compile. Rust has no
**implicit conversion**, so `.0` unwraps explicitly. Zero-field forms
are **zero-sized** markers.

## Why It Matters

Newtypes turn unit mix-ups into compile errors instead of hardware
failures.

## Example

Thrusters want `Newtons` but sensors give `PoundsOfForce`; only an
explicit converter bridges them.

## Code

```rust
struct Point(i32, i32);
struct Newtons(f64);

fn main() {
    let p = Point(17, 23);
    println!("{} {} {}", p.0, p.1, Newtons(450.5).0);
}
```

## Code Explanation

- Names make types distinct from raw tuples and `f64`.
- `.0` reads wrapped values deliberately, never automatically.

## Common Mistakes

- **Passing raw `450.5`**: wrap as `Newtons(450.5)` first.
- **Expecting `+` across newtypes**: unwrap to `.0` or impl ops.

## Best Practices

- Newtype primitives with units or validation (`Meters`, `UserId`).
- Keep fields private so `new()` enforces checks.

## Exercise

Convert pounds to newtons (1 lbf = 4.44822 N):

```rust
struct PoundsOfForce(f64);
struct Newtons(f64);

fn main() {
    let f = PoundsOfForce(100.0);
    println!("{}", f.0);
}
```

## Solution

Unwrap, scale, re-wrap:

```rust
struct PoundsOfForce(f64);
struct Newtons(f64);

fn to_newtons(p: PoundsOfForce) -> Newtons {
    Newtons(p.0 * 4.44822)
}

fn main() {
    println!("{}", to_newtons(PoundsOfForce(100.0)).0);
}
```

Prints `444.822`.

## Mini Quiz

```yaml
- question: "What is a newtype?"
  options:
    - "Type alias"
    - "One-field wrapper with new identity"
    - "Vector"
    - "Empty fn"
  answer: 1
  explanation: "Wrappers add checking to primitives."
- question: "Read inner value how?"
  options:
    - "With .0"
    - "With [0]"
    - "Auto unwrap"
    - "cargo run"
  answer: 0
  explanation: "Fields are positional."
- question: "Why reject mixed units?"
  options:
    - "Annoyance"
    - "Prevent unit bugs at compile time"
    - "No floats"
    - "No structs"
  answer: 1
  explanation: "Types catch orbiter-class errors."
```
