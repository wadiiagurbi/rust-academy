---
slug: installing-rust
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: getting-started
chapterTitle: "Chapter 1 — Getting Started"
chapterOrder: 1
title: "Installing Rust and Cargo"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, setup, cargo, rustup]
---

## Learning Objective

By the end of this lesson, you will have a working Rust toolchain installed
and know what each tool (`rustup`, `rustc`, `cargo`) does.

## Explanation

The Rust toolchain has three main pieces. Understanding which does what saves
a lot of confusion later:

- **rustup**: the toolchain *installer and manager*. It installs Rust, keeps
  it updated (`rustup update`), and lets you switch versions (`rustup
  default stable`). Install once from [rustup.rs](https://rustup.rs).
- **rustc**: the actual *compiler*. It turns `.rs` files into binaries. You
  will almost never call it directly — Cargo does it for you.
- **cargo**: the *build tool and package manager*. It creates projects
  (`cargo new`), builds them (`cargo build`), runs them (`cargo run`),
  checks them fast (`cargo check`), runs tests (`cargo test`), formats code
  (`cargo fmt`), and downloads libraries ("crates") from
  [crates.io](https://crates.io).

Rust releases a new stable version every **six weeks**, always backwards
compatible. There are three **channels**: `stable` (what you want), `beta`
(next release preview), and `nightly` (experimental features).

Install **rust-analyzer** for completion, go-to-definition, and inline errors
(VS Code, Neovim, Emacs; or RustRover IDE).

## Why It Matters

Every later lesson assumes you can create, build, run, and test a project.
Five minutes of setup now avoids "works here, not on my machine" pain —
especially the final project, a real Cargo workspace with tests.

## Example

Creating and running your first binary crate takes three commands and
produces a runnable program plus a testable project layout.

## Code

```bash
cargo new hello_ferris
cd hello_ferris
cargo run
```

## Code Explanation

- `cargo new hello_ferris` creates a directory `hello_ferris/` with
  `Cargo.toml` (project manifest: name, version, edition, dependencies) and
  `src/main.rs` containing a hello-world `fn main`.
- `cargo run` compiles (debug profile) and executes `target/debug/hello_ferris`.
- Siblings: `cargo check` (fast check, no binary), `cargo build --release`,
  `cargo test`, `cargo fmt`, `cargo clippy`.

## Common Mistakes

- **Installing Rust via a Linux distro package or Homebrew**: these are often
  outdated. Use rustup from rustup.rs so `rustup update` keeps you current.
- **Editing code in `target/`**: that directory holds *build output*.
  Your code lives in `src/`; project settings in `Cargo.toml`.
- **Running `rustc main.rs` by hand for real projects**: works for a single
  file, but you lose dependency management, editions, and profiles. Use
  Cargo from lesson one.

## Best Practices

- Verify the install right away: `rustc --version`, `cargo --version`.
- Keep the toolchain fresh: `rustup update` monthly.
- Install rust-analyzer early — inline errors while you type speed learning.
- Pick the current **edition** (2021 or newer) in `Cargo.toml` and leave it.

## Exercise

1. Install Rust via rustup.rs and confirm versions.
2. Run `cargo new hello_ferris && cd hello_ferris && cargo run`. You should
   see `Hello, world!`.
3. Change the printed message to greet yourself by name, then run
   `cargo check` followed by `cargo run`. What is the difference in output
   between the two commands?

## Solution

After editing `src/main.rs`:

```rust
fn main() {
    println!("Hello, Ferris student!");
}
```

`cargo check` only *verifies* the code (fast, no binary produced) and prints
`Checking ... Finished`. `cargo run` compiles *and executes*, so you also
see your program's output (`Hello, Ferris student!`). Habit: `check` while
iterating, `run` when you want output.

## Mini Quiz

```yaml
- question: "Which tool downloads dependencies and builds your project?"
  options:
    - "rustc"
    - "rustup"
    - "cargo"
    - "rust-analyzer"
  answer: 2
  explanation: "cargo is the build tool and package manager; rustc only compiles, rustup only manages toolchains."
- question: "What does `cargo check` do that `cargo run` does not do alone?"
  options:
    - "It runs the test suite"
    - "It quickly verifies code without producing a binary"
    - "It publishes to crates.io"
    - "It formats your code"
  answer: 1
  explanation: "cargo check type-checks fast without codegen; ideal for the edit-check loop."
- question: "How often does Rust ship a new stable release?"
  options:
    - "Once a year"
    - "Every six weeks, backwards compatible"
    - "Whenever a feature is ready, breaking changes allowed"
    - "Only with new editions"
  answer: 1
  explanation: "Six-week trains; editions (2015/2018/2021/2024) are the opt-in mechanism for breaking changes."
```
