---
slug: rust-android
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: beyond-backend
chapterTitle: "Chapter 26 — Rust Beyond the Backend"
chapterOrder: 26
title: "Rust on Android"
difficulty: advanced
order: 1
estimatedMinutes: 12
tags: [rust, android, ffi]
---

## Learning Objective

By the end of this lesson, you will describe how Rust reaches Android through a **`cdylib`** (a C-compatible shared library) and write the exported functions behind it.

## Explanation

Android apps run Kotlin/Java on the **ART runtime**, but CPU-heavy code (codecs, crypto,_ml) can live in native libraries. Rust compiles to a `cdylib` (set `crate-type = ["cdylib"]` in `Cargo.toml`) exposing **`extern "C"` functions** (no name mangling, C calling convention) that Java reaches via **JNI** (Java Native Interface: the bridge mapping Java methods to native symbols) or directly through the **NDK** (native development kit: the C/C++ toolchain for Android). The program below compiles on host as plain Rust; the OPTIONAL checkout steps need AOSP sources and a device. OPTIONAL: in an AOSP checkout, add the crate under `external/`, wire `Android.bp`, and call it from a `System.loadLibrary` activity.

## Why It Matters

Shipping one audited Rust core inside a mobile app cuts battery use and native-crash classes while Kotlin keeps the UI layer.

## Example

Two exported routines — version string length and clamped mixing — shaped exactly as JNI-bound code would be.

## Code

```rust
/// Returns the length of the embedded version string.
#[no_mangle]
pub extern "C" fn rust_version_len() -> u32 {
    "1.0.0".len() as u32
}

/// Mixes two volumes, clamping the result to 0..=100.
#[no_mangle]
pub extern "C" fn rust_mix(a: u32, b: u32) -> u32 {
    (a + b).min(100)
}

fn main() {
    println!("{} {}", rust_version_len(), rust_mix(70, 50));
}
```

## Code Explanation

- `#[no_mangle]` keeps the symbol name intact so Java/JNI finds it; `extern "C"` fixes the calling convention. Only integer types cross here — no slices or `String`.
- Prints e.g. `5 100`: lengths and clamped sums are trivially checkable before wiring Java.

## Common Mistakes

- **Rich types over FFI**: passing `String` or slices to Java corrupts memory. Cross with integers/handles; copy buffers explicitly.
- **Panics crossing languages**: a panic through JNI aborts the process. Catch with `catch_unwind` and return error codes.
- **Skipping the host harness**: wiring Java first makes debugging slow. Prove every export in a host `main` like above.

## Best Practices

- Export few, total functions (no panics, no alloc across the boundary); document ownership of every buffer.
- OPTIONAL device work (AOSP checkout, `Android.bp`, on-device runs) comes after host tests pass.

## Exercise

Add `#[no_mangle] pub extern "C" fn rust_add_sat(a: u32, b: u32) -> u32` using `saturating_add`. Assert `rust_add_sat(u32::MAX, 1) == u32::MAX` in `main`.

## Solution

```rust
#[no_mangle]
pub extern "C" fn rust_add_sat(a: u32, b: u32) -> u32 {
    a.saturating_add(b)
}

fn main() {
    assert_eq!(rust_add_sat(u32::MAX, 1), u32::MAX);
    println!("{} {}", rust_add_sat(2, 3), rust_add_sat(u32::MAX, 1));
}
```

Prints `5 4294967295`: saturation keeps overflow defined at the language edge.

## Mini Quiz

```yaml
- question: "What is a cdylib?"
  options:
    - "A Rust test binary"
    - "A C-compatible shared library other languages load"
    - "An Android emulator"
    - "A Java archive"
  answer: 1
  explanation: "crate-type cdylib exposes extern C symbols for JNI/NDK."
- question: "Why only integers/handles across JNI?"
  options:
    - "Java dislikes numbers"
    - "Rich Rust types have layouts Java cannot uphold"
    - "Integers are faster to type"
    - "Strings are banned"
  answer: 1
  explanation: "Copy explicitly; never share ownership across the boundary."
- question: "Why is the AOSP checkout marked OPTIONAL?"
  options:
    - "It is unimportant"
    - "It needs sources, toolchain, and device; concepts come first"
    - "It never works"
    - "Hosts cannot compile Rust"
  answer: 1
  explanation: "Rust-side code is fully testable on host before device work."
```
