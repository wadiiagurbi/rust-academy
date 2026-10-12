---
slug: cli-clap
track: final-project
trackTitle: "Final Project"
trackOrder: 2
chapter: final-project
chapterTitle: "Chapter 27 — Final Project: CLI Task Manager"
chapterOrder: 27
title: "Command-Line Interface with clap"
difficulty: intermediate
order: 6
estimatedMinutes: 12
tags: [rust, cli, final-project, clap]
---

## Learning Objective

By the end of this lesson, you will parse `add/list/done/remove` with `clap` derive macros.

## Explanation

**Parsing** (turning typed words into structured values) is done by `clap` (a crate that builds `--help`, checks args, and splits subcommands). A **subcommand** (a word like `add` that selects one action plus its own args) maps to one **enum variant** (one named choice inside an `enum`, like `Add` holding a title).

A **derive** (a `#[derive(...)]` line that auto-writes trait code, here `Parser`) turns plain structs and enums into a parser. `#[command(...)]` and `#[arg(...)]` are **attributes** (bracket notes that tune the generated parser, like names and help text).

Project excerpt from `src/main.rs` (project file, verify with cargo build):

```rust
// Project file src/main.rs — needs clap derive in Cargo.toml; verify with `cargo build`.
use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(name = "task-manager", version, about = "A simple SQLite-backed CLI task manager")]
struct Cli { #[command(subcommand)] command: Commands, }
#[derive(Debug, Subcommand)]
enum Commands { Add { title: String, #[arg(long, default_value = "medium")] priority: Priority }, List { #[arg(long)] status: Option<Status>, #[arg(long)] priority: Option<Priority> }, Done { id: i64 }, Remove { id: i64 } }
```

`Cli` plus `Commands` match the real app: same struct, enum, variant, and field names.

## Why It Matters

Hand-split `std::env::args()` (raw terminal words) mishandles quotes, numbers, and `--help`. `clap` rejects `done abc` with a usage note and keeps `main` to a short `match` (an expression that runs the arm matching the value).

## Example

`task-manager add "Buy milk"` parses to `Commands::Add { title: "Buy milk", .. }`; `task-manager done 1` parses to `Commands::Done { id: 1 }`; bare `task-manager --help` lists all four subcommands.

## Code

```rust
enum Command { Add(String), List }
fn parse_args(args: &[String]) -> Option<Command> {
    match args.first().map(String::as_str) {
        Some("add") => Some(Command::Add(args.get(1).cloned().unwrap_or_default())),
        Some("list") => Some(Command::List),
        _ => None,
    }
}
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match parse_args(&args) {
        Some(Command::Add(title)) => println!("add {title}"),
        Some(Command::List) => println!("list"),
        None => println!("usage: tasks add <title> | tasks list"),
    }
}
```

## Code Explanation

- `enum Command { Add(String), List }` models two subcommands without any external crate.
- `fn parse_args(args: &[String]) -> Option<Command>` turns raw words into `Some` variant or `None` for help.
- `args.first().map(String::as_str)` looks at the first word; `args.get(1)` takes the title.
- `fn main()` collects `std::env::args()`, matches the result, and prints one line per case.

## Common Mistakes

- **Missing `derive` feature**: `clap = "4"` alone lacks macros; use `features = ["derive"]`.
- **Forgetting `#[command(subcommand)]`**: without it `add` parses as a flag and every call errors.
- **Duplicating validation**: let `clap` check types (`i64`), and keep business checks (id exists) in `store.rs`.

## Best Practices

- Keep `Cli` and `Commands` in `main.rs` so storage and model files stay UI-free.
- Give each arg help text once parsing works, so `--help` teaches the four commands.
- `match` on `command` with one arm per variant, each calling one `store.rs` helper returning `anyhow::Result`.

## Exercise

Without running `rustc`, read the Code sample and predict: what does it print for `add "Buy milk"` and for no args?

Then set `clap = { version = "4", features = ["derive"] }`, add the `Cli` plus `Commands` types, and run `cargo run -- --help`.

## Solution

`add "Buy milk"` prints `add Buy milk`; no args prints the `usage:` line.

```rust
enum Command { Add(String), List }
fn main() {
    let cmds = [Command::Add("Buy milk".to_string()), Command::List];
    for cmd in cmds {
        match cmd {
            Command::Add(title) => println!("add {title}"),
            Command::List => println!("list"),
        }
    }
}
```

The `match` runs one arm per variant. Capstone step: `cargo run -- --help` now lists `add/list/done/remove`; wire each arm to the store next.

## Mini Quiz

```yaml
- question: "What does #[derive(Parser)] do for Cli?"
  options:
    - "Generates CLI parsing from the struct definition"
    - "Creates the SQLite tables"
    - "Runs the test suite"
    - "Formats the code"
  answer: 0
  explanation: "Derive writes the parsing code so Cli::parse reads terminal args."
- question: "What is a subcommand here?"
  options:
    - "A SQL query string"
    - "A test function"
    - "A word like add that selects an action with its own args"
    - "A compiler flag"
  answer: 2
  explanation: "Each Commands variant becomes one subcommand word with its args."
- question: "Why keep Cli in main.rs?"
  options:
    - "clap only works in main.rs"
    - "Parsing stays separate from task data and storage logic"
    - "It makes SQL faster"
    - "Tests cannot import from main"
  answer: 1
  explanation: "Thin wiring in main keeps task.rs and store.rs reusable and testable."
```
