---
slug: gui-modules
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: modules
chapterTitle: "Chapter 18 — Modules"
chapterOrder: 18
title: "Exercise: GUI Modules"
difficulty: beginner
order: 4
estimatedMinutes: 15
tags: [rust, modules, exercise]
---

## Learning Objective

By the end of this lesson, you will split a GUI library across files in a cargo project and verify it with `cargo test`.

## Explanation

This is a **cargo task**: `cargo new gui_lab`, with `src/main.rs` (the crate root: `mod` declarations, `use` lines, `main`, tests) and `src/widgets.rs` (the module body). A **widget** is a small UI piece: a `Button` (labelled clickable rectangle) and a `Label` (text line). Rule under test: `label()` text is readable anywhere (`pub`), while `Button` internals stay hidden (private fields, construction only via `Button::new`). `todo!()` bodies compile but panic until implemented.

## Why It Matters

Separating `widgets` from `main` mirrors real GUI crates: hidden rendering internals behind a tidy public API.

## Example

```text
test label_is_readable ... ok
test button_renders_label ... ok
```

## Code

Multi-file cargo layout — `cargo new gui_lab` with `src/main.rs` plus
sibling `src/widgets.rs`. Not a single-file `rustc` program; run with `cargo test` / `cargo run`.

```rust
// File: src/main.rs
mod widgets;

use widgets::{Button, label};

fn main() {
    println!("{}", Button::new("OK").render());
    println!("{}", label("Hi"));
}

#[test]
fn label_is_readable() { assert_eq!(label("Hi"), "Hi"); }

#[test]
fn button_renders_label() { assert_eq!(Button::new("OK").render(), "[OK]"); }
```

```rust
// File: src/widgets.rs
pub struct Button { text: String, width_px: u32 }

impl Button {
    pub fn new(text: &str) -> Self { Button { text: text.to_string(), width_px: 80 } }
    pub fn render(&self) -> String { todo!() }
}

pub fn label(text: &str) -> String { todo!() }
```

## Code Explanation

- Multi-file cargo layout: `mod widgets;` in `src/main.rs` loads sibling `src/widgets.rs`; the `use` line shortens both names. Private fields force construction through `new`; `&str` borrows become owned `String`s via `to_string()`.

## Common Mistakes

- **`mod` in the wrong file**: it belongs in `main.rs`; inside `widgets.rs` it seeks `widgets/widgets.rs`.
- **Public fields**: exposing `width_px` breaks encapsulation — keep fields private, expose methods.

## Best Practices

- Crate root stays thin: declarations, imports, entry point, tests. Rendering returns `String` so logic is testable without a screen.

## Exercise

Write both files, implement the two `todo!()`s so tests pass and `cargo run` prints `[OK]` then `Hi`.

## Solution

Completed `src/widgets.rs` (`main.rs` unchanged):

```rust
pub struct Button { text: String, width_px: u32 }

impl Button {
    pub fn new(text: &str) -> Self { Button { text: text.to_string(), width_px: 80 } }
    pub fn render(&self) -> String { let _ = self.width_px; format!("[{}]", self.text) }
}

pub fn label(text: &str) -> String { text.to_string() }
```

`render` wraps text in brackets; `label` echoes its input. Tests green, `cargo run` prints `[OK]` and `Hi`.

## Mini Quiz

```yaml
- question: "Where does `mod widgets;` look?"
  options:
    - "Sibling `src/widgets.rs` next to the declaring file"
    - "On crates.io"
    - "In std"
    - "In `tests/`"
  answer: 0
  explanation: "Semicolon `mod` maps to a sibling file or directory."
- question: "Why keep `Button` fields private?"
  options:
    - "Structs get smaller"
    - "`new` keeps internals changeable without breaking callers"
    - "Fields cannot be public"
    - "Tests cannot read them"
  answer: 1
  explanation: "Hidden representation plus constructor preserves refactor freedom."
- question: "What does `text.to_string()` do?"
  options:
    - "Borrows longer"
    - "Copies the borrow into an owned `String`"
    - "Converts to bytes"
    - "Nothing; same type"
  answer: 1
  explanation: "`&str` borrows; `String` owns — the struct keeps owned data."
```
