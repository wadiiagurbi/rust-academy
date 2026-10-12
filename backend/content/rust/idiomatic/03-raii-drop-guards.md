---
slug: drop-guards
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: idiomatic
chapterTitle: "Chapter 23 — Idiomatic Rust"
chapterOrder: 23
title: "RAII and Drop Guards"
difficulty: intermediate
order: 3
estimatedMinutes: 10
tags: [rust, raii, drop]
---

## Learning Objective

By the end of this lesson, you will write a **guard** (a value whose `Drop` runs cleanup) so scoped work always restores state, even on early return.

## Explanation

**RAII** ties cleanup to ownership: when a value's **scope** (the block it lives in) ends, its `Drop::drop` runs automatically — files close, locks release. A **guard** is a small struct holding what must be undone; its `drop` performs the undo. This covers early `return`, `?` (the operator returning errors upward), and panics alike, because all of them run destructors while unwinding.

## Why It Matters

Manual cleanup at every exit leaks locks and skews logs. Guards put the undo next to the do, once, and the compiler runs it on every path.

## Example

An indentation guard: entering a section indents logging, leaving always dedents.

## Code

```rust
use std::cell::Cell;
use std::rc::Rc;
struct Indent { depth: Rc<Cell<usize>> }
impl Indent {
    fn enter(d: &Rc<Cell<usize>>) -> Self { d.set(d.get() + 1); Self { depth: Rc::clone(d) } }
}
impl Drop for Indent {
    fn drop(&mut self) { self.depth.set(self.depth.get() - 1); }
}
fn main() {
    let depth = Rc::new(Cell::new(0));
    { let _g = Indent::enter(&depth); println!("in: {}", depth.get()); }
    println!("out: {}", depth.get());
}
```

## Code Explanation

- `Cell<usize>` is **interior mutability**: shared-but-mutable state for one thread; `Rc` (reference-counted sharing) lets guard and owner share it.
- `_g` lives to end of block; `drop` runs at `}`. Prints `in: 1` then `out: 0`.

## Common Mistakes

- **Forgetting the binding**: `Indent::enter(&d);` drops instantly. Bind it (`let _g = ...`).
- **Cleanup in a method**: `fn close()` relies on callers remembering. Put it in `drop` instead.
- **Panicking in `drop`**: a panic while unwinding aborts the program. Keep `drop` infallible.

## Best Practices

- Name guards after the held state (`Indent`, `LockGuard`); document what `drop` restores.
- Hold only what cleanup needs — shared handles via `Rc`/`Arc`, never bulky clones.

## Exercise

Add `fn log(&self, msg: &str)` printing two spaces per depth plus the message. Enter two nested guards and log from each level.

## Solution

```rust
use std::cell::Cell;
use std::rc::Rc;
struct Indent { depth: Rc<Cell<usize>> }
impl Indent {
    fn enter(d: &Rc<Cell<usize>>) -> Self { d.set(d.get() + 1); Self { depth: Rc::clone(d) } }
    fn log(&self, msg: &str) { println!("{}{}", "  ".repeat(self.depth.get()), msg); }
}
impl Drop for Indent {
    fn drop(&mut self) { self.depth.set(self.depth.get() - 1); }
}
fn main() {
    let depth = Rc::new(Cell::new(0));
    let a = Indent::enter(&depth);
    a.log("outer");
    { Indent::enter(&depth).log("inner"); }
    a.log("back");
}
```

Prints depths 1, 2, 1: each exit dedents exactly once.

## Mini Quiz

```yaml
- question: "What is RAII?"
  options:
    - "Manual free() calls"
    - "Cleanup tied to ownership; drop runs at scope end"
    - "A garbage collector pass"
    - "Reference counting only"
  answer: 1
  explanation: "Owning values clean themselves on every exit path automatically."
- question: "Why bind the guard to _g?"
  options:
    - "Style only"
    - "An unbound temporary drops immediately, undoing too early"
    - "It speeds up code"
    - "The compiler requires the name"
  answer: 1
  explanation: "Lifetime of the guard equals the protected region."
- question: "What must drop() avoid?"
  options:
    - "Reading fields"
    - "Panicking — it can abort during unwinding"
    - "Running quickly"
    - "Being documented"
  answer: 1
  explanation: "Keep destructors infallible so failures stay recoverable."
```
