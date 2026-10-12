---
slug: clone-copy
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: ownership
chapterTitle: "Chapter 13 — Ownership and Memory"
chapterOrder: 13
title: "Clone versus Copy"
difficulty: beginner
order: 4
estimatedMinutes: 10
tags: [rust, ownership, clone, copy]
---

## Learning Objective

By the end of this lesson, you will choose between `Clone` (explicit deep copy) and `Copy` (implicit bitwise duplicate).

## Explanation

The **`Clone` trait** (a shared method interface) gives `.clone()`, an explicit deep copy: cloning a `String` duplicates heap bytes, so every allocation shows in the source. The **`Copy` trait** marks types duplicating by bitwise stack copy — integers, `bool`, `char`, shared references. `Copy` types never move: after `let y = x;`, `x` still works. Heap types can never be `Copy`; opt in with `#[derive(Copy, Clone)]` (a **derive** auto-generates the code).

## Why It Matters

Cheap stack duplicates happen silently, heap duplication always spells `.clone()`; spotting those calls finds allocations worth borrowing away.

## Example

`x: i32` copies implicitly; derived `Point` copies too; `String` needs `.clone()` for a second owner.

## Code

```rust
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);
fn main() {
    let (x, p1) = (42, Point(3, 4));
    let (y, p2) = (x, p1);
    println!("{x} {y} {p1:?} {p2:?}");
    let s = String::from("hi");
    println!("{} {}", s, s.clone());
}
```

## Code Explanation

- `let (y, p2) = (x, p1);` duplicates stack bytes; both sources stay valid because `i32` and `Point` are `Copy`.
- `s.clone()` deep-copies heap bytes into an independent second owner that frees its own memory.

## Common Mistakes

- **Deriving `Copy` with a `String` field**: rejected, since bitwise copies would double-free; use `Clone` only.
- **Assuming `.clone()` is cheap**: it deep-copies; fine to start with, then optimize the clones away.

## Best Practices

- Derive `Copy` only for small stack-only types; treat each `.clone()` as a future borrowing candidate.

## Exercise

Why does `p1` survive but `s` not? Fix the last line:

```rust
// DOES-NOT-COMPILE
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);
fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("{p1:?} {p2:?}");
    let s = String::from("hi");
    let t = s;
    println!("{s} {t}");
}
```

## Solution

`Point` is `Copy` (duplicated); `String` moved. Clone instead:

```rust
fn main() {
    let s = String::from("hi");
    let t = s.clone();
    println!("{s} {t}");
}
```

## Mini Quiz

```yaml
- question: "What does String .clone() do?"
  options:
    - "Moves ownership"
    - "Explicit deep copy with its own heap bytes"
    - "Borrows the bytes"
    - "Frees the original"
  answer: 1
  explanation: "Clone duplicates heap data for two owners."
- question: "Which types can be Copy?"
  options:
    - "Any Clone type"
    - "Small stack-only types like i32, bool, char"
    - "String and Vec"
    - "Drop types"
  answer: 1
  explanation: "Copy is a bitwise stack duplicate; heap owners excluded."
- question: "After `let y = x;` with x: i32, is x usable?"
  options:
    - "No, it moved"
    - "Yes, i32 was duplicated"
    - "Only in unsafe"
    - "Only if mut"
  answer: 1
  explanation: "Copy types never move."
```
