---
slug: embedded-crates
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: bare-metal
chapterTitle: "Chapter 25 — Bare-Metal Rust"
chapterOrder: 25
title: "Useful Embedded Crates"
difficulty: advanced
order: 4
estimatedMinutes: 10
tags: [rust, embedded, crates]
---

## Learning Objective

By the end of this lesson, you will pick the right **crate** (a Rust dependency package) for heapless storage, messaging, and logging on firmware.

## Explanation

Embedded crates fill the gaps `no_std` leaves: **`heapless`** offers fixed-capacity `Vec`/`Queue` (array-backed collections that error instead of allocating); **`embassy`** provides async executors plus channels for tasks without an OS; **`defmt`** is a logging framework sending tiny format indices over the debug probe instead of full strings, saving flash and bandwidth. All three are `core`-friendly and host-testable in logic. The queue below mirrors the `heapless` idea with plain `core`-only code.

## Why It Matters

`std` containers fail to link on firmware; knowing the three standard substitutes unblocks storage, concurrency, and observability immediately.

## Example

A fixed-capacity queue: push until full, pop in order, no heap involved.

## Code

```rust
struct FixedQueue { buf: [u8; 4], head: usize, len: usize }
impl FixedQueue {
    fn new() -> Self { Self { buf: [0; 4], head: 0, len: 0 } }
    fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.len == self.buf.len() { return Err("queue full"); }
        self.buf[(self.head + self.len) % self.buf.len()] = byte; self.len += 1; Ok(())
    }
    fn pop(&mut self) -> Option<u8> {
        if self.len == 0 { return None; }
        let byte = self.buf[self.head]; self.head = (self.head + 1) % self.buf.len(); self.len -= 1; Some(byte)
    }
}
fn main() {
    let mut q = FixedQueue::new(); q.push(1).expect("space"); q.push(2).expect("space");
    println!("{:?} {:?}", q.pop(), q.pop());
}
```

## Code Explanation

- A **ring buffer** reuses slots via modulo indexing (`%` wraps past the end); this is the `heapless` queue idea in miniature: bounded, no allocator, FIFO order. Prints `Some(1) Some(2)`.

## Common Mistakes

- **Assuming `std::collections`**: `VecDeque` needs `alloc`; on bare chips that link fails. Use `heapless` or hand-rolled rings.
- **Unbounded channels**: infinite queues exhaust RAM silently. Bound every queue and handle full as an error.
- **`println!` on firmware**: full format strings bloat flash. Use `defmt` indices plus a probe reader.

## Best Practices

- `heapless` for storage, `embassy` channels for tasks, `defmt` for logs — the standard trio covers most firmware.
- Prototype queue logic on host with asserts; move to the board only when green.

## Exercise

Push `10, 20, 30` into a fresh queue, pop one, push `40`, then pop everything and print the sequence.

## Solution

```rust
struct FixedQueue { buf: [u8; 4], head: usize, len: usize }
impl FixedQueue {
    fn new() -> Self { Self { buf: [0; 4], head: 0, len: 0 } }
    fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.len == self.buf.len() { return Err("full"); }
        self.buf[(self.head + self.len) % self.buf.len()] = byte; self.len += 1; Ok(())
    }
    fn pop(&mut self) -> Option<u8> {
        if self.len == 0 { return None; }
        let byte = self.buf[self.head]; self.head = (self.head + 1) % self.buf.len(); self.len -= 1; Some(byte)
    }
}
fn main() {
    let mut q = FixedQueue::new();
    for b in [10, 20, 30] { q.push(b).expect("space"); }
    assert_eq!(q.pop(), Some(10)); q.push(40).expect("space");
    let mut out = Vec::new(); while let Some(b) = q.pop() { out.push(b); }
    println!("{out:?}");
}
```

Prints `[20, 30, 40]`: FIFO order survives the wrap-around.

## Mini Quiz

```yaml
- question: "What does heapless provide?"
  options:
    - "A garbage collector"
    - "Fixed-capacity collections that error instead of allocating"
    - "Faster internet"
    - "An OS kernel"
  answer: 1
  explanation: "Array-backed Vec and Queue work with core alone."
- question: "What is embassy for?"
  options:
    - "Package publishing"
    - "Async tasks, timers, and channels without an OS"
    - "Web hosting"
    - "CPU manufacturing"
  answer: 1
  explanation: "Executors plus primitives structure concurrent firmware."
- question: "Why defmt instead of println! on firmware?"
  options:
    - "println! is forbidden syntax"
    - "Tiny indices save flash and probe bandwidth"
    - "defmt runs faster on PCs"
    - "No reason"
  answer: 1
  explanation: "Full format strings are heavy; indices decode on the host."
```
