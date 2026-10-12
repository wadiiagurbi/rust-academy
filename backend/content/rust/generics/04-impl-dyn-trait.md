---
slug: impl-dyn-trait
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: generics
chapterTitle: "Chapter 9 — Generics"
chapterOrder: 9
title: "Impl Trait and Dyn Trait"
difficulty: intermediate
order: 4
estimatedMinutes: 12
tags: [rust, generics, impl-trait, dyn]
---

## Learning Objective

By the end of this lesson, you will choose `impl Trait` static dispatch or `dyn Trait` dynamic dispatch.

## Explanation

**Static dispatch** (`impl Trait`) pins the type at compile time with specialized code. **Dynamic dispatch** (`dyn Trait`) picks the impl at runtime via a **trait object**: a pointer plus a method table (**vtable**). A **`Box<dyn Trait>`** owns such an object on the heap. Use `impl Trait` for speed, `dyn Trait` for mixed-type collections.

## Why It Matters

Static is zero-cost; dynamic holds mixed types behind one interface. The choice trades speed against flexibility.

## Example

A shelter greets one pet statically, then holds cats and dogs together in a boxed vector.

## Code

```rust
trait Speak { fn speak(&self) -> String; }

struct Cat;
struct Dog;

impl Speak for Cat { fn speak(&self) -> String { String::from("meow") } }
impl Speak for Dog { fn speak(&self) -> String { String::from("woof") } }

fn greet(pet: &impl Speak) -> String { pet.speak() }

fn main() {
    let pets: Vec<Box<dyn Speak>> = vec![Box::new(Cat), Box::new(Dog)];
    println!("{}", greet(&Cat));
    println!("{}", pets[1].speak());
}
```

## Code Explanation

- `Cat;` is a **unit struct** (marker with no fields).
- `greet` uses static dispatch; the vector uses `Box<dyn Speak>` for mixed pets.
- Output is `meow` then `woof`.

## Common Mistakes

- **Storing bare `dyn`**: unsized, cannot live in a `Vec`. Fix: wrap in `Box` or `&`.
- **Mixed list with `impl Trait`**: one `impl` hides one type. Fix: use `Box<dyn Trait>` elements.

## Best Practices

- Default to `impl Trait` params; reach for `Box<dyn Trait>` for heterogeneous collections.

## Exercise

Predict the output (`meow` then `woof`), then add a `Duck` unit struct saying `quack`: implement `Speak` for it, greet it statically, and push one into `pets`.

## Solution

It prints `meow`. The new type joins both paths:

```rust
trait Speak { fn speak(&self) -> String; }
struct Duck;
impl Speak for Duck { fn speak(&self) -> String { String::from("quack") } }
fn greet(pet: &impl Speak) -> String { pet.speak() }
fn main() {
    let pets: Vec<Box<dyn Speak>> = vec![Box::new(Duck)];
    println!("{}", greet(&Duck));
    println!("{}", pets[0].speak());
}
```

## Mini Quiz

```yaml
- question: "impl Trait vs dyn Trait?"
  options:
    - "impl resolves at compile time; dyn at runtime via vtable"
    - "They are identical"
    - "impl is slower"
    - "dyn needs no trait"
  answer: 0
  explanation: "Static specializes; dynamic looks up methods live."
- question: "Why wrap `dyn` in `Box`?"
  options:
    - "Trait objects are unsized; pointers have known size"
    - "Box is louder"
    - "Only style"
    - "Box drops the vtable"
  answer: 0
  explanation: "Boxes give a fixed pointer size for any implementor."
- question: "When prefer `Box<dyn Trait>`?"
  options:
    - "One collection holding mixed types"
    - "Every parameter"
    - "Never"
    - "Integers only"
  answer: 0
  explanation: "Mixed collections are the classic case."
```
