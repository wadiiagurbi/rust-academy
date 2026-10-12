---
slug: builder-exercise
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: ownership
chapterTitle: "Chapter 13 — Ownership and Memory"
chapterOrder: 13
title: "Exercise: Builder Type"
difficulty: intermediate
order: 6
estimatedMinutes: 15
tags: [rust, exercise, ownership, builder]
---

## Learning Objective

By the end of this lesson, you will move owned data through a **builder** (a helper assembling a value step by step) with passing checks.

## Explanation

A **builder** wraps an unfinished value; each setter takes `mut self` by value, edits inside, and returns `Self` for **chaining**. `impl Into<String>` (a **generic bound** accepting `&str` or `String`) keeps call sites flexible, and `build(self)` moves the finished value out.

## Why It Matters

Builders move ownership piece by piece — the pattern behind configs, queries, and fixtures.

## Example

`PackageBuilder::new("log").version("0.4").build()` moves the name in, replaces the default version, and moves the `Package` out.

## Code

```rust
#[derive(Debug)]
struct Package { name: String, version: String }
struct PackageBuilder(Package);
impl PackageBuilder {
    fn new(n: impl Into<String>) -> Self { Self(Package { name: n.into(), version: "0.1".into() }) }
    fn build(self) -> Package { self.0 }
}
fn main() { println!("{:?}", PackageBuilder::new("log").build()); }
```

## Code Explanation

- The tuple struct owns the in-progress package; `new` fills defaults with `.into()`; `build(self)` moves the value out with no clone.

## Common Mistakes

- **Taking `&mut self`**: borrows instead of moving, so chains fight the checker; take `mut self`, return `Self`.

## Best Practices

- Consume the builder (`mut self -> Self`) for linear chains; default every field in `new`.

## Exercise

Run `cargo new builder_exercise`, fill the `todo!()` stubs, and pass `cargo run` (prints the package, assert passes, no `.clone()`):

```rust
// Stub: fill the todo!() bodies (panics until filled)
#[derive(Debug)]
struct Package { name: String, version: String }
struct PackageBuilder(Package);
impl PackageBuilder {
    fn new(n: impl Into<String>) -> Self { todo!() }
    fn version(mut self, v: impl Into<String>) -> Self { todo!() }
    fn build(self) -> Package { todo!() }
}
fn main() {
    let p = PackageBuilder::new("log").version("0.4").build();
    assert_eq!(p.version, "0.4");
    println!("{p:?}");
}
```

## Solution

Each method moves the single owner forward:

```rust
#[derive(Debug)]
struct Package { name: String, version: String }
struct PackageBuilder(Package);
impl PackageBuilder {
    fn new(n: impl Into<String>) -> Self { Self(Package { name: n.into(), version: "0.1".into() }) }
    fn version(mut self, v: impl Into<String>) -> Self { self.0.version = v.into(); self }
    fn build(self) -> Package { self.0 }
}
fn main() {
    let p = PackageBuilder::new("log").version("0.4").build();
    assert_eq!(p.version, "0.4");
    println!("{p:?}");
}
```

## Mini Quiz

```yaml
- question: "Why take `mut self` and return `Self`?"
  options:
    - "To borrow cheaply"
    - "To move ownership through the chain so calls link"
    - "To allow many owners"
    - "To skip checks"
  answer: 1
  explanation: "By-value chaining moves the single owner step to step."
- question: "What does `impl Into<String>` accept?"
  options:
    - "Only String"
    - "Only &str"
    - "Both &str and String, via .into()"
    - "Integers"
  answer: 2
  explanation: "The bound abstracts over string-like inputs."
- question: "What should build() do?"
  options:
    - "Clone inside"
    - "Move the finished value to the caller"
    - "Drop early"
    - "Return a borrow"
  answer: 1
  explanation: "The completed value transfers to the caller."
```
