---
slug: pinning
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: unsafe-deep
chapterTitle: "Chapter 24 — Unsafe Deep Dive"
chapterOrder: 24
title: "Pinning and Immovable Futures"
difficulty: advanced
order: 4
estimatedMinutes: 15
tags: [rust, pin, async]
---

## Learning Objective

By the end of this lesson, you will explain **pinning** (a promise a value will never move) and use `Pin` to drive a value in place.

## Explanation

Some values point into themselves — an **async block** (a paused computation the executor resumes) keeps state referencing its own frame. Moving it would dangle those inner pointers, so types **opt out of moving** via `PhantomPinned` (a marker making the type `!Unpin`, i.e. not movable once pinned). `Pin<P>` (a wrapper around a pointer `P`) is the contract: access through `&mut`, but never move out or swap. `Box::pin` (pin a heap value in one step) is the common entry; `Pin::new` works only for `Unpin` types needing no promise.

## Why It Matters

Every `.await` runs inside a pinned future. Knowing what pinning forbids (moves, swaps, `mem::replace`) explains executor errors that otherwise look like noise.

## Example

Pin a counter on the heap, mutate it in place, and show the address never changes.

## Code

```rust
use std::pin::Pin;
#[derive(Debug)]
struct Counter { n: u32 }
impl Counter {
    fn bump(self: Pin<&mut Self>) { self.get_mut().n += 1; }
}
fn main() {
    let mut pinned: Pin<Box<Counter>> = Box::pin(Counter { n: 0 });
    let before = &*pinned as *const Counter as usize;
    pinned.as_mut().bump();
    println!("{pinned:?} moved={}", before != &*pinned as *const Counter as usize);
}
```

## Code Explanation

- `Pin<&mut Self>` means "callable only when pinned"; `get_mut` gives in-place access without moving.
- `&*pinned as *const Counter` reads the heap address (a number, not a dereference). Prints `Counter { n: 1 } moved=false`.

## Common Mistakes

- **Moving out of `Pin`**: `*pinned` or `mem::swap` breaks the promise — the compiler rejects it; do not `unsafe` around it.
- **Assuming all types need it**: most types are `Unpin` (movable, unaffected). Only `!Unpin` types (futures, `PhantomPinned`) need discipline.
- **Pinning the stack wrongly**: `Box::pin` is safe; stack locals need `pin!` — scope them carefully.

## Best Practices

- Take `self: Pin<&mut Self>` only where self-references demand it; plain `&mut self` elsewhere.
- Prefer `Box::pin` and `pin!`; reserve `new_unchecked` for audited libraries.

## Exercise

Add `fn reset(self: Pin<&mut Self>)` setting `n` to 0. Bump twice, reset once, printing after each step.

## Solution

```rust
use std::pin::Pin;
#[derive(Debug)]
struct Counter { n: u32 }
impl Counter {
    fn bump(self: Pin<&mut Self>) { self.get_mut().n += 1; }
    fn reset(self: Pin<&mut Self>) { self.get_mut().n = 0; }
}
fn main() {
    let mut pinned: Pin<Box<Counter>> = Box::pin(Counter { n: 0 });
    pinned.as_mut().bump();
    pinned.as_mut().bump();
    println!("{pinned:?}");
    pinned.as_mut().reset();
    println!("{pinned:?}");
}
```

Prints `Counter { n: 2 }` then `Counter { n: 0 }`: all mutation happened in place.

## Mini Quiz

```yaml
- question: "What does pinning promise?"
  options:
    - "The value is immutable"
    - "The value will never move in memory again"
    - "The value is heap allocated"
    - "The value is thread-safe"
  answer: 1
  explanation: "Self-referential state stays valid only at a fixed address."
- question: "Which types need pinning discipline?"
  options:
    - "All types"
    - "Only !Unpin types like futures holding self-references"
    - "Only integers"
    - "Only static items"
  answer: 1
  explanation: "Unpin types move freely; !Unpin types opt out once pinned."
- question: "Why self: Pin<&mut Self> receivers?"
  options:
    - "They run faster"
    - "They prove the call happens at a pinned address"
    - "They avoid borrowing"
    - "They are required syntax everywhere"
  answer: 1
  explanation: "The receiver type carries the no-move guarantee into the method."
```
