---
slug: enums
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: user-types
chapterTitle: "Chapter 6 — User-Defined Types"
chapterOrder: 6
title: "Enums"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, enums, types, match]
---

## Learning Objective

By the end of this lesson, you will model choices with enums carrying unit, tuple, and struct data.

## Explanation

An **enum** is a type with a fixed set of **variants** (allowed cases),
as in `enum Direction { Left, Right }`. Each value is one variant
(`Direction::Left`); a **discriminant** tag remembers which one at runtime.

Variants carry data: **unit** (`Pass`), **tuple** (`Run(Direction)`),
**struct** (`Teleport { x: u32, y: u32 }`). `#[derive(Debug)]` enables
`{:?}` inspection. `match` unpacks exhaustively — missing arms fail to compile.

## Why It Matters

Enums replace string flags: `Pass` vs `Run(dir)` cannot mix; payloads ride with their case.

## Example

A game move is `Pass`, `Run(dir)`, or `Teleport { x, y }`. Building
`Run(Direction::Left)` shows tag plus payload in one value.

## Code

```rust
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
}

fn main() {
    let dir = Direction::Left;
    let mv = PlayerMove::Run(dir);
    println!("move: {mv:?}");
}
```

## Code Explanation

- `#[derive(Debug)]` enables `{mv:?}`; without it printing fails.
- `Direction` offers two unit variants — pure choices.
- `PlayerMove` mixes unit (`Pass`) with tuple `Run(Direction)`.
- `Run(dir)` moves `dir` in, printing `move: Run(Left)`.

## Common Mistakes

- **Forgetting `derive(Debug)`**: `{:?}` printing then fails — add it above each enum.
- **Non-exhaustive `match`**: list every variant or add `_` deliberately.
- **Using strings for choices**: `"left"` typos compile; enums make invalid states unrepresentable.

## Best Practices

- Keep payloads inline so data cannot drift from its case.
- Order variants common to rare; document each payload.
- Match exhaustively without `_` so new variants flag handlers.

## Exercise

Add `Teleport { x: u32, y: u32 }`, build `Run('N')` and `(3, 7)`, then match both.

```rust
#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(char),
}
```

## Solution

Struct variants carry named fields:

```rust
#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(char),
    Teleport { x: u32, y: u32 },
}

fn main() {
    let a = PlayerMove::Run('N');
    let b = PlayerMove::Teleport { x: 3, y: 7 };
    println!("{a:?} then {b:?}");
}
```

It prints `Run('N') then Teleport { x: 3, y: 7 }` — one type, each variant its own shape.

## Mini Quiz

```yaml
- question: "What is an enum variant?"
  options:
    - "A Cargo dependency"
    - "One allowed case of the enum, optionally with data"
    - "A compiler warning"
    - "A file in target/"
  answer: 1
  explanation: "Variants enumerate choices; payloads ride along with the case."
- question: "What does #[derive(Debug)] do?"
  options:
    - "Publishes the crate"
    - "Auto-generates {:?} printing support"
    - "Runs the test suite"
    - "Formats Cargo.toml"
  answer: 1
  explanation: "Deriving Debug enables inspection printing for custom types."
- question: "Why match enums exhaustively?"
  options:
    - "It runs faster"
    - "Missing variants become compile errors, not runtime bugs"
    - "It is required for all integers"
    - "It removes the need for enums"
  answer: 1
  explanation: "Exhaustiveness forces every case to be handled explicitly."
```
