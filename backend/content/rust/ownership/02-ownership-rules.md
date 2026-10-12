---
slug: ownership-rules
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: ownership
chapterTitle: "Chapter 13 — Ownership and Memory"
chapterOrder: 13
title: "Ownership Rules and Scope"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, ownership, scope, drop]
---

## Learning Objective

By the end of this lesson, you will apply the three ownership rules to predict when a value is created, used, and freed.

## Explanation

An **owner** is the one variable that frees a value; a **scope** (a `{ ... }` block) is where it is valid. The rules: every value has exactly one owner; only one owner at a time; the value is **dropped** (freed, running cleanup) when the owner leaves scope. A **destructor** is that cleanup code. Rust inserts the drop automatically — no `free()` call, no collector pause.

## Why It Matters

Single ownership frees heap memory exactly once at zero runtime cost; "one owner, freed at scope end" makes later errors traceable by hand.

## Example

A `Point` built inside a block prints there, then drops at the closing brace; later code cannot name it.

## Code

```rust
struct Point(i32, i32);
fn main() {
    {
        let p = Point(3, 4);
        println!("inside: {}, {}", p.0, p.1);
    } // p drops here
    println!("back in outer scope");
}
```

## Code Explanation

- `struct Point(i32, i32);` is a **tuple struct** (fields read as `p.0`, `p.1`); `let p = ...` makes `p` the sole owner.
- The closing brace drops `p`; naming it later fails with `cannot find value p in this scope`.

## Common Mistakes

- **Using a value after its block**: the owner is gone; widen the scope or return the value.
- **Expecting two owners**: `let q = p;` moves heap ownership (lesson 3); `p` is then retired.

## Best Practices

- Keep scopes tight so values drop early; share access with borrows (`&`), not extra owners.

## Exercise

Does this compile? Quote the error, then fix it:

```rust
// DOES-NOT-COMPILE
struct Point(i32, i32);
fn main() {
    {
        let p = Point(3, 4);
        println!("{}", p.0);
    }
    println!("{}", p.1);
}
```

## Solution

No (cannot find value `p` in this scope). Use it before the drop:

```rust
struct Point(i32, i32);
fn main() {
    {
        let p = Point(3, 4);
        println!("{} {}", p.0, p.1);
    }
}
```

## Mini Quiz

```yaml
- question: "How many owners can a value have at once?"
  options:
    - "Exactly one"
    - "As many as borrow it"
    - "One per scope"
    - "Unlimited"
  answer: 0
  explanation: "Every value has precisely one owner at all times."
- question: "When is a value dropped?"
  options:
    - "When the collector runs"
    - "When its owner goes out of scope"
    - "When last read"
    - "Never"
  answer: 1
  explanation: "Scope end triggers the drop automatically."
- question: "What is a scope?"
  options:
    - "A heap allocation"
    - "A `{ ... }` block where a binding is valid"
    - "A compiler flag"
    - "A pointer kind"
  answer: 1
  explanation: "Bindings live within the block that declares them."
```
