---
slug: module-visibility
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: modules
chapterTitle: "Chapter 18 — Modules"
chapterOrder: 18
title: "Files and Visibility"
difficulty: beginner
order: 2
estimatedMinutes: 12
tags: [rust, modules, visibility, files]
---

## Learning Objective

By the end of this lesson, you will map modules to files and gate access with `pub` and `pub(crate)`.

## Explanation

`mod vault;` (semicolon, no body) loads the module from `vault.rs` (or `vault/mod.rs`) beside the current file; `mod shelves;` inside `vault.rs` loads `vault/shelves.rs`. **Visibility** decides callers: private (default) means only the defining module and descendants; `pub` opens to everyone; `pub(crate)` (public-in-crate) opens crate-wide but hides the item from downstream packages. Idiom: keep helpers private, expose one `pub` function calling them — a clean API over freely changeable internals.

## Why It Matters

Private-by-default keeps refactoring safe; deliberate `pub` boundaries mark what others may depend on.

## Example

A `vault` exposing `open()` while hiding `combination()` — identical inline or split across files.

## Code

```rust
mod vault {
    pub fn open() -> &'static str {
        let _code = combination();
        "vault open"
    }

    fn combination() -> u32 {
        1234
    }

    pub(crate) fn audit() -> &'static str {
        "audit logged"
    }
}

fn main() {
    println!("{}", vault::open());
    println!("{}", vault::audit());
}
```

## Code Explanation

- `open()` is `pub`; it may use private `combination()`, which outsiders can never name.
- `audit()` is `pub(crate)`: callable in this crate, invisible to downstream crates.

## Common Mistakes

- **File without declaration**: a stray `vault.rs` with no `mod vault;` is ignored by the compiler.
- **Wrong nesting path**: `mod shelves;` in `vault.rs` loads `vault/shelves.rs`, not root `shelves.rs`.
- **Everything `pub`**: freezing internals into API. Start private, widen only on need.

## Best Practices

- Keep roots (`main.rs`/`lib.rs`) as tables of contents: `mod` lines plus re-exports.
- Hide helpers; use `pub(crate)` for cross-module plumbing, `pub` for real API.

## Exercise

Split the program into `main.rs` (`mod vault;` + `main`) and `vault.rs` (the body). Then call `vault::combination()` from `main` — record the exact error.

## Solution

`src/main.rs` holds `mod vault;` plus `main` (calling `open` and `audit`); `src/vault.rs` holds the three functions unwrapped:

```rust
pub fn open() -> &'static str {
    let _code = combination();
    "vault open"
}

fn combination() -> u32 {
    1234
}

pub(crate) fn audit() -> &'static str {
    "audit logged"
}
```

The forbidden call fails with `error[E0603]: function combination is private` — privacy holds across files.

## Mini Quiz

```yaml
- question: "What does `mod vault;` do?"
  options:
    - "Deletes the module"
    - "Loads its body from sibling `vault.rs`"
    - "Makes everything public"
    - "Runs its tests"
  answer: 1
  explanation: "The semicolon form maps a module to a file."
- question: "What does `pub(crate)` allow?"
  options:
    - "Use from any package"
    - "Use crate-wide, but not from downstream crates"
    - "Use only in the module"
    - "Use only in tests"
  answer: 1
  explanation: "pub(crate) widens to the crate while hiding public API."
- question: "Why keep helpers private?"
  options:
    - "Private code is faster"
    - "Internals stay refactorable without breaking callers"
    - "Public fns cannot call them"
    - "The compiler demands it"
  answer: 1
  explanation: "A small public surface keeps the API stable."
```
