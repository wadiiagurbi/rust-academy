---
slug: maybeuninit
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: unsafe-deep
chapterTitle: "Chapter 24 — Unsafe Deep Dive"
chapterOrder: 24
title: "MaybeUninit Without UB"
difficulty: advanced
order: 3
estimatedMinutes: 12
tags: [rust, unsafe, maybeuninit]
---

## Learning Objective

By the end of this lesson, you will initialise a buffer with **MaybeUninit** (a wrapper allowed to hold uninitialised memory) and convert it to initialised exactly once.

## Explanation

Reading uninitialised memory is UB, even for integers — the compiler may assume it never happens. `MaybeUninit<T>` is a **wrapper type** exempt from that assumption: you may allocate it, then write each slot with `write` (a move needing no prior value, unlike assignment which would drop garbage). Track completion yourself; only when every slot is written, read each back with `assume_init_read` (the unchecked claim "this slot is initialised"). Never read before writing, never `write` twice without dropping.

## Why It Matters

FFI out-parameters, ring buffers, and decoders fill memory the compiler cannot see. `MaybeUninit` is the only sound staging area for that pattern.

## Example

Build `[64, 128, 192]` in uninitialised backing, then bless it initialised.

## Code

```rust
use std::mem::MaybeUninit;

fn build() -> [u32; 3] {
    let mut buf: [MaybeUninit<u32>; 3] = [MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit()];
    for (i, slot) in buf.iter_mut().enumerate() {
        slot.write((i as u32 + 1) * 64);
    }
    // SAFETY: every slot 0..3 written exactly once above.
    unsafe { [buf[0].assume_init_read(), buf[1].assume_init_read(), buf[2].assume_init_read()] }
}

fn main() {
    println!("{:?}", build());
}
```

## Code Explanation

- `MaybeUninit::uninit()` allocates without initialising; `write` moves a value into each slot, and `assume_init_read` reads each back. Prints `[64, 128, 192]`.

## Common Mistakes

- **Reading too early**: `assume_init_read` before the last write blesses garbage — reads become UB even if "overwritten later".
- **Assigning instead of writing**: `*ptr = v` drops the old (garbage) value first. Use `ptr::write`, which skips the drop.
- **Panic gaps**: panicking mid-fill then assuming init skips slots. Initialise in infallible loops or guard with a written-count.

## Best Practices

- Write in index order with `enumerate`; keep the SAFETY comment citing the covering loop, and bless each slot exactly once.

## Exercise

Write `fn zeros() -> [u32; 4]` producing `[0, 0, 0, 0]` via `MaybeUninit`, then assert each element equals 0 in `main`.

## Solution

```rust
use std::mem::MaybeUninit;

fn zeros() -> [u32; 4] {
    let mut buf: [MaybeUninit<u32>; 4] = [MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit()];
    for slot in buf.iter_mut() {
        slot.write(0);
    }
    // SAFETY: all four slots written exactly once above.
    unsafe { [buf[0].assume_init_read(), buf[1].assume_init_read(), buf[2].assume_init_read(), buf[3].assume_init_read()] }
}

fn main() {
    let z = zeros();
    assert_eq!(z, [0, 0, 0, 0]);
    println!("{z:?}");
}
```

Prints `[0, 0, 0, 0]`: every slot proven written before the blessing.

## Mini Quiz

```yaml
- question: "Why not use plain [u32; 3] as scratch space?"
  options:
    - "It is slower"
    - "Reading or dropping uninitialised values is UB"
    - "Arrays cannot be mutated"
    - "It needs nightly"
  answer: 1
  explanation: "MaybeUninit lifts the initialised assumption until you bless it."
- question: "Why slot.write(v) instead of *ptr = v?"
  options:
    - "Shorter syntax"
    - "Assignment drops garbage first; write moves without reading"
    - "write is faster always"
    - "No reason"
  answer: 1
  explanation: "Dropping uninitialised memory is itself UB."
- question: "When is assume_init_read legal?"
  options:
    - "Any time"
    - "Only after that slot is written exactly once"
    - "Before writing"
    - "Only for u8"
  answer: 1
  explanation: "Each read claims initialisation — prove the write first."
```
