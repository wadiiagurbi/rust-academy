---
slug: iterator-chaining
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: iterators
chapterTitle: "Chapter 17 — Iterators"
chapterOrder: 17
title: "Exercise: Chaining Iterators"
difficulty: beginner
order: 4
estimatedMinutes: 15
tags: [rust, iterators, exercise]
---

## Learning Objective

By the end of this lesson, you will build a tested iterator pipeline in a cargo project with `cargo test`.

## Explanation

This is a **cargo task**: `cargo new chain_lab`, paste the stub into `src/main.rs`, replace `todo!()` (a macro that compiles but panics with `not yet implemented` if executed). Run `cargo test` (builds the crate plus its `#[test]` functions — test cases that pass when they return without panicking). Goal: keep readings `>= 10`, double each, `collect()` into `Vec<i32>`. `process` takes a **slice** (`&[i32]`, a borrowed view of consecutive elements) so callers lend data without moving it.

## Why It Matters

Chaining small adapters into a tested pipeline is the everyday shape of Rust data processing.

## Example

Test criteria — aim for this `cargo test` output:

```text
test keeps_and_doubles_large_values ... ok
test drops_everything_below_threshold ... ok
```

## Code

```rust
fn process(values: &[i32]) -> Vec<i32> {
    todo!()
}

fn main() {
    println!("{:?}", process(&[3, 8, 12, 5, 20, 7]));
}

#[test]
fn keeps_and_doubles_large_values() {
    assert_eq!(process(&[3, 8, 12, 5, 20, 7]), vec![24, 40]);
}

#[test]
fn drops_everything_below_threshold() {
    assert_eq!(process(&[1, 2, 3]), Vec::<i32>::new());
}
```

## Code Explanation

- `todo!()` compiles; both `#[test]`s panic until the logic lands.
- `assert_eq!` (a macro panicking with a diff on mismatch) encodes each criterion.

## Common Mistakes

- **Mapping before filtering**: doubling first changes what passes `>= 10`. Filter raw readings first.
- **Comparing `&i32` with `i32`**: destructure (`|&&v|`) or add `.copied()` before comparing.
- **Ending with `map`**: that returns an iterator, not `Vec<i32>` — terminate with `collect()`.

## Best Practices

- Order: borrow (`iter`), filter, transform (`map`), then `collect()`.
- Keep `process` pure (no printing inside) so tests stay deterministic.

## Exercise

Replace `todo!()` with the chain. Pass criteria: both tests green, `cargo run` prints `[24, 40]`.

## Solution

```rust
fn process(values: &[i32]) -> Vec<i32> {
    values.iter().filter(|&&v| v >= 10).map(|&v| v * 2).collect()
}

fn main() {
    println!("{:?}", process(&[3, 8, 12, 5, 20, 7]));
}

#[test]
fn keeps_and_doubles_large_values() {
    assert_eq!(process(&[3, 8, 12, 5, 20, 7]), vec![24, 40]);
}

#[test]
fn drops_everything_below_threshold() {
    assert_eq!(process(&[1, 2, 3]), Vec::<i32>::new());
}
```

## Mini Quiz

```yaml
- question: "What does `todo!()` do in a stub?"
  options:
    - "Skips compiling the function"
    - "Compiles but panics with `not yet implemented` if executed"
    - "Generates the solution"
    - "Ignores the test"
  answer: 1
  explanation: "todo!() keeps the build green while failing loudly at runtime."
- question: "Why take `&[i32]` instead of `Vec<i32>`?"
  options:
    - "Slices allocate faster"
    - "Borrowing lets callers lend data without moving it"
    - "Vec cannot be a parameter"
    - "They are identical"
  answer: 1
  explanation: "A slice accepts vectors, arrays, and slices by borrow."
- question: "Correct pipeline order?"
  options:
    - "map, filter, collect"
    - "filter raw values, then map, then collect"
    - "collect, filter, map"
    - "Order is irrelevant"
  answer: 1
  explanation: "Filtering first preserves the threshold; mapping changes values."
```
