---
slug: geometry
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: references-intro
chapterTitle: "Chapter 5 — References"
chapterOrder: 5
title: "Exercise: Geometry"
difficulty: beginner
order: 5
estimatedMinutes: 15
tags: [rust, exercise, references, slices]
---

## Learning Objective

By the end of this lesson, you will read via `&` and write via
`&mut` on 3D vectors in Cargo.

## Explanation

A **vector** is `[f64; 3]` for x, y, z. **Magnitude** is
`sqrt(x^2+y^2+z^2)` via `sqrt()`. **Normalize** divides each part by
magnitude for length 1. Shared `&[f64; 3]` reads; mutable `&mut [f64;
3]` writes with `*item /= mag;`. A **Cargo task** is `cargo new`,
editing `src/main.rs`, proving with `cargo run`. `todo!()` stubs
panic until replaced.

## Why It Matters

Read-then-calibrate models sensor pipelines and makes ownership
click through signatures.

## Example

`[0,1,0]` has length 1; `[1,2,9]` normalizes to length 1 keeping
direction.

## Code

```rust
fn magnitude(v: &[f64; 3]) -> f64 {
    let mut s = 0.0;
    for c in v {
        s += c * c;
    }
    s.sqrt()
}

fn main() {
    println!("mag: {}", magnitude(&[0.0, 1.0, 0.0]));
}
```

## Code Explanation

- `&[f64; 3]` borrows; callers keep ownership with `&v`.
- Squares sum, then `sqrt()` returns length (`1` here).

## Common Mistakes

- **By-value `[f64; 3]`**: copies each call; borrow instead.
- **Missing `*` on write**: use `*item /= mag` through `&mut`.

## Best Practices

- Reuse `magnitude` inside `normalize` as one truth source.
- Compare floats with epsilon, not exact `==`.

## Exercise

Fill stubs so `cargo run` passes:

```bash
cargo new geometry_exercise
cd geometry_exercise
cargo run
```

```rust
fn magnitude(v: &[f64; 3]) -> f64 {
    todo!("magnitude")
}

fn normalize(v: &mut [f64; 3]) {
    todo!("normalize")
}

fn main() {
    let mut v = [1.0, 2.0, 9.0];
    normalize(&mut v);
    println!("{} {v:?}", magnitude(&v));
}
```

Criteria: unit magnitude is `1.0`; normalized within `1e-6` of 1.

## Solution

Sum squares, root, divide each:

```rust
fn magnitude(v: &[f64; 3]) -> f64 {
    let mut s = 0.0;
    for c in v {
        s += c * c;
    }
    s.sqrt()
}

fn normalize(v: &mut [f64; 3]) {
    let m = magnitude(v);
    for item in v {
        *item /= m;
    }
}

fn main() {
    let mut v = [1.0, 2.0, 9.0];
    normalize(&mut v);
    println!("{} {v:?}", magnitude(&v));
}
```

## Mini Quiz

```yaml
- question: "Read-only vector signature?"
  options:
    - "v: [f64; 3]"
    - "v: &[f64; 3] -> f64"
    - "-> String"
    - "v: &mut String"
  answer: 1
  explanation: "Shared borrows inspect without move."
- question: "Write via item: &mut f64?"
  options:
    - "item /= m"
    - "*item /= m"
    - "item = m"
    - "Cannot write"
  answer: 1
  explanation: "Deref assigns through borrow."
- question: "Normalize guarantees?"
  options:
    - "Length 1, same direction"
    - "All zeros"
    - "Sorted vector"
    - "Moves ownership"
  answer: 0
  explanation: "Scaling by magnitude keeps direction."
```
