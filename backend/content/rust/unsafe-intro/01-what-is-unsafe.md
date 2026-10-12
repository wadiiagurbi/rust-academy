---
slug: unsafe-intro
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: unsafe-intro
chapterTitle: "Chapter 21 — Unsafe Rust Intro"
chapterOrder: 21
title: "What Is Unsafe Rust?"
difficulty: intermediate
order: 1
estimatedMinutes: 10
tags: [rust, unsafe, safety]
---

## Learning Objective

By the end of this lesson, you will state what `unsafe` permits and fence one operation behind a documented invariant.

## Explanation

**Safe Rust** is the default: the compiler rejects data races, dangling references, and out-of-bounds access. **`unsafe`** opens five extra operations — dereferencing raw pointers, calling `unsafe` functions, touching `static mut` globals, implementing `unsafe` traits, accessing union fields. It does *not* switch off the borrow checker; you take responsibility for those five. Each block owes a **safety invariant** (a comment stating what makes it sound, e.g. "pointer is valid and aligned"). A wrong invariant means **undefined behaviour** (the program's meaning is void — anything may happen, anywhere). Use `unsafe` only at FFI, hot loops, or abstractions whose proof the compiler cannot see.

## Why It Matters

OS kernels, drivers, and C libraries live behind `unsafe`. Tiny documented cores wrapped in safe APIs beat fearing the keyword.

## Example

Bump a counter through a raw pointer — one unverifiable operation, fenced and justified.

## Code

```rust
fn main() {
    let mut score = 41;
    let pointer: *mut i32 = &mut score;
    unsafe {
        // SAFETY: `pointer` comes from `&mut score`: valid, aligned,
        // and the only active path to `score` right now.
        *pointer += 1;
    }
    println!("score: {score}");
}
```

## Code Explanation

- `*mut i32` is a **mutable raw pointer**: an address with no borrow rules (may dangle or alias).
- Creating it is safe; *dereferencing* needs `unsafe`. Prints `score: 42`.

## Common Mistakes

- **Giant blocks**: fence the single operation so readers see which line needs permission.
- **No SAFETY comment**: uncommented `unsafe` forces re-deriving the proof. State it where risk is taken.
- **Assuming checks off**: borrow errors still fire inside `unsafe`. It adds permissions, removes almost none.

## Best Practices

- One operation per block, each with a `// SAFETY:` comment (validity, alignment, aliasing, lifetime).
- Expose a safe wrapper so callers never write `unsafe` themselves.

## Exercise

Start `score` at 99, bump by 5. Predict the output. Then point at `std::ptr::null_mut()` and run — what failure class is this?

## Solution

```rust
fn bump(score: &mut i32, amount: i32) {
    let pointer: *mut i32 = score;
    unsafe {
        // SAFETY: `pointer` borrows `score` for this call: valid,
        // aligned, unaliased for the duration.
        *pointer += amount;
    }
}

fn main() {
    let mut score = 99;
    bump(&mut score, 5);
    println!("score: {score}");
}
```

Prints `score: 104`; `main` stays safe. Null dereference is undefined behaviour — segfault, corruption, or false calm; never shippable.

## Mini Quiz

```yaml
- question: "What does `unsafe` allow?"
  options:
    - "Everything; checks off"
    - "Five extra operations; other checks still apply"
    - "Faster arithmetic only"
    - "Skipping types"
  answer: 1
  explanation: "Permissions added, verification mostly kept."
- question: "What is a safety invariant?"
  options:
    - "A CI test"
    - "Stated facts (validity, aliasing, lifetime) making it sound"
    - "A license header"
    - "A deprecation"
  answer: 1
  explanation: "The SAFETY comment records the proof for reviewers."
- question: "Undefined behaviour means?"
  options:
    - "A documented error"
    - "Program meaning void — anything may happen, anywhere"
    - "A silent panic"
    - "Slow code"
  answer: 1
  explanation: "Broken contracts let the compiler assume impossibilities."
```
