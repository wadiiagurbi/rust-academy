---
slug: rust-chromium
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: beyond-backend
chapterTitle: "Chapter 26 — Rust Beyond the Backend"
chapterOrder: 26
title: "Rust in Chromium"
difficulty: advanced
order: 2
estimatedMinutes: 12
tags: [rust, chromium, ffi]
---

## Learning Objective

By the end of this lesson, you will describe Chromium's Rust boundary (**`rust_static_library`**: a GN rule compiling Rust into one linkable unit) and write the safe side of it.

## Explanation

**Chromium** is the open-source browser under Chrome, built mostly in C++ with **GN** (its meta-build system generating build files) orchestrating targets. New Rust code enters as a `rust_static_library` target: Rust compiles with the project's toolchain, C++ calls in through `extern "C"` shims, and `unsafe` stays quarantined at that seam. The pattern is a **checked boundary**: Rust validates every integer/enum from C++ before acting, returning error codes instead of panicking. The program below is the Rust side, host-compilable; the OPTIONAL part (a Chromium checkout, `gn gen`, `ninja`) needs the full tree. OPTIONAL: in a Chromium checkout, add the target, bind one C++ caller, and run the component test.

## Why It Matters

Parsers and codecs are historically the browser's riskiest C++; moving them to checked Rust shrinks the memory-safety attack surface measurably.

## Example

A parser entry point: validate the kind code, then answer from a table — no panics, no unchecked indexing.

## Code

```rust
/// Classifies a codec id from C++. Returns -1 for unknown ids.
#[no_mangle]
pub extern "C" fn rust_codec_class(id: i32) -> i32 {
    match id {
        1 | 2 => 10,
        3 => 20,
        _ => -1,
    }
}

fn main() {
    println!("{} {} {}", rust_codec_class(1), rust_codec_class(3), rust_codec_class(9));
}
```

## Code Explanation

- `match` is **exhaustive**: the `_` arm forces a defined answer for every unknown id, so C++ always gets a code, never UB.
- Prints `10 20 -1`: known ids classify, anything else is rejected explicitly.

## Common Mistakes

- **Panicking at the seam**: C++ cannot catch Rust panics — abort follows. Return error codes from every exported function.
- **Trusting C++ enums**: unvalidated integers index tables out of bounds. Match first, act second.
- **Wide boundaries**: exporting whole object graphs multiplies audit. Export few scalar functions with total behaviour.

## Best Practices

- Every export is total (defined for all inputs), panic-free, and documented with its error codes.
- Prove the Rust side on host with hostile inputs before touching GN or C++ callers.

## Exercise

Add `rust_is_lossless(id: i32) -> bool` returning true only for id `2`. Assert all four ids `1..=4` behave, and print the results.

## Solution

```rust
#[no_mangle]
pub extern "C" fn rust_codec_class(id: i32) -> i32 {
    match id {
        1 | 2 => 10,
        3 => 20,
        _ => -1,
    }
}

#[no_mangle]
pub extern "C" fn rust_is_lossless(id: i32) -> bool {
    id == 2
}

fn main() {
    assert_eq!(rust_codec_class(9), -1);
    assert!(rust_is_lossless(2));
    assert!(!rust_is_lossless(3));
    println!("{} {}", rust_codec_class(2), rust_is_lossless(2));
}
```

Prints `10 true`: classification and property queries stay total and panic-free.

## Mini Quiz

```yaml
- question: "What is rust_static_library in Chromium?"
  options:
    - "A JavaScript bundle"
    - "A GN rule compiling Rust into one linkable unit"
    - "A test browser"
    - "A CSS theme"
  answer: 1
  explanation: "GN builds the Rust crate; C++ links and calls its shims."
- question: "Why must exports be total and panic-free?"
  options:
    - "For speed"
    - "C++ cannot catch Rust panics; every input needs a defined answer"
    - "GN forbids errors"
    - "Browsers dislike numbers"
  answer: 1
  explanation: "Error codes cross the seam; panics would abort the process."
- question: "Why validate C++ integers first?"
  options:
    - "Integers are slow"
    - "Untrusted values must never index tables unchecked"
    - "Validation is optional style"
    - "C++ sends no integers"
  answer: 1
  explanation: "Match-then-act converts hostile inputs into error codes."
```
