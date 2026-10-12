---
slug: string-vec
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: std-types
chapterTitle: "Chapter 11 — Standard Library Types"
chapterOrder: 11
title: "String and Vec"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, string, vec, std]
---

## Learning Objective

By the end of this lesson, you will build `String` text and grow a `Vec` with `push`.

## Explanation

A **`String`** owns growable heap text; a **string slice** (`&str`) borrows a view like `"hi"`. A **`Vec<T>`** owns an ordered growable list. **`push`** appends to a vector, **`push_str`** appends a slice to a `String`. Owned types grow; borrowed views are cheap to pass.

## Why It Matters

Messages and lists appear everywhere; owning versus borrowing decides who may grow the data.

## Example

A greeter builds one owned string per guest and stores all greetings for sending.

## Code

```rust
fn greetings(names: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for name in names {
        out.push(format!("hi {name}!"));
    }
    out
}

fn main() {
    let list = greetings(&["ana", "ben"]);
    println!("{list:?} {}", list.len());
}
```

## Code Explanation

- Takes a slice of borrowed names; `format!` builds each owned greeting.
- Output is `["hi ana!", "hi ben!"] 2`.

## Common Mistakes

- **Pushing `&str` into `Vec<String>`**: types differ. Fix: convert with `to_string()` or `format!`.
- **Forgetting `mut`**: growth mutates. Fix: declare `let mut`.

## Best Practices

- Take `&str`/`&[T]` params; return owned `String`/`Vec`; pre-size with capacity when known.

## Exercise

Predict the output, then change the prefix to `"welcome "` and confirm both greetings update.

```rust
fn greetings(names: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for name in names {
        out.push(format!("hi {name}!"));
    }
    out
}

fn main() {
    println!("{:?}", greetings(&["ana", "ben"]));
}
```

## Solution

It prints both `hi` greetings. Only the template changes:

```rust
fn greetings(names: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for name in names {
        out.push(format!("welcome {name}!"));
    }
    out
}

fn main() {
    println!("{:?}", greetings(&["ana", "ben"]));
}
```

## Mini Quiz

```yaml
- question: "String vs &str?"
  options:
    - "String owns growable text; &str borrows a view"
    - "Identical"
    - "&str owns data"
    - "String can't grow"
  answer: 0
  explanation: "Owned grows; borrowed views read cheaply."
- question: "What does Vec `push` do?"
  options:
    - "Appends one element"
    - "Removes first"
    - "Sorts"
    - "Clears"
  answer: 0
  explanation: "`push` grows the list by one."
- question: "Why need `mut`?"
  options:
    - "Growth mutates the collection"
    - "It is faster"
    - "It returns nothing"
    - "It fails silently"
  answer: 0
  explanation: "Mutation requires a mutable binding."
```
