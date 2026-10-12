---
slug: cargo-projects
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: getting-started
chapterTitle: "Chapter 1 — Getting Started"
chapterOrder: 1
title: "Cargo Projects and Workspaces"
difficulty: beginner
order: 4
estimatedMinutes: 10
tags: [rust, cargo, projects, workspaces]
---

## Learning Objective

By the end of this lesson, you will create a Cargo project and
explain what `Cargo.toml` and workspaces do.

## Explanation

A **crate** is one library or binary that `rustc` builds. A
**package** is a Cargo bundle: one `Cargo.toml` file (the
**manifest**) plus crates. A **workspace** groups packages so they
share one `target/` folder and one `Cargo.lock` file.
`cargo new garden` makes a binary package with `src/main.rs`;
`cargo new --lib tools` makes a library with `src/lib.rs`.
`Cargo.toml` holds name, version, and **edition** (language rules,
such as 2021). `Cargo.lock` pins dependency versions.

## Why It Matters

Every exercise is a Cargo project. Knowing code lives in `src/`,
settings in `Cargo.toml`, and output in `target/` prevents setup pain.

## Example

A garden tracker has a reusable function plus a binary calling it.
Below both sit together; in a real package they split across files.

## Code

```rust
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn main() {
    println!("{}", greet("Ferris"));
}
```

## Code Explanation

- `fn greet(name: &str) -> String` takes borrowed text (`&str`) and
  returns owned text (`String`) built with the `format!` macro.
- `fn main()` is the binary entry point Cargo runs.
- `println!` prints the returned string; `{}` holds the argument.

## Common Mistakes

- **Editing `target/`**: it holds generated output. Edit `src/`.
- **Calling `rustc` by hand**: you lose edition flags. Use cargo.

## Best Practices

- Use `cargo check` while editing and `cargo run` for output.
- Keep edition at `2021` or newer and commit `Cargo.lock`.

## Exercise

Run `cargo new garden && cd garden && cargo run`. Add
`fn water(plants: u32) -> u32` returning `plants + 1`, print it,
and re-run. Which file holds the manifest?

## Solution

`Cargo.toml` is the manifest. The code prints `watered: 4`:

```rust
fn water(plants: u32) -> u32 {
    plants + 1
}

fn main() {
    println!("watered: {}", water(3));
}
```

## Mini Quiz

```yaml
- question: "What is a Cargo package?"
  options:
    - "A single .rs file"
    - "Cargo.toml plus one or more crates"
    - "The target/ folder"
    - "A toolchain channel"
  answer: 1
  explanation: "A package is a manifest plus crates it describes."
- question: "Where does build output go?"
  options:
    - "src/"
    - "target/"
    - "Cargo.toml"
    - "Cargo.lock"
  answer: 1
  explanation: "target/ holds binaries; src/ holds code you edit."
- question: "When does a workspace help?"
  options:
    - "Formatting one file"
    - "Sharing builds across related packages"
    - "Skipping cargo check"
    - "Editing one function"
  answer: 1
  explanation: "Workspaces share one lockfile and target directory."
```
