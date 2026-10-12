---
slug: slices
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: references-intro
chapterTitle: "Chapter 5 — References"
chapterOrder: 5
title: "Slices"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, slices, arrays, borrowing]
---

## Learning Objective

By the end of this lesson, you will slice arrays and take `&[T]`
over any length.

## Explanation

A **slice** `&[T]` is a borrowed view: pointer plus length, without
naming host size. **Ranges** pick spans: `&a[2..4]` takes indexes 2–3,
`&a[..]` takes all, `&a[2..]` and `&a[..4]` drop one bound. Slices own
nothing, so they cannot `push` or grow past the host. The host must
outlive its views.

## Why It Matters

One `&[T]` signature sums and searches any length without copies or
overloads.

## Example

`[10..60]` array views `[30, 40]` via `&a[2..4]`; a `sum(&[i32])`
helper totals full and partial views alike.

## Code

```rust
fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for v in values {
        total += *v;
    }
    total
}

fn main() {
    let a = [10, 20, 30, 40, 50, 60];
    println!("{:?}", &a[2..4]);
    println!("sum: {}", sum(&a[..]));
}
```

## Code Explanation

- `for v in values` yields `&i32`, so `*v` adds through deref.
- `&a[2..4]` is `[30, 40]`; full sum is `210`.

## Common Mistakes

- **Slice past host life**: hosts must outlive views; return `Vec`.
- **`push` on slice**: borrows cannot grow.

## Best Practices

- Take `&[T]` not `&[T; N]` to serve every length.
- Iterate slices directly to elide bounds checks.

## Exercise

What prints? Add `first(&[i32])`:

```rust
fn main() {
    let a = [10, 20, 30, 40];
    println!("{:?}", &a[1..3]);
}
```

## Solution

Prints `[20, 30]`:

```rust
fn first(values: &[i32]) -> i32 {
    values[0]
}

fn main() {
    let a = [10, 20, 30, 40];
    println!("first: {}", first(&a[1..3]));
}
```

Prints `first: 20` for any length input.

## Mini Quiz

```yaml
- question: "What is &[i32]?"
  options:
    - "Owned vector"
    - "Borrowed pointer plus length"
    - "Fixed array"
    - "String slice"
  answer: 1
  explanation: "Slices view hosts of any length."
- question: "&a[2..4] holds?"
  options:
    - "Indexes 2,3,4"
    - "Indexes 2,3 end excluded"
    - "Whole array"
    - "Empty"
  answer: 1
  explanation: "Range ends exclude the last."
- question: "Can slice outlive host?"
  options:
    - "Yes always"
    - "No, host outlives view"
    - "Only release"
    - "Only empty"
  answer: 1
  explanation: "Borrow rules enforce lifetimes."
```
