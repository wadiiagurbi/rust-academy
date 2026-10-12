---
slug: memory-model
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: ownership
chapterTitle: "Chapter 13 — Ownership and Memory"
chapterOrder: 13
title: "Stack, Heap, and the Memory Model"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, ownership, memory, stack, heap]
---

## Learning Objective

By the end of this lesson, you will describe where Rust values live (stack versus heap) and why that split needs ownership rules.

## Explanation

A **stack** is a fast region tied to function calls: each local gets a fixed-size slot freed on return. A **heap** is a pool for values sized only at runtime (like a growing `String`); it needs bookkeeping, so it is slower and never cleans itself up. A `String` shows both: the **metadata** (pointer, length, capacity) sits on the stack while the bytes live on the heap. **Capacity** is reserved bytes; **length** is used bytes. Heap data outlives any one frame, so Rust needs rules for who frees it — that is ownership.

## Why It Matters

Every ownership error is the compiler defending this model: stack slots free themselves, heap memory needs exactly one cleaner.

## Example

`String::from("Hello")` stores `(ptr, len 5, cap 5)` on the stack and the five bytes on the heap; `len()` reads the metadata.

## Code

```rust
fn main() {
    let s = String::from("Hello");
    println!("len: {}, capacity: {}", s.len(), s.capacity());
}
```

## Code Explanation

- `String::from("Hello")` heap-allocates the bytes; `s` holds the `(pointer, length, capacity)` triple on the stack.
- `s.len()` reads used bytes (5); `s.capacity()` reads reserved bytes (5 here; it grows after `push_str` reallocates).

## Common Mistakes

- **Thinking `String` is stack-only**: the handle is fixed size, the bytes are not — that gap is why ownership rules exist.
- **Mixing length and capacity**: length is used bytes, capacity is reserved bytes.
- **Expecting a collector**: values drop when their owner leaves scope, never via background collection.

## Best Practices

- Keep small fixed-size data (`i32`, `bool`) on the stack by value; sketch the stack triple plus heap bytes when confused.

## Exercise

Predict the numbers, then check with `rustc`:

```rust
fn main() {
    let s = String::from("Hello, world");
    println!("{} {}", s.len(), s.capacity());
}
```

## Solution

It prints `12 12`:

```rust
fn main() {
    let s = String::from("Hello, world");
    assert_eq!((s.len(), s.capacity()), (12, 12));
    println!("{} {}", s.len(), s.capacity());
}
```

Twelve heap bytes; the stack handle records length 12, capacity 12.

## Mini Quiz

```yaml
- question: "Where do a String's bytes live?"
  options:
    - "On the stack, next to the variable"
    - "On the heap, reached through a stack pointer"
    - "In the binary only"
    - "In a collected pool"
  answer: 1
  explanation: "The stack holds (ptr, len, cap); bytes live on the heap."
- question: "What is capacity?"
  options:
    - "Used bytes"
    - "Reserved heap bytes, possibly more than used"
    - "Variables in scope"
    - "Stack frame size"
  answer: 1
  explanation: "Capacity is reserved space; length is used space."
- question: "Why does heap data need ownership rules?"
  options:
    - "The heap is slower"
    - "Someone must free it exactly once, with no collector"
    - "Stack values need none"
    - "Heap slots are fixed size"
  answer: 1
  explanation: "The compiler assigns exactly one owner to free each value."
```
