---
slug: owned-trait-objects
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: smart-pointers
chapterTitle: "Chapter 14 — Smart Pointers"
chapterOrder: 14
title: "Owned Trait Objects"
difficulty: intermediate
order: 3
estimatedMinutes: 12
tags: [rust, smart-pointers, trait-objects, dyn]
---

## Learning Objective

By the end of this lesson, you will hold mixed types behind one interface with `Box<dyn Trait>` and name the vtable cost.

## Explanation

A **trait object** (`dyn Pet`) is a dynamically sized "any type with this trait" view. `Dog` and `Cat` differ in size, so `Vec<dyn Pet>` is impossible — vectors need fixed-size elements. A **fat pointer** (data address plus **vtable** address, a table of method implementations) solves it. `Box<dyn Pet>` pairs heap ownership with that indirection, so one vector mixes cats and dogs via **dynamic dispatch**: one vtable lookup per call.

## Why It Matters

Plugins, widgets, and pets need one container for many types; owned trait objects give that with clear ownership at a small price.

## Example

A vector of boxed cat and dog prints two greetings; each `talk()` dispatches through its own vtable entry.

## Code

```rust
trait Pet { fn talk(&self) -> String; }
struct Dog { name: String }
struct Cat { lives: i8 }
impl Pet for Dog { fn talk(&self) -> String { format!("Woof, I am {}!", self.name) } }
impl Pet for Cat { fn talk(&self) -> String { String::from("Miau!") } }
fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![Box::new(Cat { lives: 9 }), Box::new(Dog { name: "Fido".into() })];
    for pet in pets { println!("{}", pet.talk()); }
}
```

## Code Explanation

- `Box<dyn Pet>` erases size differences (one box plus vtable link each), so mixed `vec![...]` shares one element type; calls resolve at runtime.

## Common Mistakes

- **`Vec<dyn Pet>`**: unsized — every trait object needs `Box`, `&`, or `Rc` around it; edition 2021 also requires spelling `dyn Pet`.

## Best Practices

- Use `Box<dyn Trait>` for owned mixed sets with a narrow interface; prefer generics when types are known at compile time.

## Exercise

Add a `Fish`, then predict the output:

```rust
trait Pet { fn talk(&self) -> String; }
struct Fish;
impl Pet for Fish { fn talk(&self) -> String { String::from("Blub!") } }
fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![Box::new(Fish)];
    for pet in pets { println!("{}", pet.talk()); }
}
```

## Solution

`Blub!` — the vtable routes to `Fish::talk`:

```rust
trait Pet { fn talk(&self) -> String; }
struct Fish;
impl Pet for Fish { fn talk(&self) -> String { String::from("Blub!") } }
fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![Box::new(Fish)];
    assert_eq!(pets.len(), 1);
    println!("{}", pets[0].talk());
}
```

## Mini Quiz

```yaml
- question: "Why is Vec<dyn Pet> rejected?"
  options:
    - "Traits forbid vectors"
    - "dyn Pet is unsized; elements need one fixed size"
    - "Vectors cannot own"
    - "Pet lacks methods"
  answer: 1
  explanation: "Mixed sizes need indirection (Box)."
- question: "What is a fat pointer?"
  options:
    - "Data address plus vtable address"
    - "Double heap use"
    - "A counted pointer"
    - "A null pointer"
  answer: 0
  explanation: "Both addresses enable runtime lookup."
- question: "Cost of calling through dyn?"
  options:
    - "None"
    - "One vtable hop per call"
    - "An allocation per call"
    - "A thread per call"
  answer: 1
  explanation: "The method is found in the vtable at runtime."
```
