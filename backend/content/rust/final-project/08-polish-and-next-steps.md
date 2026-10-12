---
slug: cli-polish
track: final-project
trackTitle: "Final Project"
trackOrder: 2
chapter: final-project
chapterTitle: "Chapter 27 — Final Project: CLI Task Manager"
chapterOrder: 27
title: "Polish and Next Steps"
difficulty: beginner
order: 8
estimatedMinutes: 9
tags: [rust, cli, final-project, polish]
---

## Learning Objective

By the end of this lesson, you will polish output, format code, and name one follow-up feature.

## Explanation

**Polish** (small finish work that makes output clear and code tidy) turns a working CLI into one people enjoy. Three quick wins: friendly `list` lines (`1. [x] Buy milk`), `cargo fmt` (the formatter that rewrites spacing uniformly), and `cargo clippy` (the linter that flags common mistakes with fixes).

A **flag** (an option like `--all` starting with dashes) such as `list --all` vs hiding done tasks is a good first extra. An **integration test** (a test in `tests/` driving the built binary end to end) locks the four commands together after refactors.

## Why It Matters

Graders and users judge the demo: aligned output plus `fmt`-clean code reads as finished, and one scoped extra shows you can extend without breaking `add/list/done/remove`.

## Example

Before polish `list` prints `1|Buy milk|1`; after, it prints `1. [x] Buy milk` with empty-list text `No tasks yet. Add one with: tasks add "Buy milk"`.

## Code

```rust
struct Task {
    title: String,
    done: bool,
}

fn display(task: &Task) -> String {
    let mark = if task.done { "x" } else { " " };
    format!("[{mark}] {}", task.title)
}

fn main() {
    let t = Task { title: String::from("Buy milk"), done: true };
    println!("{}", display(&t));
}
```

## Code Explanation

- `struct Task` here is trimmed to the two fields that affect display.
- `fn display(task: &Task) -> String` borrows the task (no ownership move) and returns owned text.
- `if task.done { "x" } else { " " }` is an **expression** (a snippet returning a value) picking the checkbox mark.
- `format!("[{mark}] {}", task.title)` builds the line; `println!` prints `[x] Buy milk`.
- This file uses only the standard library, so plain `rustc --edition 2021` compiles it.

## Common Mistakes

- **Adding five features at once**: each extra flag doubles manual checks; ship one (`--all` or priorities) well.
- **Skipping `fmt`**: mixed spacing hides real diffs in review; run `cargo fmt` before the final demo.
- **Printing raw `0/1`**: users read `[ ]`/`[x]`; convert the stored integer at the display edge.

## Best Practices

- Handle the empty list first: print the exact `add` command so new users know the next step.
- Run the trio before calling it done: `cargo fmt`, `cargo clippy`, then `cargo test`.
- Write the README demo as four copy-paste lines: `add`, `list`, `done`, `list` showing `[x]`.

## Exercise

Without running code, predict: what does the Code sample print, and does it compile?

Then apply `display` to your `list` output, run `cargo fmt`, `cargo clippy`, and `cargo test` in order.

## Solution

It prints `[x] Buy milk` and it compiles cleanly.

```rust
struct Task {
    title: String,
    done: bool,
}

fn display(task: &Task) -> String {
    let mark = if task.done { "x" } else { " " };
    format!("[{mark}] {}", task.title)
}

fn main() {
    let t = Task { title: String::from("Buy milk"), done: true };
    println!("{}", display(&t));
}
```
```
Borrowing with `&Task` lets `list` print every row without moving them. Capstone step: demo `add/list/done/remove` end to end, then pick one extra like `list --all` as your next feature.

## Mini Quiz

```yaml
- question: "What does cargo fmt do?"
  options:
    - "Adds new dependencies"
    - "Runs the test suite"
    - "Rewrites spacing and layout uniformly"
    - "Publishes the crate"
  answer: 2
  explanation: "fmt applies standard formatting so diffs stay focused on logic."
- question: "What does the display helper return for a finished task?"
  options:
    - "A line like [x] Buy milk"
    - "A SQL query string"
    - "An error value"
    - "Nothing — it deletes the task"
  answer: 0
  explanation: "display maps done to x and pending to a space inside brackets."
- question: "What is a good first extra after the four commands work?"
  options:
    - "Five new commands at once"
    - "One small flag like list --all with tests"
    - "Rewriting storage by hand"
    - "Removing all tests"
  answer: 1
  explanation: "One scoped extra keeps the finished CLI stable while showing growth."
```
