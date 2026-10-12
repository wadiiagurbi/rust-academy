---
slug: supertraits
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: methods-traits
chapterTitle: "Chapter 8 — Methods and Traits"
chapterOrder: 8
title: "Supertraits and Associated Types"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, traits, supertraits, associated-types]
---

## Learning Objective

By the end of this lesson, you will require a supertrait and fill in an associated type.

## Explanation

A **supertrait** (`trait Child: Parent`) requires implementors to also implement `Parent`. An **associated type** (`type Item;`) is a placeholder each implementor fills once, fitting outputs tied to `Self` like an iterator's single `Item` type.

## Why It Matters

Supertraits compose capabilities, and associated types keep signatures short: one `Item` per type, no repeated generics.

## Example

A `Report` trait requires `Display` text output and names its line type once per implementor.

## Code

```rust
use std::fmt::Display;

trait Report: Display {
    type Line;
    fn line(&self) -> Self::Line;
}

struct Score(u32);

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "score {}", self.0)
    }
}

impl Report for Score {
    type Line = String;
    fn line(&self) -> String {
        format!("{}", self)
    }
}

fn main() {
    println!("{}", Score(99).line());
}
```

## Code Explanation

- `trait Report: Display` forces every reporter to be printable.
- `type Line = String` fixes this reporter's line type.
- `format!("{}", self)` renders through the `Display` impl, printing `score 99`.

## Common Mistakes

- **Missing the supertrait**: `impl Report` alone errors. Fix: implement `Display` too.
- **Forgetting `type Line`**: the compiler demands it. Fix: assign a concrete type per impl.

## Best Practices

- State real dependencies as supertraits; name associated types clearly (`Item`, `Line`).

## Exercise

Using the Code section's `Score(99)`, predict the output, then change it to `Score(7)` and confirm the printed line.

## Solution

It prints `score 99`; the value flows through `Display`:

```rust
use std::fmt::Display;

struct Score(u32);

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "score {}", self.0) }
}

fn main() { println!("{}", Score(7)); }
```

## Mini Quiz

```yaml
- question: "What does `trait Report: Display` require?"
  options:
    - "Each Report impl also implements Display"
    - "Display implements Report"
    - "Report holds Display fields"
    - "Nothing"
  answer: 0
  explanation: "The bound after the colon is checked by the compiler."
- question: "What is an associated type?"
  options:
    - "A placeholder each implementor fills once"
    - "An alias for String"
    - "A per-method generic"
    - "A private field"
  answer: 0
  explanation: "One concrete type per impl keeps APIs tidy."
- question: "Associated type vs generic?"
  options:
    - "Use associated when each type has one natural output"
    - "Use it for speed"
    - "Use it for empty traits"
    - "Never"
  answer: 0
  explanation: "One Item per type beats repeating parameters."
```
