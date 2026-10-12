---
slug: move-semantics
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: ownership
chapterTitle: "Chapter 13 — Ownership and Memory"
chapterOrder: 13
title: "Move Semantics"
difficulty: beginner
order: 3
estimatedMinutes: 10
tags: [rust, ownership, move, String]
---

## Learning Objective

By the end of this lesson, you will trace ownership transfers (**moves**) through assignment and function calls.

## Explanation

A **move** transfers ownership: after `let s2 = s1;`, `s2` owns the value and `s1` is retired. Only the record moves (plus the small stack metadata); heap bytes stay put. Passing to a by-value parameter moves the same way, so the caller loses it. To keep using the value, pass a **clone** (explicit deep copy, next lesson) or lend it with a borrow (`&`, chapter 15).

## Why It Matters

Moves give C-like speed (no hidden heap copies) with safety: double free is impossible because the old owner is statically retired.

## Example

`say_hello(name)` takes a `String`. With `name.clone()` the original survives; with `name` the memory frees at return.

## Code

```rust
fn say_hello(name: String) {
    println!("Hello {name}");
}
fn main() {
    let name = String::from("Alice");
    say_hello(name.clone());
    say_hello(name);
}
```

## Code Explanation

- A by-value parameter takes ownership on each call; `name.clone()` moves only the copy, so `name` stays valid for the second call.
- The second call moves `name` itself; its text frees when the function returns, exactly once.

## Common Mistakes

- **Using a moved value**: two plain calls fail with `use of moved value`; clone first or borrow.
- **Assuming bytes copied**: moves copy the stack triple only; `.clone()` is the deep copy and is never implicit.

## Best Practices

- Treat by-value passes as transfers; prefer borrowing (`&name`) when the function only reads.

## Exercise

Fix this so both greetings print:

```rust
// DOES-NOT-COMPILE
fn say_hello(name: String) {
    println!("Hello {name}");
}
fn main() {
    let name = String::from("Alice");
    say_hello(name);
    say_hello(name);
}
```

## Solution

Clone for the first call so the second can move the original:

```rust
fn say_hello(name: String) {
    println!("Hello {name}");
}
fn main() {
    let name = String::from("Alice");
    say_hello(name.clone());
    say_hello(name);
}
```

Each `String` drops once. (Alternative: take `&str` and call twice.)

## Mini Quiz

```yaml
- question: "After `let s2 = s1;` for Strings, what holds?"
  options:
    - "Two independent copies"
    - "s2 owns it; s1 is retired"
    - "s1 owns it; s2 borrows"
    - "Bytes are duplicated"
  answer: 1
  explanation: "Assignment moves ownership; the source is retired."
- question: "When is a by-value argument's heap data freed?"
  options:
    - "Never"
    - "At function return, since the parameter owned it"
    - "It returns to the caller"
    - "By the collector"
  answer: 1
  explanation: "The parameter was the owner, so its scope end drops it."
- question: "How to keep using a String after by-value passing?"
  options:
    - "Nothing needed"
    - "Pass name.clone() or lend a borrow"
    - "Declare it mutable"
    - "Add a comment"
  answer: 1
  explanation: "Clone to duplicate, or borrow so ownership never moves."
```
