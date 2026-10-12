---
slug: drop-trait
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: ownership
chapterTitle: "Chapter 13 — Ownership and Memory"
chapterOrder: 13
title: "The Drop Trait"
difficulty: beginner
order: 5
estimatedMinutes: 10
tags: [rust, ownership, drop, destructor]
---

## Learning Objective

By the end of this lesson, you will run custom cleanup with `Drop` and predict the order values drop in.

## Explanation

The **`Drop` trait** runs code at scope end: `fn drop(&mut self)` gets an exclusive borrow of the dying value. It takes `&mut self`, not `self`, so the value is not moved into its own cleanup (which would recurse forever); fields drop afterwards. `std::mem::drop(x)` is a helper taking ownership to free a value early. Drops run in reverse declaration order: last created, first dropped.

## Why It Matters

`Drop` is Rust's destructor: files close and locks release on every exit path, panics included, with no runtime.

## Example

Nested `Droppable` values announce themselves: inner drops at its block end, outer at the end of `main`.

## Code

```rust
struct Droppable(&'static str);
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}
fn main() {
    let _a = Droppable("a");
    {
        let _b = Droppable("b");
        println!("inside inner block");
    }
    println!("back in main");
}
```

## Code Explanation

- `&'static str` lives for the whole run (**`'static`** means "lives forever"); `_b` drops first, `_a` last — reverse creation order.

## Common Mistakes

- **Calling `a.drop()`**: forbidden (would double-drop); use `std::mem::drop(a)` for early release.
- **Deriving `Copy` on a `Drop` type**: rejected, since copies cannot run cleanup exactly once.

## Best Practices

- Release resources (locks, files) in `drop` so panics clean up too; keep `drop` bodies short and panic-free.

## Exercise

What prints, in order? Verify with `rustc`:

```rust
struct Droppable(&'static str);
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}
fn main() {
    let _a = Droppable("a");
    println!("one");
    let _b = Droppable("b");
    println!("two");
}
```

## Solution

`one`, `two`, `Dropping b`, `Dropping a` — body prints run first; locals drop in reverse at scope end:

```rust
struct Droppable(&'static str);
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}
fn main() {
    let _a = Droppable("a");
    println!("one");
    let _b = Droppable("b");
    println!("two");
}
```

## Mini Quiz

```yaml
- question: "What signature does Drop require?"
  options:
    - "fn drop(self)"
    - "fn drop(&mut self)"
    - "fn drop(&self) -> bool"
    - "fn drop() -> Self"
  answer: 1
  explanation: "&mut self avoids moving the value into its cleanup."
- question: "In what order do locals drop?"
  options:
    - "Declaration order"
    - "Reverse declaration order"
    - "By name"
    - "Randomly"
  answer: 1
  explanation: "Last created drops first, like nested blocks."
- question: "What does std::mem::drop(x) do?"
  options:
    - "Calls a.drop()"
    - "Takes ownership so x frees at once, ending it early"
    - "Leaks x"
    - "Clones x"
  answer: 1
  explanation: "Ownership transfer plus scope end frees the value now."
```
