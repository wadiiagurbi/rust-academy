---
slug: methods
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: methods-traits
chapterTitle: "Chapter 8 — Methods and Traits"
chapterOrder: 8
title: "Methods"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, methods, self, impl]
---

## Learning Objective

By the end of this lesson, you will define methods with `impl` and pick the right receiver.

## Explanation

A **method** is a function attached to a type, called as `rect.area()`. An **`impl` block** holds a type's methods. The **receiver** states access: `&self` borrows read-only, `&mut self` borrows mutably, `self` takes ownership. An **associated function** like `Rect::new` has no receiver and is called on the type itself.

## Why It Matters

Receivers document intent: readers see at a glance whether a call reads, changes, or consumes the value.

## Example

A rectangle computes area by borrow, stretches by mutable borrow, and builds new values through `new`.

## Code

```rust
struct Rect { w: f64, h: f64 }

impl Rect {
    fn new(w: f64, h: f64) -> Self { Self { w, h } }
    fn area(&self) -> f64 { self.w * self.h }
    fn stretch(&mut self, f: f64) { self.w *= f; self.h *= f; }
}

fn main() {
    let mut b = Rect::new(3.0, 4.0);
    println!("{:.1}", b.area());
    b.stretch(2.0);
    println!("{:.1}", b.area());
}
```

## Code Explanation

- `Self` means the impl target (`Rect`); `new` is the constructor idiom.
- `area(&self)` reads; `stretch(&mut self)` mutates both fields.
- Output is `12.0` then `48.0` after doubling each side.

## Common Mistakes

- **Calling `&mut self` on immutable binding**: needs `let mut`. Fix: declare the variable mutable.
- **Taking `self` for readers**: moves the value needlessly. Fix: default readers to `&self`.

## Best Practices

- Use `&self` for readers, `&mut self` for mutators, `self` for converters.

## Exercise

Using the Code section's 3-by-4 `Rect`, predict the two numbers, then add `fn is_square(&self) -> bool` (true when `w == h`) and print it for `b`.

## Solution

Area is `12.0`; the check compares fields:

```rust
struct Rect { w: f64, h: f64 }

impl Rect {
    fn new(w: f64, h: f64) -> Self {
        Self { w, h }
    }
    fn is_square(&self) -> bool {
        self.w == self.h
    }
}

fn main() {
    println!("{}", Rect::new(3.0, 4.0).is_square());
}
```

## Mini Quiz

```yaml
- question: "Which receiver borrows read-only?"
  options:
    - "`&self`"
    - "`&mut self`"
    - "`self`"
    - "No receiver"
  answer: 0
  explanation: "`&self` borrows immutably; use it for readers."
- question: "What is `Rect::new`?"
  options:
    - "An associated function called on the type"
    - "A mutating method"
    - "A free function elsewhere"
    - "A macro"
  answer: 0
  explanation: "No `self` means it is called as `Type::name()`."
- question: "When should a method take `self`?"
  options:
    - "When it consumes or converts the value"
    - "For every reader"
    - "Never"
    - "Only for printing"
  answer: 0
  explanation: "By-value moves ownership, fitting builders."
```
