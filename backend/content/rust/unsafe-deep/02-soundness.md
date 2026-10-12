---
slug: soundness
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: unsafe-deep
chapterTitle: "Chapter 24 — Unsafe Deep Dive"
chapterOrder: 24
title: "Soundness: Safe Wrappers Must Hold"
difficulty: advanced
order: 2
estimatedMinutes: 12
tags: [rust, unsafe, soundness]
---

## Learning Objective

By the end of this lesson, you will define **soundness** (no misuse of a safe API can cause undefined behaviour) and spot one unsound wrapper.

## Explanation

An API is **sound** when every program built only from its safe functions is free of **undefined behaviour** (UB: the standard voids the program's meaning — anything may happen). Soundness covers reachable misuse, not intended use: if innocent-looking safe inputs trigger UB inside, the wrapper is **unsound**, even if nobody has done so yet. The classic failure is aliasing `&mut` (an exclusive borrow the compiler assumes is unique) — e.g. two live `&mut` to one buffer from a safe function.

## Why It Matters

Unsound libraries poison every downstream crate: callers did nothing wrong yet inherit UB the optimizer can turn into distant miscompilation.

## Example

A bounds-checked writer: safe inputs either write inside the buffer or return `Err` — no input reaches UB.

## Code

```rust
struct Buf { data: [u8; 4] }
impl Buf {
    fn write(&mut self, offset: usize, byte: u8) -> Result<(), String> {
        if offset >= self.data.len() { return Err(format!("offset {offset} out of bounds")); }
        // SAFETY: `offset < len` checked above; `&mut` proves exclusivity.
        unsafe { *self.data.get_unchecked_mut(offset) = byte; }
        Ok(())
    }
}
fn main() {
    let mut b = Buf { data: [0; 4] };
    println!("{:?} {:?}", b.write(1, 9), b.write(7, 9));
    println!("{:?}", b.data);
}
```

## Code Explanation

- `get_unchecked_mut` skips the bounds check; the `if` above is the proof. `&mut self` guarantees no aliasing writer.
- Prints `Ok(())`, `Err(...)`, then `[0, 9, 0, 0]`: misuse becomes an error value.

## Common Mistakes

- **Trusting length fields**: safe callers can desync a stored `len` from its pointer. Recompute from the borrow.
- **Returning `&mut` from shared state**: two calls alias silently. Return values, or take `&mut self`.
- **"Nobody would do that"**: soundness covers all reachable safe programs, including adversarial ones.

## Best Practices

- For each `unsafe` op, list the safe inputs that could violate it; close each with types or checks.
- Fuzz the wrapper and run Miri (`cargo miri test`) to catch aliasing UB.

## Exercise

Add `fn fill(&mut self, value: u8)` writing every slot via the same unchecked store, with a one-line comment arguing why no safe input breaks it.

## Solution

```rust
struct Buf { data: [u8; 4] }
impl Buf {
    fn fill(&mut self, value: u8) {
        for i in 0..self.data.len() {
            // SAFETY: `i` ranges over `0..len` only; `&mut` proves exclusivity.
            unsafe { *self.data.get_unchecked_mut(i) = value; }
        }
    }
}
fn main() {
    let mut b = Buf { data: [0; 4] };
    b.fill(5);
    println!("{:?}", b.data);
}
```

Prints `[5, 5, 5, 5]`: the loop range is itself the bound proof, so no input exists to break.

## Mini Quiz

```yaml
- question: "What does soundness require?"
  options:
    - "No unsafe blocks anywhere"
    - "No safe-API misuse can cause undefined behaviour"
    - "Fast benchmarks"
    - "Zero warnings"
  answer: 1
  explanation: "All reachable safe programs must stay defined, adversarial ones included."
- question: "Why is a second live &mut unsound?"
  options:
    - "It is merely slow"
    - "The compiler assumes exclusivity; aliasing voids that proof"
    - "It leaks memory"
    - "It needs more RAM"
  answer: 1
  explanation: "Optimizer reasoning built on uniqueness then miscompiles."
- question: "How do you review a wrapper for soundness?"
  options:
    - "Read intended uses only"
    - "Name every safe input that could break each unsafe clause"
    - "Count lines of code"
    - "Check formatting"
  answer: 1
  explanation: "Hostile-but-safe inputs are the test set that matters."
```
