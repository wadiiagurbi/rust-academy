---
slug: destructuring
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: compound-types
chapterTitle: "Chapter 4 — Compound Types"
chapterOrder: 4
title: "Destructuring"
difficulty: beginner
order: 4
estimatedMinutes: 8
tags: [rust, patterns, destructuring, tuples]
---

## Learning Objective

By the end of this lesson, you will split tuples with patterns and
skip parts with `_` and `..`.

## Explanation

**Destructuring** splits compounds in one `let`, as in `let (l, m, r)
= t;`. The **pattern** is the left shape; names become bindings.
These are **irrefutable** (always matching), checked at compile time.
`_` (called **wildcard**) drops one piece; `..` (called **rest**)
drops several, as in `(first, .., last)`.

## Why It Matters

Named parts beat `t.0` indexing, and `_` documents intentional gaps
without warnings.

## Example

`(1, 5, 3)` unpacks to test `l < m && m < r`, printing unordered
without dot indexes.

## Code

```rust
fn check_order(t: (i32, i32, i32)) -> bool {
    let (l, m, r) = t;
    l < m && m < r
}

fn main() {
    println!("{}", check_order((1, 5, 3)));
}
```

## Code Explanation

- Pattern binds `l=1, m=5, r=3`; `&&` chains comparisons to false.
- Output is `false` since `5 < 3` fails.

## Common Mistakes

- **Wrong arity**: two-pattern on three-tuple fails; match counts.
- **Mixing names and .0**: pick one source of truth per scope.

## Best Practices

- Destructure at entry when callers pass tuples.
- Use `_` singly and `..` for runs of ignored values.

## Exercise

Print only the middle, then edges:

```rust
fn main() {
    let data = ('a', 777, true);
    let (_, num, _) = data;
    println!("num: {num}");
}
```

## Solution

Keeps edges, skips middle:

```rust
fn main() {
    let data = ('a', 777, true);
    let (first, .., last) = data;
    println!("{first} {last}");
}
```

Prints `a true`. `..` skipped `777`.

## Mini Quiz

```yaml
- question: "let (a, b, c) = t does what?"
  options:
    - "Brackets index"
    - "Splits into bindings"
    - "Creates tuple"
    - "Loops"
  answer: 1
  explanation: "Pattern binds each position."
- question: "_ in patterns?"
  options:
    - "Binds later"
    - "Discards that piece"
    - "Matches zero"
    - "Exits"
  answer: 1
  explanation: "Wildcard ignores intentionally."
- question: ".. in (first, .., last)?"
  options:
    - "Binds middle"
    - "Skips middle, keeps edges"
    - "Reverses"
    - "Runtime check"
  answer: 1
  explanation: "Rest skips unnamed positions."
```
