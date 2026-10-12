---
slug: cli-persistence
track: final-project
trackTitle: "Final Project"
trackOrder: 2
chapter: final-project
chapterTitle: "Chapter 27 — Final Project: CLI Task Manager"
chapterOrder: 27
title: "Persistence with SQLite"
difficulty: intermediate
order: 4
estimatedMinutes: 12
tags: [rust, cli, final-project, sqlite]
---

## Learning Objective

By the end of this lesson, you will create the tasks table and insert plus list rows through `store.rs`.

## Explanation

**Persistence** (keeping data after the program exits) here means rows in SQLite: a **database** (a file holding tables you query with SQL) accessed through `rusqlite` (a Rust crate that wraps SQLite).

A **table** (named rows with fixed columns) holds our tasks: `id INTEGER PRIMARY KEY AUTOINCREMENT`, `title TEXT NOT NULL`, `status TEXT NOT NULL`, plus `priority TEXT NOT NULL`. A `Connection` (an open handle to the `.db` file) runs SQL via `execute` (for writes) and `query_map` (for reads).

Project excerpt from `src/store.rs` (project file, verify with cargo build):

```rust
// Project file src/store.rs — needs rusqlite in Cargo.toml; verify with `cargo build`.
use anyhow::{Context, Result};
use rusqlite::{params, Connection};
const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, status TEXT NOT NULL, priority TEXT NOT NULL, created_at TEXT NOT NULL DEFAULT (datetime('now')))";
pub struct Store { conn: Connection, }
impl Store { fn init(&self) -> Result<()> { self.conn.execute_batch(SCHEMA).context("failed to initialise tasks table")?; Ok(()) } }
```

`Store::open` opens the file with `Connection::open` and calls `init`, which runs `execute_batch(SCHEMA)` to create the table.

## Why It Matters

Memory-only tasks vanish when the binary exits. A SQLite file keeps todos between `add` and `list` runs with no server to install.

## Example

First run creates `tasks.db` with an empty `tasks` table; `add "Buy milk"` inserts a row with status `todo`; `list` reads every row back into `Task` values for printing.

## Code

```rust
use std::collections::HashMap;
struct MiniStore { rows: HashMap<i64, String>, next_id: i64 }
impl MiniStore {
    fn new() -> Self { Self { rows: HashMap::new(), next_id: 1 } }
    fn insert(&mut self, title: &str) -> i64 {
        let id = self.next_id;
        self.next_id += 1;
        self.rows.insert(id, title.to_string());
        id
    }
    fn get(&self, id: i64) -> Option<&str> {
        self.rows.get(&id).map(String::as_str)
    }
}
fn main() {
    let mut store = MiniStore::new();
    let id = store.insert("Buy milk");
    println!("{id}: {}", store.get(id).unwrap_or("missing"));
}
```

## Code Explanation

- `use std::collections::HashMap;` brings the in-memory map used here to stand in for the SQLite table.
- `struct MiniStore` holds `rows` plus `next_id`, the same two jobs as `id INTEGER PRIMARY KEY AUTOINCREMENT`.
- `fn insert(&mut self, title: &str) -> i64` stores a copy and returns its id, like `INSERT INTO tasks`.
- `fn get(&self, id: i64) -> Option<&str>` returns the title or `None`, like a `SELECT ... WHERE id = ?1`.
- `fn main()` inserts `"Buy milk"`, reads it back, and prints `1: Buy milk`.

## Common Mistakes

- **Opening a new file per command by accident**: pass one path like `tasks.db` so `add` and `list` share rows.
- **Storing `bool` directly**: SQLite has no bool column; this app stores `status` and `priority` as `TEXT`.
- **Forgetting `init`**: inserting before creating the table fails with `no such table: tasks`.

## Best Practices

- Call `Store::open` once at startup before any insert or select, so a fresh machine works first try.
- Use `?` placeholders (like `VALUES (?1, ?2, ?3)`) for titles so quotes in text cannot break SQL.
- Keep SQL strings next to tiny helpers (`add_task`, `list_tasks`) so `main.rs` never writes raw SQL.

## Exercise

Without running `rustc`, read the Code sample and predict: what does it print?

Then add `rusqlite = "0.32"` under `[dependencies]`, write the `Store` plus `SCHEMA` excerpt into `src/store.rs`, and run `cargo build`.

## Solution

It prints `1: Buy milk` because `insert` returns id `1` and `get(1)` finds the row.

```rust
use std::collections::HashMap;

fn main() {
    let mut rows: HashMap<i64, String> = HashMap::new();
    rows.insert(1, "Buy milk".to_string());
    println!("1: {}", rows.get(&1).unwrap_or(&String::from("missing")));
}
```

The map stands in for the `tasks` table. Capstone step: `src/store.rs` can now create `tasks.db`; next you wire inserts and reads for `add` and `list`.

## Mini Quiz

```yaml
- question: "What does Store::open plus init do?"
  options:
    - "Deletes the database file"
    - "Creates the tasks table if missing and returns Ok on success"
    - "Parses CLI arguments"
    - "Prints tasks to the terminal"
  answer: 1
  explanation: "CREATE TABLE IF NOT EXISTS makes startup safe to repeat."
- question: "How are status and priority stored in SQLite here?"
  options:
    - "As TEXT like todo and medium, converted in Rust"
    - "As a separate database file"
    - "As raw Rust bool bytes"
    - "They are not stored at all"
  answer: 0
  explanation: "The SCHEMA uses TEXT columns for status and priority."
- question: "How do you check an excerpt that needs rusqlite?"
  options:
    - "With rustc on the single file"
    - "It needs no checking"
    - "With cargo build inside the package"
    - "By renaming the file"
  answer: 2
  explanation: "External crates resolve through Cargo, so cargo build is the check."
```
