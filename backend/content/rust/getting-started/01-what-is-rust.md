---
slug: what-is-rust
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: getting-started
chapterTitle: "Chapter 1 — Getting Started"
chapterOrder: 1
title: "What is Rust?"
difficulty: beginner
order: 1
estimatedMinutes: 8
tags: [rust, basics, introduction]
---

## Learning Objective

By the end of this lesson, you will state what Rust is, what it is for, and why teams bet on it.

## Explanation

Rust is a **programming language**: you write `.rs` text files and the
**compiler** (`rustc`) translates them into fast machine code.

Rust is **statically compiled** (checked before it runs; mistakes caught at
compile time) and **systems programming** (C-like control, no garbage
collector, but compiler-checked safety — no use-after-free or data races).

Typical flow:

1. Write code in `src/main.rs` inside a Cargo **package**.
2. Run `cargo run`; Cargo calls `rustc`.
3. The compiler checks types, borrows, lifetimes — errors tell you the fix.
4. You get one native binary, no runtime to install.

## Why It Matters

Rust removes the old trade-off: *control* (C/C++, fast but crash-prone) vs
*safety* (Go, Java, Python with a GC tax). It gives both — hence OSes,
browsers, engines, embedded, and fast servers.

## Example

A greeting program is fully checked: the name's type, ownership, and safe printing.

## Code

```rust
fn main() {
    let visitor_name = "Sarah";
    println!("Welcome back, {visitor_name}!");
}
```

## Code Explanation

- `fn main()` is the entry point every binary calls first.
- `let visitor_name = "Sarah";` binds an immutable `&str`; `mut` opts into reassignment.
- `println!` with `!` is a **macro**, expanded at compile time.
- `{visitor_name}` is inline format args (2021+); older style uses `{}` plus the variable.
- `;` ends statements; a line without `;` is an **expression** whose value returns.

## Common Mistakes

- **Reassigning immutable**: `visitor_name = "Bob";` fails — use `let mut`.
- **Forgetting `!`**: `println("hi")` calls nothing; macros need `!`.
- **Fearing red errors**: read top-down; the first error is usually the fix.

## Best Practices

- Save Rust files with the `.rs` extension and build with `cargo run`, never
  by invoking `rustc` by hand (Cargo passes the right flags and manages
  dependencies).
- Run `cargo fmt` (formatting) and `cargo clippy` (lints) early and often —
  idiomatic formatting is part of Rust culture.
- Read compiler errors fully before searching the web; they suggest fixes
  with a `help:` line more often than any other language's compiler.

## Exercise

Without running any code, predict: what does this print, and does it compile?

```rust
fn main() {
    let x = 5;
    println!("x is {x}");
}
```

Then change it so `x` becomes `6` after the first print and prints again.
What keyword did you need, and why?

## Solution

It prints `x is 5` and it compiles fine. To mutate `x` you need `mut`:

```rust
fn main() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is now {x}");
}
```

`mut` marks the binding as mutable. Rust makes you opt in so that every
mutation in your code is visible and deliberate — the foundation of its
safety guarantees.

## Mini Quiz

```yaml
- question: "When does the Rust compiler check your program?"
  options:
    - "While it runs, line by line"
    - "Before it ever runs, at compile time"
    - "Only when you run cargo test"
    - "It doesn't — Rust has no compiler checks"
  answer: 1
  explanation: "Rust is statically compiled: rustc checks types, borrows and lifetimes before producing a binary."
- question: "What does `let mut x = 5;` mean that `let x = 5;` does not?"
  options:
    - "x is a different type"
    - "x can be reassigned later"
    - "x is stored on the heap"
    - "x is visible in other files"
  answer: 1
  explanation: "Bindings are immutable by default; `mut` opts in to reassignment."
- question: "Why is `println!` written with an exclamation mark?"
  options:
    - "It is a macro, expanded at compile time"
    - "It prints louder than print"
    - "It is an async function"
    - "It is imported from another crate"
  answer: 0
  explanation: "The `!` marks a macro invocation; println! needs it to accept a variable number of arguments."
```
