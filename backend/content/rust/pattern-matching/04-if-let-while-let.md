---
slug: let-else
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: pattern-matching
chapterTitle: "Chapter 7 — Pattern Matching"
chapterOrder: 7
title: "If Let and While Let"
difficulty: beginner
order: 4
estimatedMinutes: 9
tags: [rust, if-let, while-let, option]
---

## Learning Objective

By the end of this lesson, you will use `if let` for one pattern and `while let` to loop on it.

## Explanation

An **`if let`** tests a single pattern: on a fit the body runs with bound names, else the `else` runs. An **`Option`** is an enum with `Some(value)` (present) and `None` (absent). A **`while let`** repeats while the pattern keeps matching. A **`let-else`** requires the pattern to fit and must diverge (return or break) otherwise.

## Why It Matters

Most code cares about one case: `if let` skips full `match` boilerplate, and `while let` drains queues in two lines.

## Example

A task queue pops numbers until empty; an optional mode string picks the run configuration.

## Code

```rust
fn main() {
    let mut queue = vec![3, 2, 1];
    while let Some(task) = queue.pop() {
        println!("run {task}");
    }
    let config: Option<String> = Some("fast".to_string());
    if let Some(mode) = config {
        println!("mode {mode}");
    } else {
        println!("default mode");
    }
}
```

## Code Explanation

- `queue.pop()` returns `Some(value)` while items remain, `None` when empty.
- `while let` binds each popped number and ends at the first `None`; prints `run 1`, `run 2`, `run 3`.
- `if let Some(mode)` moves `config` and prints `mode fast`.

## Common Mistakes

- **Expecting exhaustiveness**: `if let` covers one pattern only. Fix: use `match` when every case matters.
- **Reusing a moved value**: `if let Some(m) = config` moves it. Fix: match on `&config` to borrow.

## Best Practices

- Use `if let` for one case plus fallback; `let-else` to unwrap required values early.

## Exercise

Predict the output, then rewrite with `let-else` printing `fallback` and returning on `None`.

```rust
fn main() {
    let pick: Option<u32> = None;
    if let Some(n) = pick {
        println!("picked {n}");
    } else {
        println!("nothing picked");
    }
}
```

## Solution

It prints `nothing picked`. The `let-else` version:

```rust
fn main() {
    let pick: Option<u32> = None;
    let Some(n) = pick else {
        println!("fallback");
        return;
    };
    println!("picked {n}");
}
```

## Mini Quiz

```yaml
- question: "When should you use `if let` over `match`?"
  options:
    - "One case matters, the rest share an else"
    - "Exhaustiveness is required"
    - "Matching integers only"
    - "No values are bound"
  answer: 0
  explanation: "`if let` fits a single case plus fallback."
- question: "What does `while let Some(x) = it.next()` do?"
  options:
    - "Loops once with zero"
    - "Repeats while the iterator yields values"
    - "Skips the body"
    - "Panics at the end"
  answer: 1
  explanation: "Each Some continues; the first None ends it."
- question: "What must a `let-else` else block do?"
  options:
    - "Print something"
    - "Diverge: return, break, or similar"
    - "Hold a second pattern"
    - "Nothing special"
  answer: 1
  explanation: "It must never fall through, so later code trusts the binding."
```
