---
slug: aliases-const-static
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: user-types
chapterTitle: "Chapter 6 — User-Defined Types"
chapterOrder: 6
title: "Type Aliases, Const, Static"
difficulty: beginner
order: 4
estimatedMinutes: 10
tags: [rust, types, const, static]
---

## Learning Objective

By the end of this lesson, you will shorten types with aliases and choose `const` vs `static`.

## Explanation

A **type alias** (`type Floor = i32;`) is a second name for an existing
type. Alias and original interchange freely — readability only, no checking
(`Floor` signals intent where `i32` does not).

A **`const`** is inlined at each use, needs an explicit type
(`const MAX: u32`) and a compile-time initializer. A **`static`** is one
memory location for the whole run, with a borrowable address. Use `const`
for values; `static` for shared state needing one home. Both use
`SCREAMING_SNAKE_CASE`.

## Why It Matters

Aliases add meaning without new types; `const` copies per use while
`static` shares one home — pick by identity needs.

## Example

`Floor` aliases `i32`, `DIGEST_SIZE` sizes buffers, `BANNER` shares one string.

## Code

```rust
type Floor = i32;
const DIGEST_SIZE: usize = 3;
static BANNER: &str = "Welcome to RustOS";

fn main() {
    let floor: Floor = 2;
    let buf = [0_u8; DIGEST_SIZE];
    println!("{BANNER} floor {floor} len {}", buf.len());
}
```

## Code Explanation

- `type Floor = i32;`: same type, clearer intent for `let floor: Floor = 2;`.
- `const DIGEST_SIZE: usize = 3;` inlines `3`; `usize` suits lengths and indexes.
- `[0_u8; DIGEST_SIZE]` needs a compile-time length, so `const` fits.
- `static BANNER: &str` is one shared string; prints `Welcome to RustOS floor 2 len 3`.

## Common Mistakes

- **Expecting safety from aliases**: `Floor` accepts any `i32`,
  including negatives. For validation, use a newtype instead.
- **Using `static mut` casually**: mutable globals need `unsafe`
  (explicit opt-out of checks) and risk races. Prefer `const` or a
  `Mutex` in `static` for shared mutation.
- **Omitting `const` types**: `const X = 3;` fails; constants require
  `: Type` (unlike `let` inference).

## Best Practices

- Alias complex or domain types at module top to shorten signatures.
- Prefer `const` for math, sizes, and defaults; reserve `static` for
  identity (shared tables, banners, locks).
- Name both in `SCREAMING_SNAKE_CASE` and keep values near their
  first use for discoverability.

## Exercise

Alias `Meters` for `f64`, add `const STEP: f64 = 0.5;`, and print a
two-step distance. When would `static` fit instead?

```rust
fn main() {
    let d: f64 = 1.0;
    println!("{d}");
}
```

Introduce the alias and const, then describe a banner-like counter
case needing `static`.

## Solution

Alias plus const keep meaning and size explicit:

```rust
type Meters = f64;
const STEP: Meters = 0.5;

fn main() {
    let d: Meters = STEP + STEP;
    println!("distance: {d}");
}
```

It prints `distance: 1`. Use `static` when one shared home matters
(such as a global banner string or a lock every thread touches),
not for plain copyable numbers like steps.

## Mini Quiz

```yaml
- question: "What does `type Floor = i32` create?"
  options:
    - "A new checked type distinct from i32"
    - "An interchangeable nickname for i32"
    - "A constant value 32"
    - "A new crate"
  answer: 1
  explanation: "Aliases aid reading; only newtypes add type checking."
- question: "When should you prefer `const`?"
  options:
    - "When one shared address is required"
    - "For inlined compile-time values like sizes"
    - "For mutable global state"
    - "Never, always use static"
  answer: 1
  explanation: "const values inline at each use and need no identity."
- question: "What distinguishes `static`?"
  options:
    - "It is always mutable"
    - "It is one memory location for the program run"
    - "It needs no type annotation"
    - "It lives only inside functions"
  answer: 1
  explanation: "statics have identity and address, suiting shared tables and locks."
```
