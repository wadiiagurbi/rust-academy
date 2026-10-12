---
slug: if-expressions
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: control-flow
chapterTitle: "Chapter 3 — Control Flow"
chapterOrder: 3
title: "If Expressions"
difficulty: beginner
order: 2
estimatedMinutes: 8
tags: [rust, if, expressions, branches]
---

## Learning Objective

By the end of this lesson, you will return values from `if` with
matching branch types.

## Explanation

An **`if` expression** tests a **condition** (a `bool` like `x < 20`)
and runs one **branch**. No parentheses wrap the condition; braces are
required. As an expression, `if` yields the taken branch value, so
both arms need the same **type**. In `let size = if ...`, the `let`
itself needs `;`. Conditions must be exactly `bool`, never numbers.

## Why It Matters

Returning from `if` removes `let mut` temporaries and previews how
`match` works as an expression too.

## Example

Weight under 20 maps to `"small"`, else `"large"`, stored in `label`
for later printing.

## Code

```rust
fn main() {
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("{x} is {size}");
}
```

## Code Explanation

- Both arms return `&str`, so types agree.
- `let size` stores the result; output is `10 is small`.

## Common Mistakes

- **`"small";` in arm**: yields `()` and mismatches. Keep arms bare.
- **`if x` with integer**: needs `bool`; write `if x != 0`.

## Best Practices

- Prefer `if` values over mutating temporaries.
- Cover fallback with `else` so one arm always runs.

## Exercise

Fix the type mismatch to print `small`:

```rust
fn main() {
    let x = 10;
    let size = if x < 20 { "small" } else { 99 };
    println!("{size}");
}
```

## Solution

Arms must share `&str`:

```rust
fn main() {
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("{size}");
}
```

## Mini Quiz

```yaml
- question: "What must if branches share?"
  options:
    - "Same names"
    - "Same value type"
    - "Same length"
    - "Nothing"
  answer: 1
  explanation: "if has one type shared by arms."
- question: "Parentheses around condition?"
  options:
    - "Always"
    - "No, write if x < 20"
    - "Only for else-if"
    - "Only release"
  answer: 1
  explanation: "No parens; braces required."
- question: "Condition type?"
  options:
    - "Any integer"
    - "Exactly bool"
    - "Any string"
    - "Any Option"
  answer: 1
  explanation: "Comparisons must produce bool."
```
