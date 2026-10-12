---
slug: modules
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: modules
chapterTitle: "Chapter 18 — Modules"
chapterOrder: 18
title: "Modules"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, modules, organization]
---

## Learning Objective

By the end of this lesson, you will group code with `mod` and call items through paths like `garden::water()`.

## Explanation

A **module** (`mod`) is a named container grouping related items. Modules nest into a **module tree** addressed by **paths** (segments joined with `::`, e.g. `garden::tools::water()`). The tree root is the **crate root** (`main.rs` for binaries, `lib.rs` for libraries). `crate` starts a path at the root (**absolute path**, e.g. `crate::garden::water()`); without it the path resolves from the current module (**relative path**). Items are private to their parent by default; `pub` (public) opens them.

## Why It Matters

Modules are how Rust scales: each feature gets a namespace, collisions vanish, readers navigate by path.

## Example

A gardening app with nested `tools`, called through a relative and an absolute path.

## Code

```rust
mod garden {
    pub fn plant() -> &'static str {
        "planted a rose"
    }

    pub mod tools {
        pub fn water() -> &'static str {
            "watered the beds"
        }
    }
}

fn main() {
    println!("{}", garden::plant());
    println!("{}", crate::garden::tools::water());
}
```

## Code Explanation

- `mod garden { ... }` declares the module inline; big projects use `mod garden;` plus a `garden.rs` file instead.
- Without `pub`, the calls in `main` would fail with `function is private`.
- Both paths print their messages.

## Common Mistakes

- **Calling a private item**: fails with `function is private`. Fix: add `pub` or expose a `pub` wrapper.
- **Short path**: `tools::water()` from `main` fails (`unresolved import`); write `garden::tools::water()`.
- **File without `mod`**: a lone `garden.rs` is ignored until the parent declares `mod garden;`.

## Best Practices

- One responsibility per module; nest only true children (`garden::tools`).
- Use relative paths inside a module, `crate::` paths across distant branches.

## Exercise

Add `garden::pests` with `pub fn inspect()` returning `"no aphids"`, called via both path styles. What breaks if you drop `pub`?

## Solution

```rust
mod garden {
    pub mod pests {
        pub fn inspect() -> &'static str {
            "no aphids"
        }
    }
}

fn main() {
    println!("{}", garden::pests::inspect());
    println!("{}", crate::garden::pests::inspect());
}
```

Both print `no aphids`; without `pub` both fail with `function inspect is private`.

## Mini Quiz

```yaml
- question: "What is a module?"
  options:
    - "A cargo package"
    - "A named container grouping items, addressed via paths"
    - "A test block"
    - "An external library"
  answer: 1
  explanation: "`mod` makes a namespace; paths locate items."
- question: "What does leading `crate::` mean?"
  options:
    - "An external crate"
    - "Anchor the lookup at the crate root"
    - "The standard library"
    - "The current file"
  answer: 1
  explanation: "`crate::` starts resolution at the root from any module."
- question: "Are items public by default?"
  options:
    - "Yes, all public"
    - "No, private until marked `pub`"
    - "Only functions are"
    - "Only in binaries"
  answer: 1
  explanation: "Rust defaults to private; `pub` opts items out."
```
