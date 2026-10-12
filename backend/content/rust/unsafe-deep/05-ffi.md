---
slug: ffi-basics
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: unsafe-deep
chapterTitle: "Chapter 24 — Unsafe Deep Dive"
chapterOrder: 24
title: "FFI Basics and Safe Wrappers"
difficulty: advanced
order: 5
estimatedMinutes: 15
tags: [rust, ffi, unsafe]
---

## Learning Objective

By the end of this lesson, you will declare a foreign function, call it inside `unsafe`, and expose a total safe wrapper.

## Explanation

**FFI** (foreign function interface) calls code written in another language, usually C. An **`extern "C"` block** declares the foreign signature under the C **ABI** (application binary interface: the low-level calling convention for passing arguments) — trusted blindly, linked from the system C library. Calling it is `unsafe` because Rust cannot verify the other side. A **safe wrapper** is a normal `fn` that shrinks inputs to the provably-valid set (here: everything except `i32::MIN`, whose negation overflows) and holds the single `unsafe` call plus SAFETY comment.

## Why It Matters

OS calls, codecs, and drivers enter Rust through FFI. One audited wrapper per function keeps every other call site safe and reviewable.

## Example

C's `abs` (absolute value for 32-bit integers), wrapped so `main` never writes `unsafe`.

## Code

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

/// Absolute value via the C library (all inputs except `i32::MIN`).
fn safe_abs(value: i32) -> i32 {
    // SAFETY: C `abs` is pure with no shared state; the `i32::MIN`
    // overflow edge is excluded by the caller's contract below.
    unsafe { abs(value) }
}

fn main() {
    println!("{}", safe_abs(-9));
}
```

## Code Explanation

- The `unsafe extern` declaration is an unchecked claim — copy the signature from official headers exactly. Prints `9`.
- Purity (no globals touched) plus the input limit is the whole proof; anything wider needs more clauses.

## Common Mistakes

- **Signature mismatch**: declaring `u64` where C takes `i32` corrupts the stack silently. Match the header type-for-type.
- **Leaking `unsafe`**: `unsafe { abs(x) }` at every call site spreads audit load. Wrap once, call safely.
- **Ignoring edge inputs**: `abs(i32::MIN)` overflows in C too. Name and handle the edge in the wrapper.

## Best Practices

- One wrapper per foreign function; document valid inputs, thread safety, and who frees returned pointers.
- Keep `unsafe extern` blocks minimal — declarations only, no Rust logic beside them.

## Exercise

Harden `safe_abs` to saturate: return `i32::MAX` for `i32::MIN` without calling C. Print `safe_abs(-9)` and `safe_abs(i32::MIN)`.

## Solution

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

/// Absolute value via the C library, saturating at `i32::MIN`.
fn safe_abs(value: i32) -> i32 {
    if value == i32::MIN {
        return i32::MAX;
    }
    // SAFETY: pure with no shared state; `i32::MIN` excluded
    // above, so the C overflow edge is unreachable.
    unsafe { abs(value) }
}

fn main() {
    println!("{} {}", safe_abs(-9), safe_abs(i32::MIN));
}
```

Prints `9 2147483647`: the guard converts UB into documented saturation.

## Mini Quiz

```yaml
- question: "What does extern \"C\" declare?"
  options:
    - "A Rust module"
    - "Foreign fns under the C ABI, trusted unverified"
    - "A test suite"
    - "Compiler intrinsics"
  answer: 1
  explanation: "Signature and convention only; the foreign side is your claim."
- question: "Why is calling it unsafe?"
  options:
    - "It is slow"
    - "Rust cannot verify the foreign implementation"
    - "It needs nightly"
    - "It allocates"
  answer: 1
  explanation: "The compiler checks stop at the language boundary."
- question: "What belongs in the safe wrapper?"
  options:
    - "Raw passthrough of all inputs"
    - "Input limits plus one audited unsafe call"
    - "A second FFI call"
    - "Nothing; wrappers are optional"
  answer: 1
  explanation: "Narrowed inputs make the call provably valid for all callers."
```
