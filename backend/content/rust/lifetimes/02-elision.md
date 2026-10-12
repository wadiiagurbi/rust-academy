---
slug: lifetime-elision
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: lifetimes
chapterTitle: "Chapter 16 — Lifetimes"
chapterOrder: 16
title: "Lifetime Elision"
difficulty: intermediate
order: 2
estimatedMinutes: 10
tags: [rust, lifetimes, elision, functions]
---

## Learning Objective

By the end of this lesson, you will apply the three elision rules to read signatures that omit lifetimes.

## Explanation

**Elision** is shorthand, not inference: three fixed rules fill missing lifetimes. One: each elided input gets its own. Two: with exactly one input lifetime, it flows to all elided outputs (`identity`). Three: with several inputs but a `&self` receiver, `&self` flows to outputs (`get` returns a borrow of `self`). If outputs stay unassigned — two plain inputs, one return — E0106 asks for names.

## Why It Matters

Most real signatures elide; the three patterns reveal hidden links in iterators, accessors, and builders.

## Example

`identity` links output to its single input; `Foo::get` links output to `&self` beside a second parameter.

## Code

```rust
fn identity(a: &i32) -> &i32 {
    a
}
struct Foo(i32);
impl Foo {
    fn get(&self, other: &i32) -> &i32 {
        let _ = other;
        &self.0
    }
}
fn main() {
    let (v, f) = (7, Foo(9));
    println!("{} {}", identity(&v), f.get(&v));
}
```

## Code Explanation

- `identity` matches rule two (lone input fills the return); `get` matches rule three (`&self` fills the return; `other` is free).
- Broken variant (does not compile): `fn first(a: &i32, b: &i32) -> &i32 { a }`
  matches no rule: error[E0106]: missing lifetime specifier.

## Common Mistakes

- **Expecting elision with two inputs**: write `<'a>` yourself; with `&self` present, the receiver wins, not the last parameter.
- **Reading bodies for clues**: only the three positional rules count, never the implementation.

## Best Practices

- Elide when rules apply; annotate when E0106 asks; with two possible sources, name one shared `'a`.

## Exercise

Which rule covers each function? Confirm with `rustc`:

```rust
fn only_args(a: &i32, b: &i32) {
    let _ = (a, b);
}
fn identity(a: &i32) -> &i32 {
    a
}
fn main() {
    let (x, y) = (1, 2);
    only_args(&x, &y);
    println!("{}", identity(&x));
}
```

## Solution

Rule one for `only_args` (fresh lifetime per input, no output); rule two for `identity`:

```rust
fn only_args(_a: &i32, _b: &i32) {}
fn identity(a: &i32) -> &i32 {
    a
}
fn main() {
    let x = 1;
    assert_eq!(*identity(&x), 1);
    println!("elision ok");
}
```

Underscore prefixes (`_a`) mark "intentionally unused".

## Mini Quiz

```yaml
- question: "What is elision?"
  options:
    - "Body inference"
    - "Shorthand by three positional rules"
    - "Disabled checking"
    - "Runtime tracking"
  answer: 1
  explanation: "Only the rules determine the filled lifetimes."
- question: "identity uses which rule?"
  options:
    - "One: fresh per input"
    - "Two: single input flows to outputs"
    - "Three: &self flows out"
    - "None"
  answer: 1
  explanation: "One input lifetime is shared with the return."
- question: "When must you annotate?"
  options:
    - "Always"
    - "When outputs stay unassigned, e.g. two inputs one output"
    - "With no references"
    - "Never"
  answer: 1
  explanation: "E0106 then requests explicit names."
```
