---
slug: loops
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: control-flow
chapterTitle: "Chapter 3 — Control Flow"
chapterOrder: 3
title: "Loops: While, For, Loop"
difficulty: beginner
order: 4
estimatedMinutes: 10
tags: [rust, loops, while, for]
---

## Learning Objective

By the end of this lesson, you will choose `while`, `for`, or `loop`
for terminating iteration.

## Explanation

**`while`** repeats while a `bool` holds. **`for`** visits a
**range** (`1..5` excludes 5; `1..=5` includes it) or collection
items. **`loop`** repeats until `break`. An **iterator** powers `for`
under the hood (details later). Pick `while` for conditions, `for`
for spans and items, `loop` for run-until-event servers.

## Why It Matters

`for` over ranges avoids manual indexes and off-by-one faults that
plague hand-rolled counters.

## Example

Halving 200 while above 10 ends at 6. `for n in 1..5` prints 1–4.

## Code

```rust
fn main() {
    let mut x = 200;
    while x >= 10 {
        x /= 2;
    }
    println!("halved: {x}");
    for n in 1..5 {
        println!("n: {n}");
    }
}
```

## Code Explanation

- `x /= 2` halves each round: 200→100→50→25→12→6, then stops.
- `1..5` yields 1, 2, 3, 4; `1..=5` would include 5.

## Common Mistakes

- **Loop never flips**: body must move toward the exit.
- **Including 5 with `..`**: use `..=` for inclusive ends.

## Best Practices

- Prefer `for` over indexes; it is bounds-checked.
- Reserve `loop` for true run-until-event cases with visible `break`.

## Exercise

How many lines print? Change to `1..=5`:

```rust
fn main() {
    for x in 1..5 {
        println!("x: {x}");
    }
}
```

## Solution

Four lines (1–4):

```rust
fn main() {
    for x in 1..=5 {
        println!("x: {x}");
    }
}
```

Now five lines (1–5). One character changes the boundary.

## Mini Quiz

```yaml
- question: "When to use for?"
  options:
    - "Loop forever"
    - "Visit each item in a span"
    - "Only conditions"
    - "Never"
  answer: 1
  explanation: "for handles ranges and collections."
- question: "1..5 versus 1..=5?"
  options:
    - "Same"
    - "First excludes 5, second includes it"
    - "First includes 5"
    - "Both infinite"
  answer: 1
  explanation: ".. excludes end; ..= includes it."
- question: "How does loop end?"
  options:
    - "Condition false"
    - "Explicit break inside"
    - "One iteration"
    - "Never"
  answer: 1
  explanation: "loop has no condition; break exits."
```
