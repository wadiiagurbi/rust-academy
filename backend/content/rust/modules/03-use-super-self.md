---
slug: use-paths
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: modules
chapterTitle: "Chapter 18 — Modules"
chapterOrder: 18
title: "Use, Super, and Self"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, modules, use, paths]
---

## Learning Objective

By the end of this lesson, you will shorten paths with `use` and navigate with `super`, `self`, and brace groups.

## Explanation

`use` brings an item into scope (callable by short name): `use super::helper;` lets a child call `helper()` directly. **`super`** means the parent module; **`self`** means the current module (as in `use self::deep::deep_value;`). Imports group with braces (`use garden::{plant, water};`), stay local to their module, and re-export with `pub use` (making the imported name itself public, giving callers a short stable path).

## Why It Matters

A glance at the `use` block lists a module's dependencies, and `super` keeps parent calls working through reorganizations.

## Example

A nested `inner` reaching its parent's `helper` via `super` and its own child via `self`.

## Code

```rust
mod outer {
    pub fn helper() -> i32 { 42 }

    pub mod inner {
        use super::helper;
        use self::deep::deep_value;

        mod deep {
            pub fn deep_value() -> i32 { 7 }
        }

        pub fn run() -> i32 { helper() + deep_value() }
    }
}

fn main() { println!("{}", outer::inner::run()); }
```

## Code Explanation

- `use super::helper;` imports the parent's function; renaming `outer` later still resolves. `use self::deep::deep_value;` descends from the current module. `run()` prints `49`.

## Common Mistakes

- **Importing a private item**: `use super::secret;` fails unless the parent marked it `pub` (or you are a descendant with access).
- **Assuming global scope**: an import in `inner` is invisible to `outer` and `main`. Repeat or `pub use` where needed.
- **`self`/`super` swap**: `self::helper` searches the current module; the parent is `super`.

## Best Practices

- One `use` block per module top; group siblings with braces. Prefer `super` over long `crate::` paths in deep trees.

## Exercise

Add `pub fn triple(value: i32) -> i32` to `outer`, import it into `inner`, and return `triple(deep_value())`. What prints?

## Solution

```rust
mod outer {
    pub fn triple(value: i32) -> i32 { value * 3 }

    pub mod inner {
        use super::triple;
        use self::deep::deep_value;

        mod deep {
            pub fn deep_value() -> i32 { 7 }
        }

        pub fn run() -> i32 { triple(deep_value()) }
    }
}

fn main() { println!("{}", outer::inner::run()); }
```

`deep_value()` yields 7, tripled to `21`.

## Mini Quiz

```yaml
- question: "What does `use super::helper;` do?"
  options:
    - "Copies the function"
    - "Brings the parent's `helper` into short-name scope"
    - "Makes it public"
    - "Runs it at startup"
  answer: 1
  explanation: "`use` binds a local shorthand; calls use the bare name."
- question: "`super` and `self` mean?"
  options:
    - "Parent module and current module"
    - "Std and crate root"
    - "Test and release builds"
    - "Interchangeable aliases"
  answer: 0
  explanation: "`super` climbs one level; `self` anchors at the writer."
- question: "What is `pub use` for?"
  options:
    - "Test-only imports"
    - "Re-exporting: publishing an imported name from your module"
    - "External crates only"
    - "Hiding imports"
  answer: 1
  explanation: "Re-exports give callers short stable paths over deep trees."
```
