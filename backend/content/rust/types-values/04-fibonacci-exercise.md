---
slug: fibonacci
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: types-values
chapterTitle: "Chapter 2 — Types and Values"
chapterOrder: 2
title: "Exercise: Fibonacci"
difficulty: beginner
order: 4
estimatedMinutes: 15
tags: [rust, exercise, functions, recursion]
---

## Learning Objective

By the end of this lesson, you will implement `fib` in Cargo and
state when `u32` panics.

## Explanation

The **Fibonacci sequence** starts `[0, 1]`; later terms sum the prior
two. **Recursion** calls the function on smaller inputs until a
**base case** (`n < 2`) stops it; otherwise the **recursive case**
(`fib(n-1) + fib(n-2)`) applies. A **Cargo task** means `cargo new`,
editing `src/main.rs`, and proving with `cargo run`. `todo!()` marks
a stub that panics until replaced. With `u32` (max 4,294,967,295),
values overflow near `n = 48` and debug panics.

## Why It Matters

Recursion plus limits recurs in trees and payouts. Asking the valid
range now prevents shipped numeric bugs.

## Example

`fib(6)` splits to `fib(5) + fib(4)` down to `fib(0) = 0` and
`fib(1) = 1`. Small inputs verify logic before limits bite.

## Code

```rust
fn fib(n: u32) -> u32 {
    if n < 2 { n } else { fib(n - 1) + fib(n - 2) }
}

fn main() {
    println!("fib(10) = {}", fib(10));
}
```

## Code Explanation

- Base case `n < 2` returns `n` as a bare expression (no `;`).
- Recursive case sums two smaller answers with `+`.
- `main` prints `fib(10) = 55` as a smoke check.

## Common Mistakes

- **Leaving `todo!()`**: tests panic with not-yet-implemented.
- **Testing only large n**: recursion is slow and overflows; try 0, 1, 10.

## Best Practices

- Keep signature `fn fib(n: u32) -> u32` to match tests.
- Run `cargo test` after each edit plus `cargo fmt` at the end.

## Exercise

Create the task and replace the stub:

```bash
cargo new fib_exercise
cd fib_exercise
cargo run
```

```rust
fn fib(n: u32) -> u32 {
    todo!("implement fib")
}

fn main() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(10), 55);
    println!("fib(20) = {}", fib(20));
}
```

Criteria: `fib(0)=0`, `fib(10)=55`, `fib(20)=6765`.

## Solution

Return `n` directly in the base case:

```rust
fn fib(n: u32) -> u32 {
    if n < 2 { n } else { fib(n - 1) + fib(n - 2) }
}

fn main() {
    assert_eq!(fib(10), 55);
    assert_eq!(fib(20), 6765);
    println!("fib(20) = {}", fib(20));
}
```

Prints `fib(20) = 6765`. Near `n = 48`, `u32` overflows and debug
panics; release would wrap instead.

## Mini Quiz

```yaml
- question: "Base cases for fib?"
  options:
    - "n = 10, 20"
    - "n = 0 -> 0, n = 1 -> 1"
    - "All evens"
    - "None"
  answer: 1
  explanation: "n < 2 returns n and stops recursion."
- question: "When does u32 fib panic in debug?"
  options:
    - "Never"
    - "Near n = 48 past u32::MAX"
    - "At n = 2"
    - "Only in release"
  answer: 1
  explanation: "Values outgrow 32 bits quickly."
- question: "What does todo!() do?"
  options:
    - "Skips tests"
    - "Panics with not-yet-implemented"
    - "Formats code"
    - "Returns zero"
  answer: 1
  explanation: "todo!() marks stubs you must replace."
```
