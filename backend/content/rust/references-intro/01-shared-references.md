---
slug: shared-references
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: references-intro
chapterTitle: "Chapter 5 — References"
chapterOrder: 5
title: "Shared References"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, references, borrowing, shared]
---

## Learning Objective

By the end of this lesson, you will borrow with `&T` without moving
ownership.

## Explanation

A **reference** (a **borrow**) uses a value without taking
**ownership** (cleanup duty). A **shared reference** `&T` is read-only
and made with `&value`. `*r` (the **dereference operator**) reads the
target. Calls often auto-borrow, so `r.is_ascii()` works directly.
References are never null; absence uses `Option`. Rebinding (`r =
&b;`) re-points, never writes through.

## Why It Matters

Many readers can share configs and names without copies, grounding
later borrow-checker rules.

## Example

`r: &char` first views `A`, then re-points to `B`, printing each
target without moving either char.

## Code

```rust
fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("first: {r}");
    r = &b;
    println!("second: {r}");
}
```

## Code Explanation

- `&a` borrows `a`; `mut r` allows rebinding the pointer itself.
- Prints `first: A` then `second: B`; owners never move.

## Common Mistakes

- **`*r = 'X'`**: shared refs are read-only; need `&mut T` to write.
- **Expecting write-through**: `r = &b` rebinds, never edits `a`.

## Best Practices

- Borrow with `&T` params instead of cloning inputs.
- Prefer `&str` over `&String` for wider calls.

## Exercise

What prints? What does `*r = 'X'` report?

```rust
fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    r = &b;
    println!("r: {r}");
}
```

## Solution

Prints `r: B`:

```rust
fn main() {
    let a = 'A';
    let r: &char = &a;
    println!("via r: {r}");
}
```

`a` owns data; `r` only borrows. Writing fails as behind-`&`.

## Mini Quiz

```yaml
- question: "What is &T?"
  options:
    - "Owned copy"
    - "Read-only borrow"
    - "Null pointer"
    - "Growable buffer"
  answer: 1
  explanation: "&T views without ownership or mutation."
- question: "r = &b with r: &char does what?"
  options:
    - "Overwrites old target"
    - "Rebinds to b"
    - "Moves b"
    - "Deletes a"
  answer: 1
  explanation: "Assignment re-points the borrow."
- question: "Can safe refs be null?"
  options:
    - "Yes, check always"
    - "No, use Option"
    - "Only &mut"
    - "Only release"
  answer: 1
  explanation: "References are always valid."
```
