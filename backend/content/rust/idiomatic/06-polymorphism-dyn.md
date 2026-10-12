---
slug: polymorphism
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: idiomatic
chapterTitle: "Chapter 23 — Idiomatic Rust"
chapterOrder: 23
title: "Polymorphism with dyn Trait"
difficulty: advanced
order: 6
estimatedMinutes: 12
tags: [rust, dyn, polymorphism]
---

## Learning Objective

By the end of this lesson, you will hold mixed types behind `dyn Trait` (**dynamic dispatch**: method lookup at runtime) and choose it over generics deliberately.

## Explanation

**Generics** (types fixed at compile time) give **static dispatch**: one copy per type — fastest, but the concrete type must be known. `dyn Trait` is a **trait object**: a fat pointer of data plus a **vtable** (a table of function pointers), resolved at runtime. That allows `Vec<Box<dyn Draw>>` (a growable list of owned, mixed drawables) where each element may differ. Trait objects require **object safety**: no generic methods, no `Self`-returning constructors, since the vtable lists fixed signatures only.

## Why It Matters

Plugins, widgets, and parsers mix types in one collection. `dyn` expresses that; generics cannot, and enums need every variant known upfront.

## Example

Two shapes behind one `Draw` trait, drawn from a single vector.

## Code

```rust
trait Draw { fn draw(&self) -> String; }
struct Circle(f64);
struct Square(f64);
impl Draw for Circle {
    fn draw(&self) -> String { format!("circle r={}", self.0) }
}
impl Draw for Square {
    fn draw(&self) -> String { format!("square s={}", self.0) }
}
fn main() {
    let scene: Vec<Box<dyn Draw>> = vec![Box::new(Circle(1.0)), Box::new(Square(2.0))];
    for item in &scene { println!("{}", item.draw()); }
}
```

## Code Explanation

- `Box<dyn Draw>` is an owned trait object: `Box` (heap allocation, single ownership) plus a runtime method table.
- `&scene` borrows so the loop keeps ownership. Prints both shape lines.

## Common Mistakes

- **Generic methods in the trait**: `fn make<T>(&self) -> T` breaks object safety. Split generic parts out.
- **`dyn` everywhere**: lookup plus boxes cost more than generics. Use `dyn` for genuinely mixed collections only.
- **Forgetting `Box`**: bare `dyn Draw` is unsized and cannot sit in a `Vec`. Wrap in `Box`, `Rc`, or `&`.

## Best Practices

- Keep object-safe traits small; put constructors on concrete types, behaviour on the trait.
- Prefer `&dyn Trait` for borrowed views; `Box<dyn Trait>` for owned storage.

## Exercise

Add `struct Line` drawing `"line"`, push it into `scene`, and add `fn count(scene: &[Box<dyn Draw>]) -> usize`. Print both.

## Solution

```rust
trait Draw { fn draw(&self) -> String; }
struct Circle(f64);
struct Line;
impl Draw for Circle {
    fn draw(&self) -> String { format!("circle r={}", self.0) }
}
impl Draw for Line {
    fn draw(&self) -> String { "line".to_string() }
}
fn count(scene: &[Box<dyn Draw>]) -> usize { scene.len() }
fn main() {
    let scene: Vec<Box<dyn Draw>> = vec![Box::new(Circle(1.0)), Box::new(Line)];
    for item in &scene { println!("{}", item.draw()); }
    println!("n={}", count(&scene));
}
```

Prints the two drawings then `n=2`: the slice view borrows without taking ownership.

## Mini Quiz

```yaml
- question: "When is dyn Trait the right call?"
  options:
    - "Always; it is fastest"
    - "Mixed types in one collection, known only at runtime"
    - "Never; it is deprecated"
    - "Only for numbers"
  answer: 1
  explanation: "Dynamic dispatch handles heterogeneity generics cannot express."
- question: "What is a vtable?"
  options:
    - "A hash map of variables"
    - "A per-type table of function pointers behind dyn"
    - "A compile error list"
    - "A memory allocator"
  answer: 1
  explanation: "The fat pointer pairs data with its method table."
- question: "What breaks object safety?"
  options:
    - "Taking &self"
    - "Generic methods or Self-returning constructors"
    - "Returning String"
    - "Having two methods"
  answer: 1
  explanation: "Vtables need fixed signatures; generics have none."
```
