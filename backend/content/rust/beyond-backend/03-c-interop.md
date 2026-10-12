---
slug: c-interop
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: beyond-backend
chapterTitle: "Chapter 26 — Rust Beyond the Backend"
chapterOrder: 26
title: "C Interop: Both Directions"
difficulty: intermediate
order: 3
estimatedMinutes: 12
tags: [rust, c, ffi]
---

## Learning Objective

By the end of this lesson, you will call C from Rust and expose Rust to C using **`extern "C"`** (the C calling convention) in both directions.

## Explanation

C interop has two halves. **Calling C** means declaring the foreign signature in an `unsafe extern "C"` block (a trusted, unverified claim linked from the system C library) and invoking it inside `unsafe` with a SAFETY comment. **Exposing Rust** means marking a function `#[no_mangle] pub extern "C"` (symbol name preserved, C-callable) that takes only **FFI-safe types** (integers, pointers — never `String`, `Vec`, or slices, whose layouts C cannot uphold) and never panics. Both halves meet in safe wrappers: check on the way in, convert on the way out.

## Why It Matters

Every OS API, sensor SDK, and game engine plugin speaks C. Both directions together let Rust reuse that world and be reused by it.

## Example

Import C's `abs`, wrap it safely, and export a saturating adder back to C.

## Code

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn c_abs(value: i32) -> i32 {
    // SAFETY: C `abs` is pure with no shared state; valid for
    // every `i32` except `i32::MIN`, excluded by callers here.
    unsafe { abs(value) }
}

/// Saturating addition, callable from C.
#[no_mangle]
pub extern "C" fn rust_add_sat(a: i32, b: i32) -> i32 {
    a.saturating_add(b)
}

fn main() {
    println!("{} {}", c_abs(-4), rust_add_sat(2_000_000_000, 2_000_000_000));
}
```

## Code Explanation

- `c_abs` is the **import wrapper** (checked call into C); `rust_add_sat` is the **export** (C-callable, integers only, panic-free via saturation).
- Prints `4 2147483647`: absolute value one way, saturated sum the other.

## Common Mistakes

- **FFI-unsafe exports**: `pub extern "C" fn f(s: String)` corrupts across the boundary. Export integers/pointers; copy buffers explicitly.
- **Panics escaping**: an escaping panic through C is undefined behaviour. Use saturating/wrapping ops and error codes.
- **Signature drift**: a stale declaration (wrong width or sign) corrupts silently. Re-check against headers per release.

## Best Practices

- One wrapper per foreign function; one export per capability; document valid inputs and error codes on each.
- Test both directions from a host `main` before wiring real C callers.

## Exercise

Export `rust_mul_sat(a: i32, b: i32) -> i32` with `saturating_mul`. Print `rust_mul_sat(100_000, 100_000)` and `c_abs(-12)`.

## Solution

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn c_abs(value: i32) -> i32 {
    // SAFETY: pure with no shared state; `i32::MIN` excluded
    // by the exercise inputs below.
    unsafe { abs(value) }
}

#[no_mangle]
pub extern "C" fn rust_mul_sat(a: i32, b: i32) -> i32 {
    a.saturating_mul(b)
}

fn main() {
    println!("{} {}", rust_mul_sat(100_000, 100_000), c_abs(-12));
}
```

Prints `2147483647 12`: saturation holds on export, C answers on import.

## Mini Quiz

```yaml
- question: "What can cross an extern C boundary?"
  options:
    - "Any Rust type"
    - "Only FFI-safe types like integers and pointers"
    - "Only generics"
    - "Only closures"
  answer: 1
  explanation: "Layouts C cannot uphold must never cross; copy instead."
- question: "Why must exports never panic?"
  options:
    - "Panics are slow"
    - "Escaping panics through C are undefined behaviour"
    - "C catches them silently"
    - "Exports cannot return values"
  answer: 1
  explanation: "Use saturating ops and error codes at the seam."
- question: "What does unsafe extern declare?"
  options:
    - "A Rust implementation"
    - "A trusted, unverified foreign signature"
    - "A macro"
    - "A test"
  answer: 1
  explanation: "The body lives abroad; the declaration is your claim."
```
