---
slug: raw-pointers
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: unsafe-intro
chapterTitle: "Chapter 21 — Unsafe Rust Intro"
chapterOrder: 21
title: "Raw Pointers"
difficulty: intermediate
order: 2
estimatedMinutes: 12
tags: [rust, unsafe, pointers]
---

## Learning Objective

By the end of this lesson, you will create `*const`/`*mut` pointers, dereference them in `unsafe`, and contrast them with references.

## Explanation

A **raw pointer** (`*const T` read-only, `*mut T` writable) is a bare address: no lifetime, no borrow rules, possibly null or dangling. Creation is safe (`&value as *const i32`); **dereference** (`*ptr`, following the address) needs `unsafe` because validity is your claim. A **reference** (`&T`/`&mut T`) is always valid, aligned, and alias-checked; a raw pointer trades every guarantee for control (data structures, hardware, FFI). Reading freed memory (a **dangling pointer**) or writing through aliases are the classic routes to undefined behaviour.

## Why It Matters

Raw pointers are the vocabulary of every unsafe boundary — lists, allocators, drivers, FFI. The const/mut split plus the dereference rule keeps foundations sound.

## Example

One sensor reading, two views: a `*const` observer and a `*mut` calibrator.

## Code

```rust
fn main() {
    let mut reading = 20;
    let observer: *const i32 = &reading as *const i32;
    let calibrator: *mut i32 = &mut reading;

    unsafe {
        // SAFETY: both derive from `reading`: alive, aligned,
        // exclusively accessed in this scope.
        println!("observed: {}", *observer);
        *calibrator += 2;
    }
    println!("calibrated: {reading}");
}
```

## Code Explanation

- Both creations are safe; only the reads/writes inside `unsafe` need permission.
- Output: `observed: 20`, then `calibrated: 22`.

## Common Mistakes

- **Dereference outside `unsafe`**: fails with `dereference of raw pointer requires unsafe`. The fence is mandatory.
- **Outliving the owner**: using the pointer after `reading` drops is dangling — scope lifetimes manually.
- **Aliased `*mut` writes**: writing through the pointer while a `&reading` borrow lives breaks aliasing rules. Sequence accesses.

## Best Practices

- Prefer references; introduce raw pointers only where addresses are demanded.
- Keep the owner visibly alive for the whole `unsafe` region.
- Name pointers by role (`observer`/`calibrator`) so const/mut reads as intent.

## Exercise

Add `*calibrator += 3;` in the block. Final value? Explain why `observer` + `calibrator` coexist but `&` + `&mut` would not compile.

## Solution

```rust
fn main() {
    let mut reading = 20;
    let observer: *const i32 = &reading as *const i32;
    let calibrator: *mut i32 = &mut reading;

    unsafe {
        // SAFETY: `reading` alive and aligned; raw pointers carry
        // no borrow state, so coexistence is allowed.
        println!("observed: {}", *observer);
        *calibrator += 2;
        *calibrator += 3;
    }
    println!("calibrated: {reading}");
}
```

Final value 25. The compiler tracks no borrows for raw pointers (your burden); references carry enforced aliasing rules.

## Mini Quiz

```yaml
- question: "Which raw-pointer op needs `unsafe`?"
  options:
    - "Creating from a reference"
    - "Dereferencing (reading/writing through it)"
    - "Copying the pointer"
    - "Comparing pointers"
  answer: 1
  explanation: "Creation and copying are safe; following the address is not."
- question: "Pointer vs reference?"
  options:
    - "Identical"
    - "References guarantee validity; raw pointers guarantee nothing"
    - "Pointers are always null"
    - "References can dangle"
  answer: 1
  explanation: "References are verified; raw pointers are unchecked addresses."
- question: "What is a dangling pointer?"
  options:
    - "Address of freed memory — use is undefined behaviour"
    - "A large allocation"
    - "A null constant"
    - "A loop pointer"
  answer: 0
  explanation: "Use-after-free reads or corrupts unrelated memory."
```
