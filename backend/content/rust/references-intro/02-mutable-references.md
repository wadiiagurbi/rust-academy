---
slug: mutable-references
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: references-intro
chapterTitle: "Chapter 5 — References"
chapterOrder: 5
title: "Mutable References"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, references, mutable, borrowing]
---

## Learning Objective

By the end of this lesson, you will mutate through `&mut T` under
the exclusivity rule.

## Explanation

An **exclusive reference** (`&mut T`, also mutable reference) writes
through a borrow via `&mut value` (owner must be `mut`). Writes use
`*r = v;`. **Exclusivity** means while `&mut` lives, no other ref to
that value exists and the owner pauses. Note positions: `let mut r:
&i32` rebinds a shared ref; `let r: &mut i32` mutates the target.

## Why It Matters

In-place counters and buffers need one writer. Exclusivity catches
aliasing before it becomes threaded Heisenbugs.

## Example

`&mut point.0` changes `(1, 2)` to `(20, 2)`. Reading `point` mid-borrow
is rejected, proving the rule.

## Code

```rust
fn main() {
    let mut point = (1, 2);
    {
        let x = &mut point.0;
        *x = 20;
    }
    println!("{point:?}");
}
```

## Code Explanation

- `&mut point.0` borrows field 0 exclusively as `&mut i32`.
- `*x = 20` writes through; block ends borrow, printing `(20, 2)`.

## Common Mistakes

- **`&mut` from immutable owner**: mark owner `mut` first.
- **Using owner mid-borrow**: shorten borrows with blocks.

## Best Practices

- Scope `&mut` in the smallest block needed.
- Take `&mut` only when mutation is required; else take `&T`.

## Exercise

Fix the exclusivity error to print `(20, 2)`:

```rust
fn main() {
    let mut point = (1, 2);
    let x = &mut point.0;
    println!("{point:?}");
    *x = 20;
}
```

## Solution

End the borrow before reading owner:

```rust
fn main() {
    let mut point = (1, 2);
    {
        let x = &mut point.0;
        *x = 20;
    }
    println!("{point:?}");
}
```

## Mini Quiz

```yaml
- question: "Which type mutates through borrow?"
  options:
    - "&T"
    - "&mut T"
    - "T by copy"
    - "None"
  answer: 1
  explanation: "&mut grants write access exclusively."
- question: "Exclusivity forbids?"
  options:
    - "Other uses while &mut lives"
    - "All future uses"
    - "Reads after borrow"
    - "Nothing"
  answer: 0
  explanation: "One live &mut excludes peers."
- question: "mut r: &i32 vs r: &mut i32?"
  options:
    - "Identical"
    - "Rebind shared vs mutate target"
    - "First mutates"
    - "Null pointers"
  answer: 1
  explanation: "mut position picks binding or target."
```
