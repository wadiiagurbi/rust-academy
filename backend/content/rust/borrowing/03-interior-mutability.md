---
slug: interior-mutability
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: borrowing
chapterTitle: "Chapter 15 — Borrowing"
chapterOrder: 15
title: "Interior Mutability"
difficulty: intermediate
order: 3
estimatedMinutes: 12
tags: [rust, borrowing, cell, refcell]
---

## Learning Objective

By the end of this lesson, you will mutate through shared `&` with `Cell` and `RefCell`, naming each tool's safety check.

## Explanation

**Interior mutability** mutates through `&self` — the exception to "shared means read-only". **`Cell<T>`** offers `set`/`get` via shared reference with no runtime check; it stays safe by never handing out references (`get` copies, so `T: Copy` is required and `Cell<String>` has no `.get()`). **`RefCell<T>`** allows non-`Copy` data with a runtime tally: `borrow()` yields a `Ref`, `borrow_mut()` a `RefMut`, enforcing "many shared xor one exclusive" while running. Violations **panic** (a controlled crash) instead of corrupting memory.

## Why It Matters

Caches and mocks need mutation through shared handles; these types keep that sound, checked at compile time (`Cell`) or runtime (`RefCell`).

## Example

A non-`mut` `Cell` still updates: `set` takes `&self`, `get` copies out with no borrow left alive.

## Code

```rust
use std::cell::Cell;
fn main() {
    let cell = Cell::new(5);
    cell.set(123);
    println!("cell: {}", cell.get());
}
```

## Code Explanation

- `cell` is not `mut`, yet `set` mutates through the shared handle, and `get` copies the `i32` out with no aliasing reference left.
- Broken variant (does not compile): `.get()` on `Cell<String>` fails since `String` is not `Copy` —
  error[E0599]: the method get exists, but its trait bounds were not satisfied (String is not Copy).

## Common Mistakes

- **`.get()` on `Cell<String>`**: use `replace`, `take`, or switch to `RefCell`.
- **Overlapping `borrow`/`borrow_mut`**: the tally panics — scope each guard so it drops first.

## Best Practices

- Prefer `Cell` for `Copy` values (zero cost, no panics); keep `RefCell` guards in tight blocks.

## Exercise

What prints? What if the marked lines are restored?

```rust
use std::cell::RefCell;
fn main() {
    let cell = RefCell::new(5);
    {
        let mut r = cell.borrow_mut();
        *r = 123;
        // let other = cell.borrow();
        // println!("{other}");
    }
    println!("{cell:?}");
}
```

## Solution

`RefCell { value: 123 }`:

```rust
use std::cell::RefCell;
fn main() {
    let cell = RefCell::new(5);
    {
        let mut r = cell.borrow_mut();
        *r = 123;
    }
    assert_eq!(*cell.borrow(), 123);
    println!("{cell:?}");
}
```

The guard drops at block end, so the later `borrow()` succeeds; restoring the lines panics (`already mutably borrowed`).

## Mini Quiz

```yaml
- question: "What is interior mutability?"
  options:
    - "Mutating a mut binding"
    - "Mutating through shared & via Cell/RefCell"
    - "Unsafe bypassing"
    - "Clone before write"
  answer: 1
  explanation: "These types allow &self mutation with safety."
- question: "How is Cell safe without checks?"
  options:
    - "It never hands out references; get copies Copy values"
    - "It locks a mutex"
    - "It collects garbage"
    - "It forbids writes"
  answer: 0
  explanation: "No live reference means no aliasing violation."
- question: "Overlapping RefCell borrows cause?"
  options:
    - "Corruption"
    - "Compile error"
    - "Runtime panic"
    - "Nothing"
  answer: 2
  explanation: "The dynamic tally panics on violation."
```
