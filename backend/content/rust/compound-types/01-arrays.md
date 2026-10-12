---
slug: arrays
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: compound-types
chapterTitle: "Chapter 4 — Compound Types"
chapterOrder: 4
title: "Arrays"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, arrays, types, indexing]
---

## Learning Objective

By the end of this lesson, you will declare arrays and index them
with bounds checking.

## Explanation

An **array** is `[T; N]`: `N` values of one type `T` with fixed
length on the **stack**. `[u8; 3]` differs from `[u8; 4]` since length
is part of the type. **Indexing** (`a[2]`) starts at 0. Print with the
**debug format** `{a:?}`. `[0; 1024]` repeats zero 1024 times.
Constant out-of-range fails at compile time; runtime violations
**panic** (halt with an error).

## Why It Matters

Arrays fit pixels and board rows with known size, motivating why
vectors and slices exist for the rest.

## Example

Scores `[5, 4, 3, 2, 1]` patch index 2 to `0` and print with `{:?}`.
Bad indexes are rejected, blocking C-style overruns.

## Code

```rust
fn main() {
    let mut scores: [i8; 5] = [5, 4, 3, 2, 1];
    scores[2] = 0;
    println!("scores: {scores:?}");
}
```

## Code Explanation

- `mut` allows element writes; length stays 5.
- `scores[2] = 0` writes the third slot; output is `[5, 4, 0, 2, 1]`.

## Common Mistakes

- **Past-the-end index**: literal fails to compile; computed panics.
- **Expecting push**: length is fixed; use `Vec` to grow.

## Best Practices

- Annotate `[T; N]` so mismatches surface early.
- Prefer `for` iteration over manual indexes.

## Exercise

What prints? Hold four `7`s with repeat syntax:

```rust
fn main() {
    let data: [u8; 3] = [10, 20, 30];
    println!("{data:?}");
}
```

## Solution

Prints `[10, 20, 30]`:

```rust
fn main() {
    let data: [u8; 4] = [7; 4];
    println!("{data:?}");
}
```

Prints `[7, 7, 7, 7]`. Index 9 on length 3 fails or panics.

## Mini Quiz

```yaml
- question: "What is [i8; 5]?"
  options:
    - "Five i8 with fixed 5"
    - "Growable list"
    - "Heap bytes"
    - "A string"
  answer: 0
  explanation: "Length is part of the array type."
- question: "How to print arrays?"
  options:
    - "With {}"
    - "With {:?}"
    - "Cannot print"
    - "With cargo fmt"
  answer: 1
  explanation: "Arrays implement Debug only."
- question: "Out-of-bounds access?"
  options:
    - "Returns zero"
    - "Compile error or panic"
    - "Grows array"
    - "Wraps to 0"
  answer: 1
  explanation: "Rust checks every bound."
```
