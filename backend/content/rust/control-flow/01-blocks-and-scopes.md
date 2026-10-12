---
slug: blocks-scopes
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: control-flow
chapterTitle: "Chapter 3 — Control Flow"
chapterOrder: 3
title: "Blocks and Scopes"
difficulty: beginner
order: 1
estimatedMinutes: 8
tags: [rust, blocks, scopes, expressions]
---

## Learning Objective

By the end of this lesson, you will return values from blocks and
track scope visibility.

## Explanation

A **block** is `{ ... }` holding statements and expressions. Its value
is the final expression without `;`; with `;` it yields `()` (called
**unit**, meaning no value). An **expression** produces a value; a
**statement** ends with `;`. A **scope** runs from `let` to the
closing brace; inner blocks see outer bindings, not vice versa.
**Shadowing** (`let y = y + 1;`) re-binds the same name in place.

## Why It Matters

Functions, branches, and loops are all blocks. The tail-value rule
removes `return` clutter and keeps data flow clear.

## Example

A discount block keeps `rate` local and returns only the price.
Using `rate` outside fails with "cannot find value".

## Code

```rust
fn main() {
    let z = 13;
    let x = {
        let y = 10;
        z - y
    };
    println!("x: {x}");
}
```

## Code Explanation

- `z` is outer and visible inside; `y` dies at the brace.
- `z - y` without `;` returns `3` into `x`, printed as `x: 3`.

## Common Mistakes

- **Trailing `;`**: `z - y;` returns `()` and mismatches uses.
- **Using `y` outside**: scope ended; move the use inside.

## Best Practices

- Return via tail expressions; reserve `return` for guards.
- Declare bindings in the narrowest block needing them.

## Exercise

What prints? What if the tail gains `;`?

```rust
fn main() {
    let a = 5;
    let b = {
        let c = 7;
        a + c
    };
    println!("b: {b}");
}
```

## Solution

Prints `b: 12`:

```rust
fn main() {
    let a = 5;
    let b = {
        let c = 7;
        a + c
    };
    println!("b: {b}");
}
```

With `a + c;`, `b` becomes `()` instead of `12`.

## Mini Quiz

```yaml
- question: "Value of { let y = 2; y + 1 }?"
  options:
    - "() always"
    - "3 from the tail"
    - "2 from let"
    - "Error"
  answer: 1
  explanation: "Bare tail becomes the block value."
- question: "Scope of let inside a block?"
  options:
    - "Whole file"
    - "Until that block ends"
    - "Before the block"
    - "Sibling blocks"
  answer: 1
  explanation: "Bindings die at the closing brace."
- question: "Trailing ; on tail does what?"
  options:
    - "Speeds code"
    - "Yields ()"
    - "Exports var"
    - "Nothing"
  answer: 1
  explanation: "Semicolon turns value into unit."
```
