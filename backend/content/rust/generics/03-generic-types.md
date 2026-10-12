---
slug: generic-types
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: generics
chapterTitle: "Chapter 9 — Generics"
chapterOrder: 9
title: "Generic Types"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, generics, structs, enums]
---

## Learning Objective

By the end of this lesson, you will define generic structs and implement methods for them.

## Explanation

A **generic struct** like `struct Cage<T> { occupant: T }` stores a placeholder type picked per use. Methods need `impl<T> Cage<T>`, introducing the parameter before describing behavior for every `T`. Each concrete choice gets its own compiled version.

## Why It Matters

One container serves all cargo: parrot cages and number cages share code with separate, strict types.

## Example

A shelter tracks cages holding names or counts through the same struct and accessor.

## Code

```rust
struct Cage<T> { occupant: T }

impl<T> Cage<T> {
    fn occupant(&self) -> &T { &self.occupant }
}

fn main() {
    let parrot = Cage { occupant: String::from("Paco") };
    println!("{} {}", parrot.occupant(), Cage { occupant: 7 }.occupant());
}
```

## Code Explanation

- `T` is `String` for the parrot, `i32` for the count; the accessor borrows without moving.
- Output is `Paco 7`.

## Common Mistakes

- **Mixing `T` per value**: one `Cage<String>` cannot hold `i32`. Fix: build a second value.
- **Writing `impl Cage<T>`**: the parameter is undeclared. Fix: use `impl<T> Cage<T>`.

## Best Practices

- Return borrows from accessors; specialize with `impl Cage<f64>` for type-only helpers.

## Exercise

Predict the output, then add `fn swap<U>(self, other: U) -> Cage<U>` replacing the occupant's type.

```rust
struct Cage<T> { occupant: T }

impl<T> Cage<T> {
    fn occupant(&self) -> &T { &self.occupant }
}

fn main() {
    println!("{}", Cage { occupant: 7 }.occupant());
}
```

## Solution

It prints `7`. Swap consumes the old cage:

```rust
struct Cage<T> { occupant: T }

impl<T> Cage<T> {
    fn swap<U>(self, other: U) -> Cage<U> { Cage { occupant: other } }
}

fn main() {
    let number = Cage { occupant: String::from("Paco") }.swap(7);
    println!("{}", number.occupant);
}
```

## Mini Quiz

```yaml
- question: "What does `struct Cage<T>` declare?"
  options:
    - "A struct holding placeholder type T"
    - "A string-only struct"
    - "A function"
    - "A constant"
  answer: 0
  explanation: "T is filled with a concrete type per use."
- question: "Why write `impl<T> Cage<T>`?"
  options:
    - "To declare T before methods for all T"
    - "To import a module"
    - "To derive Debug"
    - "To run tests"
  answer: 0
  explanation: "First `<T>` declares it; second applies it."
- question: "Can one Cage<String> later hold i32?"
  options:
    - "No; T is fixed per value"
    - "Yes, anytime"
    - "Only weekly"
    - "Only with unsafe"
  answer: 0
  explanation: "A new T needs a new value."
```
