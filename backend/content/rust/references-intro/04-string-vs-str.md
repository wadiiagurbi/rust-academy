---
slug: string-vs-str
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: references-intro
chapterTitle: "Chapter 5 — References"
chapterOrder: 5
title: "String vs Str"
difficulty: beginner
order: 4
estimatedMinutes: 10
tags: [rust, strings, str, ownership]
---

## Learning Objective

By the end of this lesson, you will pick `String` for owning and
`&str` for borrowing text.

## Explanation

**`&str`** is a borrowed UTF-8 view; literals (`"Hello"`) live in the
binary. **`String`** is an owned growable buffer with `push` (char)
and `push_str` (slice). `String::from` builds from literals;
`format!` builds from values. Ranges (`&s[2..9]`) index **bytes** and
**panic** mid-character; `.chars()` walks characters safely. Prefer
`&str` params so literals and `String` views both call.

## Why It Matters

Owned versus borrowed decides who grows and who copies. `&str`
helpers accept both callers without cloning.

## Example

`"Hello "` owned plus `"World"` borrowed join via `push_str` to
`Hello World` with length 11.

## Code

```rust
fn main() {
    let s1: &str = "World";
    let mut s2: String = String::from("Hello ");
    s2.push_str(s1);
    println!("s2: {s2}");
}
```

## Code Explanation

- `s1` borrows binary text with no allocation.
- `s2` owns a buffer; `push_str` appends the slice in place.

## Common Mistakes

- **Mid-character slice**: `&s[2..3]` may panic; use `.chars()`.
- **`&String` params**: accept fewer callers than `&str`.

## Best Practices

- Accept `&str`, store `String` for clear lifetimes.
- Build with `format!` and append with `push_str`.

## Exercise

What prints? Add a safe slice view:

```rust
fn main() {
    let mut s = String::from("Hi");
    s.push('!');
    s.push_str(" bye");
    println!("{s}");
}
```

## Solution

Prints `Hi! bye`:

```rust
fn main() {
    let mut s = String::from("Hi");
    s.push('!');
    s.push_str(" bye");
    let view: &str = &s[0..2];
    println!("{s} / {view}");
}
```

Prints `Hi! bye / Hi`. ASCII bounds are safe; Unicode needs chars.

## Mini Quiz

```yaml
- question: "When to use String?"
  options:
    - "Borrowed views"
    - "Owned growable text"
    - "Never"
    - "Single chars"
  answer: 1
  explanation: "String owns and grows via push."
- question: "What is \"Hi\"?"
  options:
    - "Owned String"
    - "&str in binary"
    - "char"
    - "Vec<u8>"
  answer: 1
  explanation: "Literals are static slices."
- question: "Why can &s[2..9] panic?"
  options:
    - "Always"
    - "Bytes may split a char"
    - "No slicing"
    - "Needs nightly"
  answer: 1
  explanation: "Ranges index bytes, not chars."
```
