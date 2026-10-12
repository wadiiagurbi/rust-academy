---
slug: no-std
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: bare-metal
chapterTitle: "Chapter 25 — Bare-Metal Rust"
chapterOrder: 25
title: "Going no-std"
difficulty: advanced
order: 1
estimatedMinutes: 12
tags: [rust, no-std, embedded]
---

## Learning Objective

By the end of this lesson, you will state what `no_std` removes and write allocation-free logic that compiles in both worlds.

## Explanation

`std` is the hosted library (heap, threads, files, OS calls). `#![no_std]` drops it, keeping **`core`** (platform-free basics) plus optionally **`alloc`** (heap containers behind a provided allocator). Without an OS there is no `main` or default panic handler: firmware defines its own **entry point** and a **`#[panic_handler]`** (called on panic, usually halting). That shape needs an embedded target and will not link on a host PC:

```text
EXCERPT-ONLY (needs an embedded target; will not link on host):
#![no_std]
#![no_main]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
```

## Why It Matters

Microcontrollers have kilobytes of RAM and no OS. `no_std` plus `core`-only code is how one language spans servers and sensors.

## Example

A fixed-capacity log that never allocates — the same code compiles on host and on firmware.

## Code

```rust
struct FixedLog { buf: [u8; 8], len: usize }
impl FixedLog {
    fn new() -> Self { Self { buf: [0; 8], len: 0 } }
    fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.len >= self.buf.len() { return Err("log full"); }
        self.buf[self.len] = byte; self.len += 1; Ok(())
    }
}
fn main() {
    let mut log = FixedLog::new();
    log.push(b'h').expect("space"); log.push(b'i').expect("space");
    println!("{:?}", &log.buf[..log.len]);
    let r = log.push(b'!');
    println!("{:?}", r);
}
```

## Code Explanation

- `[u8; 8]` is a **fixed array** (eight bytes inline, no heap) using only `core` features (arrays, slices, `Result`); `&'static str` errors need no allocator, hence firmware-compatible.

## Common Mistakes

- **Reaching for `Vec`/`String`**: both need `alloc` plus an allocator firmware may lack. Start with arrays and slices.
- **Forgetting `#[panic_handler]`**: `no_std` binaries fail to link without one in the firmware crate root.
- **Testing only on firmware**: flashing for every check is slow. Keep logic in `core`-only modules tested on host.

## Best Practices

- Write drivers against `core` only; gate `alloc`/`std` helpers behind Cargo features.
- Return `Result<_, &'static str>` from fallible fixed-size operations — no formatting machinery required.

## Exercise

Add `fn clear(&mut self)` resetting `len`, and `fn is_full(&self) -> bool`. Fill the log, assert full, clear, assert empty.

## Solution

```rust
struct FixedLog { buf: [u8; 8], len: usize }
impl FixedLog {
    fn new() -> Self { Self { buf: [0; 8], len: 0 } }
    fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.is_full() { return Err("log full"); }
        self.buf[self.len] = byte; self.len += 1; Ok(())
    }
    fn clear(&mut self) { self.len = 0; } fn is_full(&self) -> bool { self.len >= self.buf.len() }
}
fn main() {
    let mut log = FixedLog::new();
    for _ in 0..8 { log.push(b'a').expect("space"); }
    assert!(log.is_full()); log.clear(); assert_eq!(log.len, 0); println!("ok");
}
```

Prints `ok`: bounded storage with explicit full/empty states, no heap involved.

## Mini Quiz

```yaml
- question: "What does #![no_std] remove?"
  options:
    - "The compiler"
    - "std (OS-backed services); core remains"
    - "All types"
    - "Unsafe code"
  answer: 1
  explanation: "core stays available; heap, threads, and files need alloc or an OS."
- question: "What must a no_std binary also provide?"
  options:
    - "A garbage collector"
    - "An entry point and a panic handler"
    - "A web server"
    - "A test suite"
  answer: 1
  explanation: "No OS means no default main or panic behaviour."
- question: "Why do embedded types use fixed arrays?"
  options:
    - "They are faster to type"
    - "No heap or allocator is needed"
    - "Slices are forbidden"
    - "Arrays use no RAM"
  answer: 1
  explanation: "Inline storage works with core alone on tiny devices."
```
