---
slug: macros-intro
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: control-flow
chapterTitle: "Chapter 3 — Control Flow"
chapterOrder: 3
title: "Macros Intro"
difficulty: beginner
order: 7
estimatedMinutes: 8
tags: [rust, macros, println, dbg]
---

## Learning Objective

By the end of this lesson, you will call standard macros and explain
the `!` marker.

## Explanation

A **macro** generates code at compile time and takes variadic args,
which functions cannot. `!` marks calls like `println!`.
`println!` prints a line; `format!` returns an owned `String` (heap
text) instead; `dbg!` logs to stderr and returns its input; `todo!`
panics with not-yet-implemented. A **format string** holds `{}` slots
filled by later args.

## Why It Matters

Seeing `!` explains variadic calls. `format!` keeps helpers testable
without capturing console output.

## Example

Factorial `4!` loops 1–4 accumulating product; `println!` reports 24
while `dbg!` could trace factors during work.

## Code

```rust
fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= i;
    }
    product
}

fn main() {
    println!("4! = {}", factorial(4));
}
```

## Code Explanation

- `1..=n` iterates inclusively; `*=` accumulates.
- Bare `product` returns; output is `4! = 24`.

## Common Mistakes

- **Missing `!`**: `println()` finds no function.
- **Shipping `todo!()`**: it panics at runtime if hit.

## Best Practices

- Use `println!` for output, `format!` for building strings.
- Remove `dbg!` before sharing finished code.

## Exercise

What is the type of `msg`?

```rust
fn main() {
    let msg = format!("hi {}", "Ferris");
    println!("{msg}");
}
```

## Solution

`msg` is an owned `String`:

```rust
fn main() {
    let msg: String = format!("hi {}", "Aisha");
    println!("{msg}");
}
```

Prints `hi Aisha`. `println!` returns `()`; `format!` builds text.

## Mini Quiz

```yaml
- question: "How to spot a macro?"
  options:
    - "Zero args"
    - "Name ends with !"
    - "No return"
    - "In Cargo.toml"
  answer: 1
  explanation: "! marks compile-time expansion."
- question: "println! versus format!?"
  options:
    - "Same"
    - "Print versus return String"
    - "Louder print"
    - "format! prints"
  answer: 1
  explanation: "One shows text, one builds it."
- question: "todo!() on run?"
  options:
    - "Skips silently"
    - "Panics not-yet-implemented"
    - "Formats file"
    - "Returns zero"
  answer: 1
  explanation: "todo!() marks unfinished code."
```
