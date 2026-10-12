---
slug: rust-versions
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: getting-started
chapterTitle: "Chapter 1 — Getting Started"
chapterOrder: 1
title: "Rust Versions and Editions"
difficulty: beginner
order: 5
estimatedMinutes: 8
tags: [rust, editions, versions, rustup]
---

## Learning Objective

By the end of this lesson, you will pick a release channel and set
the edition in `Cargo.toml`.

## Explanation

A **release channel** is a compiler stream: `stable` (tested, for
daily use), `beta` (next preview), and `nightly` (experiments). Stable
ships every six weeks with backwards compatibility. An **edition** is
opt-in language rules (`2015`, `2018`, `2021`, `2024`) declared with
`edition = "2021"` in `Cargo.toml`. Compilers mix editions, so crates
never split the ecosystem. **rustup** manages toolchains (`rustup
update`); **rustc** compiles; **cargo** builds with the right flags.

## Why It Matters

Wrong editions break new syntax like `{name}` captures. Stable plus
a pinned edition keeps every lesson reproducible.

## Example

Edition 2021 allows `println!("hi {name}")`. On 2015 you must write
`println!("hi {}", name)`. One manifest line selects the rules.

## Code

```rust
fn main() {
    let language = "Rust";
    let edition = 2021;
    println!("{language} edition {edition} ready");
}
```

## Code Explanation

- `let language = "Rust";` binds a `&str` (borrowed slice).
- `let edition = 2021;` infers `i32` from the literal.
- `println!` uses inline captures, an edition 2021 feature.

## Common Mistakes

- **Stale OS packages**: use rustup from rustup.rs for updates.
- **Old edition**: new syntax fails. Set `edition = "2021"`.

## Best Practices

- Check `rustc --version` and `cargo --version` after install.
- Run `rustup update` monthly and rebuild once.

## Exercise

Does this compile on 2021? Try editions `2015` and `2021`:

```rust
fn main() {
    let tool = "cargo";
    println!("using {tool}");
}
```

## Solution

Yes on 2021, printing `using cargo`:

```rust
fn main() {
    let tool = "cargo";
    println!("using {tool}");
}
```

On 2015 write `println!("using {}", tool);`. The edition flag picks
the formatting rules.

## Mini Quiz

```yaml
- question: "Which channel fits most developers?"
  options:
    - "nightly always"
    - "stable, updated regularly"
    - "beta pinned forever"
    - "No channel"
  answer: 1
  explanation: "Stable is tested and compatible."
- question: "What does edition = 2021 do?"
  options:
    - "Installs a language"
    - "Selects language rules for that crate"
    - "Publishes to crates.io"
    - "Formats code"
  answer: 1
  explanation: "Editions gate breaking changes per crate."
- question: "How do you check the compiler version?"
  options:
    - "cargo run"
    - "rustc --version"
    - "cargo fmt"
    - "cargo test"
  answer: 1
  explanation: "rustc --version reports the compiler build."
```
