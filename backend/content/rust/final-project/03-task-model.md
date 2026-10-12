---
slug: cli-task-model
track: final-project
trackTitle: "Final Project"
trackOrder: 2
chapter: final-project
chapterTitle: "Chapter 27 — Final Project: CLI Task Manager"
chapterOrder: 27
title: "Task Model: Structs and Enums"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, cli, final-project, struct]
---

## Learning Objective

By the end of this lesson, you will define the `Task` struct and build new tasks with `Task::new`.

## Explanation

A **struct** (a custom type that groups named fields under one name) models one row of our app. Each task needs an `id` (`i64`: a 64-bit integer, matches SQLite row ids), a `title` (`String`: owned, growable text), and `done` (`bool`: true or false).

An **impl block** (a section starting `impl Task` that holds helper functions for that type) gives us a constructor: an **associated function** (a function called as `Task::new`, not on an existing value) that fills `done` with `false` for fresh tasks.

A method like `mark_done(&mut self)` takes `&mut self` (exclusive borrow of the current value) so it can flip `done` to `true`.

## Why It Matters

Every command touches `Task`: `add` builds one, `list` prints many, `done` flips the flag, `remove` deletes by `id`. One shared definition stops the four commands from drifting apart.

## Example

`tasks add "Read chapter"` builds `Task { id: 0, title: "Read chapter", done: false }`; the store assigns the real `id` when it inserts the row, and `list` prints `[ ]` or `[x]` from `done`.

## Code

```rust
struct Task {
    id: i64,
    title: String,
    done: bool,
}

impl Task {
    fn new(id: i64, title: String) -> Self {
        Self { id, title, done: false }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }
}

fn main() {
    let mut t = Task::new(1, String::from("Read chapter"));
    t.mark_done();
    println!("{}. {} done={}", t.id, t.title, t.done);
}
```

## Code Explanation

- `struct Task { ... }` declares three fields; `Self` inside `impl` is shorthand for `Task`.
- `fn new(id: i64, title: String) -> Self` builds a fresh task with `done: false`.
- `Self { id, title, done: false }` uses field shorthand: `id` means `id: id`.
- `fn mark_done(&mut self)` borrows the task mutably so it can set `self.done = true`.
- `let mut t` is required because `mark_done` needs a mutable borrow; printing shows `1. Read chapter done=true`.

## Common Mistakes

- **Passing `&str` where `String` fits**: `Task::new(1, "hi")` fails; write `String::from("hi")` to own the text.
- **Calling `mark_done` on immutable**: without `mut` the borrow checker rejects `&mut self`.
- **Copying `id` types**: keep `i64` everywhere; mixing `i32` and `i64` breaks SQLite id plumbing later.

## Best Practices

- Keep `Task` free of printing or SQL; it only holds data plus tiny helpers like `new`.
- Derive `Debug` (compiler-written formatting) in the real file so `println!("{t:?}")` helps while testing.
- Validate the title (non-empty after trimming) inside `new` or a helper before saving.

## Exercise

Without running code, predict: what does the Code sample print, and does it compile?

Then write `src/task.rs` in your package with this `Task`, `new`, and `mark_done`, and build a quick `main` check like the sample.

## Solution

It prints `1. Read chapter done=true` and it compiles cleanly.

```rust
struct Task {
    id: i64,
    title: String,
    done: bool,
}

fn main() {
    let t = Task { id: 1, title: String::from("Read chapter"), done: true };
    println!("{}. {} done={}", t.id, t.title, t.done);
}
```

The struct owns its `title` as `String`, so tasks stay valid after the input line is dropped. Capstone step: `src/task.rs` now defines `Task::new`; the store lesson can save these values.

## Mini Quiz

```yaml
- question: "What does Task::new do in this project?"
  options:
    - "Saves a task to SQLite"
    - "Builds a Task with the given id and title and done set to false"
    - "Parses command-line arguments"
    - "Deletes a task by id"
  answer: 1
  explanation: "new is a constructor; persistence and parsing live in other files."
- question: "Why does mark_done take &mut self?"
  options:
    - "All methods must take &mut self"
    - "It makes the method run faster"
    - "It must change the value, which needs an exclusive borrow"
    - "It avoids using a struct"
  answer: 2
  explanation: "&mut self borrows the task mutably so done can flip to true."
- question: "Why is title a String rather than &str here?"
  options:
    - "The task must own its text after the input line is gone"
    - "String is the only type SQLite accepts"
    - "&str cannot be printed"
    - "String uses less memory"
  answer: 0
  explanation: "Owned String keeps the task valid; a borrow would tie it to short-lived input."
```
