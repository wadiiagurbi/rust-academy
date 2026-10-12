---
slug: borrow-errors
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: borrowing
chapterTitle: "Chapter 15 — Borrowing"
chapterOrder: 15
title: "Borrow Checker Errors"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, borrowing, errors, vec]
---

## Learning Objective

By the end of this lesson, you will read borrow errors as reallocation protection and restructure code to fix them.

## Explanation

A **reallocation** moves a `Vec`'s elements to a bigger buffer when it outgrows the old one; live references into it would **dangle** (point at freed memory). The checker refuses to grow a vector while an element borrow lives. **Iterator invalidation** is the same bug in a loop: pushing while iterating `&vec` could relocate every element mid-loop. Fixes: copy small values out, end the borrow before mutating, or mutate after the loop.

## Why It Matters

These are the first borrow fights most learners hit; read as "the compiler stopping a dangling pointer", they build trust.

## Example

Copying `vec[2]` (`i32` is `Copy`) ends the borrow at once, so the later `push` is legal and safe.

## Code

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let elem = vec[2];
    vec.push(6);
    println!("elem: {elem}, vec: {vec:?}");
}
```

## Code Explanation

- `let elem = vec[2];` copies the integer; no borrow survives, so `push` may safely reallocate afterwards.
- Broken variant (does not compile): `let elem = &vec[2];` before `vec.push(6);`
  fails with error[E0502]: cannot borrow `vec` as mutable because it is also borrowed as immutable.

## Common Mistakes

- **Holding `&vec[i]` across `push`**: copy the element or index first, then mutate.
- **Pushing inside `for x in &vec`**: the iteration borrow forbids it — push into a second buffer instead.

## Best Practices

- Copy small elements instead of holding references across mutations; scope borrows tightly, then mutate.

## Exercise

Fix this to print element and grown vector:

```rust
// DOES-NOT-COMPILE
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let elem = &vec[2];
    vec.push(6);
    println!("{elem}");
}
```

## Solution

Copy before mutating so no borrow spans the `push`:

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let elem = vec[2];
    vec.push(6);
    println!("elem: {elem}, vec: {vec:?}");
}
```

`elem: i32` owns its bytes; reallocation never touches the copy.

## Mini Quiz

```yaml
- question: "Why is push refused while &vec[2] lives?"
  options:
    - "Push is always refused"
    - "Push may reallocate, dangling the reference"
    - "The vector is immutable"
    - "Integers resist borrows"
  answer: 1
  explanation: "Reallocation frees the buffer being pointed into."
- question: "What is iterator invalidation?"
  options:
    - "A slow loop"
    - "Mutating a collection mid-iteration, risking moved elements"
    - "A missing import"
    - "Off-by-one"
  answer: 1
  explanation: "Pushing mid-loop can relocate what is read."
- question: "Idiomatic fix for small elements?"
  options:
    - "Raw pointers"
    - "Copy the value out before mutating"
    - "Wrap in Rc"
    - "Skip checks"
  answer: 1
  explanation: "A Copy ends the borrow at once."
```
