---
slug: box-pointer
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: smart-pointers
chapterTitle: "Chapter 14 — Smart Pointers"
chapterOrder: 14
title: "Box and Heap Ownership"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, smart-pointers, box, heap]
---

## Learning Objective

By the end of this lesson, you will store data on the heap with `Box<T>` and build recursive types with it.

## Explanation

A **`Box<T>`** is an owned pointer to heap data: a fixed-size stack handle plus heap storage freed when the box drops. A **smart pointer** acts like a pointer with guarantees — `Box` guarantees sole ownership and is never null. **Dereferencing** (`*five`) follows the pointer to the value. **Recursion** (a type containing itself) needs a box: a variant holding the enum directly would be infinitely sized ("recursive without indirection"); a box is pointer-sized, ending the cycle.

## Why It Matters

`Box` is the simplest heap escape hatch: dynamic sizes, cheap moves of big data, trees and lists.

## Example

`Box::new(5)` heap-allocates one integer; a `List` links nodes with `Box<List>`, each hop on the heap, each handle pointer-sized.

## Code

```rust
#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    Nil,
}
fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
    let list = List::Node(1, Box::new(List::Node(2, Box::new(List::Nil))));
    println!("{list:?}");
}
```

## Code Explanation

- `Box::new(5)` moves the integer to the heap; `*five` reads it back; `Node(i32, Box<List>)` keeps the enum fixed-size via indirection.

## Common Mistakes

- **Skipping the `Box`**: rejected with infinite-size errors; keep the indirection on every recursive link.
- **Expecting null**: `Box` always points somewhere; use `Option<Box<T>>` for absence.

## Best Practices

- Box unknown-at-compile-time sizes or large moved values; prefer `Box` while single ownership fits.

## Exercise

What prints? Check with `rustc`:

```rust
#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    Nil,
}
fn main() {
    let l = List::Node(1, Box::new(List::Nil));
    println!("{l:?}");
}
```

## Solution

`Node(1, Nil)`:

```rust
#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    Nil,
}
fn main() {
    let l = List::Node(1, Box::new(List::Nil));
    assert!(matches!(l, List::Node(1, _)));
    println!("{l:?}");
}
```

`matches!` (a pattern-testing macro) confirms head `1`; the tail ends the chain.

## Mini Quiz

```yaml
- question: "What is Box<T>?"
  options:
    - "A shared reference"
    - "An owned, non-null heap pointer"
    - "A collected handle"
    - "A raw pointer"
  answer: 1
  explanation: "Box owns its heap value and frees it on drop."
- question: "Why do recursive types need Box?"
  options:
    - "Speed only"
    - "Pointer indirection gives a fixed size"
    - "Null values"
    - "Mutation"
  answer: 1
  explanation: "Direct self-containment would be infinitely sized."
- question: "What does *five do for five: Box<i32>?"
  options:
    - "Clones the box"
    - "Dereferences to the heap integer"
    - "Drops it"
    - "Moves it"
  answer: 1
  explanation: "* follows the pointer to the heap value."
```
