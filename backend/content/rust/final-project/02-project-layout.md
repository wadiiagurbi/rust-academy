---
slug: cli-layout
track: final-project
trackTitle: "Final Project"
trackOrder: 2
chapter: final-project
chapterTitle: "Chapter 27 — Final Project: CLI Task Manager"
chapterOrder: 27
title: "Project Layout: Modules and Files"
difficulty: beginner
order: 2
estimatedMinutes: 9
tags: [rust, cli, final-project, modules]
---

## Learning Objective

By the end of this lesson, you will describe the three source files and how `mod` connects them.

## Explanation

A **module** (a named section of code, often one file, that groups related items) keeps a program tidy. Our CLI uses three:

- `src/main.rs` reads terminal input and calls helpers.
- `src/task.rs` defines the `Task` **struct** (a named group of fields, like `id`, `title`, `done`).
- `src/store.rs` saves and loads tasks from SQLite.

You connect them with `mod task;` (tells Rust to include `task.rs`) plus `use crate::task::Task;` (brings the name `Task` into scope). `crate` here means the current package. The matching project excerpt below needs `Cargo.toml` from lesson 1; check it with `cargo build`, not `rustc` alone:

```rust
// Excerpt from src/main.rs — project file, needs the full Cargo package.
mod task;
mod store;
```

## Why It Matters

One huge `main.rs` mixes parsing, data, and saving, so fixes in one area break another. Split files let you change storage without touching parsing.

## Example

Adding a task flows through layers: `main.rs` reads `add "Buy milk"`, builds a `Task` from `task.rs`, then asks `store.rs` to insert that row into the database file.

## Code

```rust
mod task {
    pub struct Task {
        pub id: i64,
        pub title: String,
    }
}

fn main() {
    let t = task::Task { id: 1, title: String::from("Buy milk") };
    println!("{}: {}", t.id, t.title);
}
```

## Code Explanation

- `mod task { ... }` declares an inline module; in the project the same line `mod task;` loads `src/task.rs` instead.
- `pub` makes the struct and fields usable outside the module; without it they stay private.
- `String::from("Buy milk")` builds an owned `String` (growable text you own) from a borrowed `&str`.
- `task::Task { ... }` is a **struct literal** (code that builds the struct by naming each field).
- `println!("{}: {}", t.id, t.title)` prints `1: Buy milk` using positional placeholders.

## Common Mistakes

- **Forgetting `mod`**: calling `task::Task` without `mod task;` in `main.rs` gives `unresolved module`.
- **Forgetting `pub`**: private fields cannot be built from `main.rs`; the compiler says `field is private`.
- **Wrong file name**: `mod store;` must match `src/store.rs` exactly; `Store.rs` fails on most systems.

## Best Practices

- Name files in `snake_case` (lowercase with underscores) so `mod task;` finds `task.rs` on every system.
- Let `main.rs` stay thin: parse input there, put logic in `task.rs` and `store.rs`.
- After adding each `mod` line, run `cargo build` so a typo shows up at once.

## Exercise

Without running code, predict: what does the Code sample print, and does it compile?

Then in your `tasks` package, create empty `src/task.rs` and `src/store.rs` and add `mod task;` plus `mod store;` at the top of `src/main.rs`.

## Solution

It prints `1: Buy milk` and it compiles cleanly.

```rust
mod task {
    pub struct Task {
        pub id: i64,
        pub title: String,
    }
}

fn main() {
    let t = task::Task { id: 1, title: String::from("Buy milk") };
    println!("{}: {}", t.id, t.title);
}
```

`pub` on the struct and fields is what lets `main` build and read them. Capstone step: your `tasks` package now has the three-file skeleton every later lesson fills in.

## Mini Quiz

```yaml
- question: "What does `mod task;` in main.rs do?"
  options:
    - "Installs the task crate from the network"
    - "Runs the tests for tasks"
    - "Includes src/task.rs as a module of the package"
    - "Deletes the task module"
  answer: 2
  explanation: "mod declares a module; with a semicolon it loads the matching file from src/."
- question: "Why is `pub` needed on Task fields used from main.rs?"
  options:
    - "pub makes the program run faster"
    - "Without pub the fields stay private to their module"
    - "pub is only a comment"
    - "pub changes the field types"
  answer: 1
  explanation: "Items are private by default; pub opens them to other modules."
- question: "Which file should own saving and loading tasks?"
  options:
    - "src/store.rs"
    - "Cargo.toml"
    - "src/task.rs holds saving too"
    - "No file — saving is automatic"
  answer: 0
  explanation: "The layout gives store.rs the saving job so main.rs stays thin."
```
