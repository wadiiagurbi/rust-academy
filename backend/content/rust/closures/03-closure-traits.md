---
slug: fn-traits
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: closures
chapterTitle: "Chapter 10 — Closures"
chapterOrder: 10
title: "Closure Traits"
difficulty: intermediate
order: 3
estimatedMinutes: 12
tags: [rust, closures, fn-traits, generics]
---

## Learning Objective

By the end of this lesson, you will separate `Fn`, `FnMut`, `FnOnce` and bound generics with them.

## Explanation

Each closure implements one **closure trait**: `Fn` borrows captures read-only, `FnMut` borrows mutably to mutate, `FnOnce` consumes captures and runs once. `Fn` is strictest, `FnOnce` broadest. A bound like `F: Fn(u32) -> String` accepts any matching callable shape.

## Why It Matters

Libraries promise precisely: observers take `Fn`, accumulators `FnMut`, one-shot tasks `FnOnce`.

## Example

A repeater calls a read-only labeler; a counter closure mutates its tally per call.

## Code

```rust
fn repeat<F: Fn(u32) -> String>(f: F, times: u32) -> Vec<String> {
    (0..times).map(|i| f(i)).collect()
}

fn main() {
    let tag = String::from("job");
    println!("{:?}", repeat(|i: u32| format!("{tag}-{i}"), 2));
    let mut total = 0;
    let mut add = |n: u32| total += n;
    add(3);
    add(4);
    println!("{total}");
}
```

## Code Explanation

- The bound takes `u32` and returns owned `String`; the labeler captures `tag` read-only (`Fn`).
- `add` mutates `total` (`FnMut`, needs `let mut`); output is the list then `7`.

## Common Mistakes

- **Bounding `Fn` for mutators**: passing `add` fails. Fix: bound `FnMut` when mutation occurs.
- **Consuming captures in `Fn`**: moving a `String` out demotes to `FnOnce`. Fix: clone inside.

## Best Practices

- Take `Fn` for observers, `FnMut` for accumulators, `FnOnce` for one-shot tasks.

## Exercise

Predict the output, then write `repeat_mut` taking `mut f: F: FnMut` for a counting closure.

```rust
fn repeat<F: Fn(u32) -> String>(f: F, times: u32) -> Vec<String> {
    (0..times).map(|i| f(i)).collect()
}

fn main() {
    println!("{:?}", repeat(|i: u32| format!("j-{i}"), 2));
}
```

## Solution

It prints `["j-0", "j-1"]`. The mutating variant:

```rust
fn repeat_mut<F: FnMut(u32) -> String>(mut f: F, times: u32) -> Vec<String> {
    (0..times).map(|i| f(i)).collect()
}

fn main() {
    let mut count = 0;
    println!("{:?}", repeat_mut(|i: u32| { count += 1; format!("{i}-{count}") }, 2));
}
```

## Mini Quiz

```yaml
- question: "Which trait is read-only?"
  options:
    - "Fn"
    - "FnMut"
    - "FnOnce"
    - "Copy"
  answer: 0
  explanation: "Fn borrows captures immutably."
- question: "When is a closure FnMut not Fn?"
  options:
    - "It mutates a capture"
    - "It takes no args"
    - "It returns unit"
    - "It never runs"
  answer: 0
  explanation: "Mutation needs a mutable borrow."
- question: "What does `F: Fn(u32) -> String` mean?"
  options:
    - "Any callable u32-to-String by borrow"
    - "A struct"
    - "Returns nothing"
    - "FnOnce only"
  answer: 0
  explanation: "It describes the accepted call shape."
```
