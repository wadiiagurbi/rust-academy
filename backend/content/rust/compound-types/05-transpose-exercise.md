---
slug: transpose
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: compound-types
chapterTitle: "Chapter 4 — Compound Types"
chapterOrder: 4
title: "Exercise: Transpose"
difficulty: beginner
order: 5
estimatedMinutes: 15
tags: [rust, exercise, arrays, matrix]
---

## Learning Objective

By the end of this lesson, you will transpose a 3x3 matrix in Cargo
and check rows became columns.

## Explanation

A **matrix** is `[[i32; 3]; 3]`: three rows of three `i32`. Outer `3`
counts rows; inner `[i32; 3]` types one row. **Transpose** maps
`result[j][i] = matrix[i][j]`. A **Cargo task** is `cargo new`,
editing `src/main.rs`, proving with `cargo run`. `todo!()` stubs
panic until replaced. `i32` arrays are **`Copy`** (assignment
duplicates), so by-value passing copies cheaply.

## Why It Matters

Grid flips model images and boards. Fresh-buffer index loops give a
safe template for rotations without aliasing.

## Example

`[[101,102,103],...]` transposes top row to `[101,201,301]`, shown
by printing rows with `{row:?}`.

## Code

```rust
fn transpose(m: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut r = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            r[j][i] = m[i][j];
        }
    }
    r
}

fn main() {
    println!("{:?}", transpose([[1, 2, 3], [4, 5, 6], [7, 8, 9]]));
}
```

## Code Explanation

- `[[0; 3]; 3]` starts a defined zero buffer (no uninit memory).
- Swapped `[j][i]` turns rows to columns; bare `r` returns it.

## Common Mistakes

- **`r[i][j] = m[i][j]`**: copies instead of transposing.
- **Leaving `todo!()`**: panics before any assert runs.

## Best Practices

- Init outputs with zeros so every cell is defined.
- Assert one cell plus full equality for fast diagnosis.

## Exercise

Implement the stub so the assert passes:

```bash
cargo new transpose_exercise
cd transpose_exercise
cargo run
```

```rust
fn transpose(m: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    todo!("transpose")
}

fn main() {
    let t = transpose([[101, 102, 103], [201, 202, 203], [301, 302, 303]]);
    assert_eq!(t, [[101, 201, 301], [102, 202, 302], [103, 203, 303]]);
    println!("{t:?}");
}
```

## Solution

Swap indexes into a fresh buffer:

```rust
fn transpose(m: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut r = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            r[j][i] = m[i][j];
        }
    }
    r
}

fn main() {
    let t = transpose([[101, 102, 103], [201, 202, 203], [301, 302, 303]]);
    assert_eq!(t, [[101, 201, 301], [102, 202, 302], [103, 203, 303]]);
    println!("{t:?}");
}
```

## Mini Quiz

```yaml
- question: "What is [[i32; 3]; 3]?"
  options:
    - "Three rows of three i32"
    - "Growable vector"
    - "Tuple strings"
    - "Nine vars"
  answer: 0
  explanation: "Outer holds three inner arrays."
- question: "Which assignment transposes?"
  options:
    - "r[i][j] = m[i][j]"
    - "r[j][i] = m[i][j]"
    - "r[i][j] = m[j][i] + 1"
    - "r = m"
  answer: 1
  explanation: "Swapped indexes flip rows/cols."
- question: "Why start from zeros?"
  options:
    - "Faster"
    - "Rust needs defined memory"
    - "Publishes crate"
    - "It is the answer"
  answer: 1
  explanation: "Safe code has no uninit memory."
```
