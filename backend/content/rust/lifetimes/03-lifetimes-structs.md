---
slug: lifetime-structs
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: lifetimes
chapterTitle: "Chapter 16 — Lifetimes"
chapterOrder: 16
title: "Lifetimes in Structs"
difficulty: intermediate
order: 3
estimatedMinutes: 10
tags: [rust, lifetimes, structs, borrowing]
---

## Learning Objective

By the end of this lesson, you will store borrows in structs and show why the data must outlive the struct.

## Explanation

Structs holding references must declare it: `struct Highlight<'doc> { slice: &'doc str, .. }` says "this view borrows data living at least `'doc`". Users must keep the source alive — early drops are compile errors, never dangling views. Such structs are lightweight **views** over owned data. When views are unneeded, owning (`String` over `&str`) is kinder to callers.

## Why It Matters

Parsers and highlighters return views; struct lifetimes promise a view never outlives its buffer.

## Example

A `Highlight` borrows `"fox"` from an owned `doc`; dropping `doc` first would be rejected.

## Code

```rust
#[derive(Debug)]
enum Color {
    Pink,
    Yellow,
}
#[derive(Debug)]
struct Highlight<'doc> {
    slice: &'doc str,
    color: Color,
}
fn main() {
    let doc = String::from("The quick brown fox.");
    let noun = Highlight { slice: &doc[16..19], color: Color::Yellow };
    println!("{noun:?}");
}
```

## Code Explanation

- `<'doc>` names the data span; `slice: &'doc str` (a borrowed **string slice**) ties the field to it — `noun` cannot outlive `doc`.
- Broken variant (does not compile): `drop(doc);` before the print
  fails with error[E0505]: cannot move out of `doc` because it is borrowed.

## Common Mistakes

- **Omitting the parameter**: borrowed fields need their span declared; the owner's life must cover the last view use.
- **Borrowing where owning fits**: lifetimes that chafe callers mean the struct should own a `String`.

## Best Practices

- One lifetime per struct while views share one source; name it after the source (`'doc`, `'buf`).

## Exercise

Does this compile? Name the error, then fix it:

```rust
// DOES-NOT-COMPILE
#[derive(Debug)]
struct Highlight<'doc> {
    slice: &'doc str,
}
fn main() {
    let noun;
    {
        let doc = String::from("The quick brown fox.");
        noun = Highlight { slice: &doc[16..19] };
    }
    println!("{noun:?}");
}
```

## Solution

No (E0597: `doc` does not live long enough): `doc` dies before the print. Keep both in one scope:

```rust
#[derive(Debug)]
struct Highlight<'doc> {
    slice: &'doc str,
}
fn main() {
    let doc = String::from("The quick brown fox.");
    let noun = Highlight { slice: &doc[16..19] };
    println!("{noun:?}");
}
```

## Mini Quiz

```yaml
- question: "Why does Highlight need <'doc>?"
  options:
    - "Heap allocation"
    - "Declares the span of stored borrowed data"
    - "Mutation"
    - "Debug derive"
  answer: 1
  explanation: "Borrowed fields must name their lifetime."
- question: "Source drops while a view lives: result?"
  options:
    - "View keeps working"
    - "Compile error: borrowed value outlived"
    - "Runtime panic"
    - "Truncation"
  answer: 1
  explanation: "The checker forbids early drops of borrowed data."
- question: "When should a struct own instead?"
  options:
    - "Never"
    - "When no zero-copy view is needed and lifetimes chafe"
    - "Only integers"
    - "Only tests"
  answer: 1
  explanation: "Ownership removes the burden when views add nothing."
```
