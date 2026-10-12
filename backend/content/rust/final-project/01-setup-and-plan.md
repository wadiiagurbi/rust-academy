---
slug: cli-task-manager-setup
track: final-project
trackTitle: "Final Project"
trackOrder: 2
chapter: final-project
chapterTitle: "Chapter 27 — Final Project: CLI Task Manager"
chapterOrder: 27
title: "Setup and Plan: CLI Task Manager"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, cli, final-project, setup]
---

## Learning Objective

By the end of this lesson, you will create the project folder and state what the finished task manager will do.

## Explanation

A **CLI** (command-line interface: a program you run in a terminal by typing its name plus words) task manager lets you track todos without a website or button.

Our finished app supports four **commands** (words after the program name that pick an action):

1. `add "Buy milk"` stores a new task.
2. `list` shows all tasks.
3. `done 1` marks task 1 as finished.
4. `remove 1` deletes task 1.

We build it as a Cargo **package** (a folder with `Cargo.toml` plus `src/` that Cargo can build). Three helper **crates** (packaged libraries listed in `Cargo.toml`) do heavy work: `clap` parses commands, `anyhow` reports errors with context, and `rusqlite` stores rows in SQLite.

Plan: scaffold the package, add the three crates, then add one file per part (`task.rs`, `store.rs`) with tests.

## Why It Matters

Small practice files teach pieces; a full CLI shows how pieces fit: parsing input, holding data in a **struct** (a named group of fields), saving to disk, and testing each part.

## Example

A finished session looks like this: you type `tasks add "Read chapter"`, then `tasks list` prints `1. [ ] Read chapter`, then `tasks done 1` flips it to `1. [x] Read chapter`.

## Code

```rust
fn main() {
    let app_name = "tasks";
    println!("{app_name} commands: add, list, done, remove");
}
```

## Code Explanation

- `fn main()` is the entry point every binary calls first.
- `let app_name = "tasks";` binds an immutable `&str` (a borrowed text view); `mut` would allow reassignment.
- `println!` with `!` is a **macro** (code expanded at compile time, not a normal function call).
- `{app_name}` is inline format args (2021+); older style writes `"{}"` plus the variable afterwards.
- `;` ends statements; the binary prints one help line when run.

## Common Mistakes

- **Creating files by hand only**: run `cargo new tasks --bin` so `Cargo.toml` and `src/main.rs` match.
- **Wrong folder**: run `cargo run -- add "x"` inside the package folder, not its parent.
- **Skipping the plan**: writing `store.rs` before `task.rs` forces rewrites; follow the order in this chapter.

## Best Practices

- Keep one idea per file: `task.rs` holds data, `store.rs` holds saving, `main.rs` wires commands.
- Pin your plan where you can see it: commands `add/list/done/remove` plus files `main.rs`, `task.rs`, `store.rs`.
- Run `cargo run -- --help` after each lesson to confirm the binary still starts.

## Exercise

Without running code, predict: what does the Code sample print, and does it compile?

Then on your machine run `cargo new tasks --bin`, open `Cargo.toml`, and add `clap`, `anyhow`, and `rusqlite` under `[dependencies]` as this chapter later shows.

## Solution

It prints `tasks commands: add, list, done, remove` and it compiles cleanly.

```rust
fn main() {
    let app_name = "tasks";
    println!("{app_name} commands: add, list, done, remove");
}
```

`app_name` is immutable because we never reassign it. Capstone step: you now have a `tasks` package folder; keep it open since every later lesson adds to it.

## Mini Quiz

```yaml
- question: "What four commands will the finished task manager support?"
  options:
    - "add, list, done, remove"
    - "new, open, save, quit"
    - "init, build, test, deploy"
    - "get, post, put, delete"
  answer: 0
  explanation: "The capstone spec fixes the commands to add, list, done, and remove."
- question: "What is a Cargo package?"
  options:
    - "A single .rs file with no config"
    - "A folder with Cargo.toml plus src/ that Cargo can build"
    - "A downloaded zip of Rust itself"
    - "A terminal window"
  answer: 1
  explanation: "The package folder plus manifest lets Cargo build, run, and test the app."
- question: "Which crate parses commands, reports errors, and stores rows in this project?"
  options:
    - "clap parses, anyhow reports, rusqlite stores"
    - "rusqlite parses, clap stores, anyhow compiles"
    - "anyhow parses, rusqlite reports, clap stores"
    - "None — the project uses no crates"
  answer: 0
  explanation: "clap handles CLI parsing, anyhow simplifies errors, rusqlite talks to SQLite."
```
