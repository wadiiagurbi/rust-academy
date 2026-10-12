---
slug: lifetime-functions
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: lifetimes
chapterTitle: "Chapter 16 — Lifetimes"
chapterOrder: 16
title: "Lifetimes in Functions"
difficulty: intermediate
order: 1
estimatedMinutes: 12
tags: [rust, lifetimes, borrowing, functions]
---

## Learning Objective

By the end of this lesson, you will annotate functions with lifetime parameters tying returned borrows to their source.

## Explanation

A **lifetime** is how long a reference stays valid; a **lifetime parameter** (`<'a>`) names one span so inputs and outputs can share it. `fn pick<'a>(c: bool, a: &'a i32, b: &'a i32) -> &'a i32` says "the result borrows from whichever input it came from — assume both". Calls check against the signature alone, never the body. Two inputs plus a borrowed return defeat elision, so annotation is mandatory.

## Why It Matters

Lifetimes are borrowing across calls: libraries promise "my output lives exactly as long as your input".

## Example

`pick(true, &a, &b)` may return either side, so the result borrows both — mutating either while `r` lives is rejected.

## Code

```rust
fn pick<'a>(c: bool, a: &'a i32, b: &'a i32) -> &'a i32 {
    if c {
        a
    } else {
        b
    }
}
fn main() {
    let (a, b) = (5, 10);
    println!("picked: {}", pick(true, &a, &b));
}
```

## Code Explanation

- `<'a>` spans the call; both inputs and the output share it, so callers must treat both inputs as borrowed while the result is used.
- Broken variant (does not compile): without annotations it fails with error[E0106]: missing lifetime specifier.

## Common Mistakes

- **Omitting annotations**: two inputs plus a return need `'a`; a shared `'a` borrows both, even if runtime picks one branch.
- **Mutating inputs early**: the extended borrow forbids it until the result's last use.

## Best Practices

- One shared `'a` when either input may flow out; separate names when outputs tie to exactly one input.

## Exercise

Fix the signature, then state what `r` borrows:

```rust
// DOES-NOT-COMPILE
fn multiple(a: &i32, b: &i32) -> &i32 {
    if a > b {
        a
    } else {
        b
    }
}
fn main() {
    let (a, b) = (5, 10);
    let r = multiple(&a, &b);
    println!("{r}");
}
```

## Solution

One shared lifetime ties output to both inputs:

```rust
fn multiple<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}
fn main() {
    let (a, b) = (5, 10);
    let r = multiple(&a, &b);
    println!("{r}");
}
```

`r` borrows both; mutating either before its last use is rejected.

## Mini Quiz

```yaml
- question: "What does <'a> introduce?"
  options:
    - "A generic value"
    - "A named span relating signature borrows"
    - "A loop label"
    - "An allocation"
  answer: 1
  explanation: "Parameters name spans shared by inputs and outputs."
- question: "What does pick's result borrow?"
  options:
    - "Only a"
    - "Only b"
    - "Both a and b"
    - "Neither"
  answer: 2
  explanation: "One shared name covers either source."
- question: "Does checking read function bodies at call sites?"
  options:
    - "Yes"
    - "No, signatures alone"
    - "Only std"
    - "Only unsafe"
  answer: 1
  explanation: "Signatures carry the borrow contract."
```
