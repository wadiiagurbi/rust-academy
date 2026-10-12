---
slug: destructuring-match
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: pattern-matching
chapterTitle: "Chapter 7 — Pattern Matching"
chapterOrder: 7
title: "Destructuring Structs and Enums"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, destructuring, structs, enums]
---

## Learning Objective

By the end of this lesson, you will destructure structs and enum variants to bind their fields.

## Explanation

**Destructuring** breaks a composite value into parts inside a pattern. A **struct** groups named data called **fields**; a **tuple variant** like `Circle(f64)` holds values in parentheses, while a **struct variant** like `Rect { w: f64, h: f64 }` holds named fields. Mirror the shape in the pattern, and use `..` (**rest**) to skip fields you do not need.

## Why It Matters

The pattern documents the expected shape, and the compiler flags structs whose new fields your code ignores without `..`.

## Example

A geometry tool stores shapes as an enum; one `match` pulls out each shape's numbers to compute its area.

## Code

```rust
enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14159 * radius * radius,
        Shape::Rect { w, h } => w * h,
    }
}

fn main() {
    println!("{:.1}", area(Shape::Rect { w: 3.0, h: 4.0 }));
}
```

## Code Explanation

- `Shape::Circle(radius)` binds the inner number to `radius`.
- `Shape::Rect { w, h }` binds both named fields at once.
- The 3-by-4 rectangle prints `12.0`.

## Common Mistakes

- **Skipping a field without `..`**: `Point { x }` errors when `y` exists. Fix: add the field or `..`.
- **Mixing tuple and struct syntax**: `Rect(w, h)` differs from `Rect { w, h }`. Fix: mirror the definition.

## Best Practices

- Destructure at the arm when you need one or two fields; use `..` to mark the rest ignored.

## Exercise

Predict the output, then bind only `x` with `..` and print just it.

```rust
struct Point { x: i32, y: i32 }

fn main() {
    let Point { x, y } = Point { x: 0, y: 0 };
    println!("{x} {y}");
}
```

## Solution

It prints `0 0`. Skipping `y` looks like this:

```rust
struct Point { x: i32, y: i32 }

fn main() {
    let Point { x, .. } = Point { x: 0, y: 0 };
    println!("{x}");
}
```

## Mini Quiz

```yaml
- question: "What is destructuring?"
  options:
    - "Deleting a value"
    - "Breaking a value into parts inside a pattern"
    - "Converting to a string"
    - "Hiding fields"
  answer: 1
  explanation: "Inner fields bind to names in one step."
- question: "What does `..` mean in a struct pattern?"
  options:
    - "Repeat a field"
    - "Skip the remaining fields"
    - "Match empty structs"
    - "Bind all to one name"
  answer: 1
  explanation: "`..` marks other fields intentionally ignored."
- question: "How do you match `Rect { w, h }`?"
  options:
    - "With `Shape::Rect { w, h }` mirroring the definition"
    - "With `Shape::Rect(w, h)`"
    - "With `Rect[w, h]`"
    - "With `Shape.Rect`"
  answer: 0
  explanation: "Patterns must mirror the definition shape."
```
