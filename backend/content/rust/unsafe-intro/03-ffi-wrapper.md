---
slug: ffi-wrapper
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: unsafe-intro
chapterTitle: "Chapter 21 — Unsafe Rust Intro"
chapterOrder: 21
title: "Wrapping Foreign Functions"
difficulty: advanced
order: 3
estimatedMinutes: 15
tags: [rust, unsafe, ffi]
---

## Learning Objective

By the end of this lesson, you will declare an `extern` function, call it in `unsafe`, and hide it behind a safe wrapper.

## Explanation

**FFI** (Foreign Function Interface) is how Rust calls other languages, usually C. An **`extern "C"` block** declares the foreign function under the C calling convention (the low-level argument-passing protocol) — trusted without verification, linked from the system C library at build time. Calling it is `unsafe` because Rust cannot check the foreign side. The pattern is a **safe wrapper**: a normal `fn` whose inputs make misuse impossible, holding one small `unsafe` block plus SAFETY comment. Here we wrap C's `abs` (absolute value for 32-bit integers), being honest about its one edge: negating `i32::MIN` overflows, so the wrapper names that limit.

## Why It Matters

Drivers, OS calls, and SDKs enter Rust through FFI. The wrapper concentrates auditing into documented lines while callers stay safe.

## Example

C's `abs`, declared foreign, called once, wrapped so `main` never writes `unsafe`.

## Code

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

/// Absolute value via the C standard library (all inputs except `i32::MIN`).
fn safe_abs(value: i32) -> i32 {
    // SAFETY: C `abs` is pure with no shared state; valid for every `i32`
    // except `i32::MIN`, which callers must not pass (see exercise).
    unsafe { abs(value) }
}

fn main() { println!("{}", safe_abs(-9)); }
```

## Code Explanation

- The `unsafe extern` declaration is trusted blindly — a wrong signature corrupts silently, so copy it from official headers. `main` prints `9` without seeing `unsafe`.

## Common Mistakes

- **Signature mismatch**: declaring `u64` where C takes `i32` corrupts the stack with no compiler error. Match the header exactly.
- **Leaking `unsafe`**: every call site writing `unsafe` spreads audit burden. Wrap once, call safely.
- **Assuming purity**: C code may hold globals or non-thread-safe statics. Record reentrancy in the comment.

## Best Practices

- One wrapper per foreign function; document valid inputs, thread safety, and who frees returned pointers.

## Exercise

Harden `safe_abs`: return `i32::MAX` for `i32::MIN` instead of invoking overflow, and print both `safe_abs(-9)` and `safe_abs(i32::MIN)` from `main`.

## Solution

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

/// Absolute value via the C standard library, saturating at `i32::MIN`.
fn safe_abs(value: i32) -> i32 {
    if value == i32::MIN {
        return i32::MAX;
    }
    // SAFETY: pure with no shared state; `i32::MIN` excluded above,
    // so the C overflow edge is unreachable.
    unsafe { abs(value) }
}

fn main() { println!("{} {}", safe_abs(-9), safe_abs(i32::MIN)); }
```

Prints `9 2147483647`: the guard converts undefined behaviour into a documented saturation before the foreign call.

## Mini Quiz

```yaml
- question: "What does `extern \"C\"` declare?"
  options:
    - "A Rust module"
    - "Foreign fns, C convention, trusted unverified"
    - "A test suite"
    - "Intrinsics"
  answer: 1
  explanation: "Signature and convention only; the foreign side is your claim."
- question: "Why a safe wrapper?"
  options:
    - "Faster"
    - "One audited boundary; all other code stays safe"
    - "Otherwise will not compile"
    - "Avoids linking"
  answer: 1
  explanation: "Concentrated unsafety plus proof minimizes audit surface."
- question: "Mismatched declared signature?"
  options:
    - "Compiler error"
    - "Silent corruption — declarations are unchecked"
    - "Auto conversion"
    - "Signatures do not matter"
  answer: 1
  explanation: "FFI claims bypass verification; header accuracy is critical."
```
