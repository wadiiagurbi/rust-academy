---
slug: threads
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: concurrency
chapterTitle: "Chapter 22 — Concurrency"
chapterOrder: 22
title: "Threads"
difficulty: intermediate
order: 1
estimatedMinutes: 12
tags: [rust, concurrency, threads]
---

## Learning Objective

By the end of this lesson, you will spawn threads, move data in with `move`, and collect results with `join()`.

## Explanation

A **thread** is an independent execution path sharing the program's memory; cores run many truly in parallel. `thread::spawn` (starts a thread running a **closure** — an inline anonymous function) returns a **`JoinHandle`** (a handle for waiting and retrieving the result). Closures usually need **`move`** (take ownership of captures) because a thread may outlive the spawning frame — borrows could dangle, so Rust demands owned data. The return value comes back through **`join()`** (blocking wait yielding `Ok(value)`, or `Err` on panic): spawn, compute, join.

## Why It Matters

Threads turn idle cores into speedup for simulations and servers — with data races rejected at compile time.

## Example

Sum 1 through 10 on a worker while `main` waits, then print it.

## Code

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        let mut total = 0;
        for n in 1..=10 {
            total += n;
        }
        total
    });
    let result = handle.join().expect("worker thread panicked");
    println!("total: {result}");
}
```

## Code Explanation

- The closure needs no `move` (nothing captured); trailing `total` is its return **expression**.
- `join()` blocks `main`; `.expect` (unwrap `Ok` or panic with context) surfaces worker panics. Prints `total: 55`.

## Common Mistakes

- **Borrowing into threads**: `&data` without `move` fails (`does not live long enough`). Move owned data, or share with `Arc` (lesson 4).
- **Forgetting `join`**: `main` returning kills the process — workers die mid-computation silently. Join every handle.
- **Blind `expect` in libraries**: fine in `main`; libraries should propagate the `Err`.

## Best Practices

- Prefer `move` closures; audit every capture as owned.
- No fire-and-forget threads: join before shutdown.

## Exercise

Spawn two threads summing 1..=10 and 11..=20, join both, print the combined total. Expected value?

## Solution

```rust
use std::thread;

fn sum_range(first: i32, last: i32) -> i32 {
    (first..=last).sum()
}

fn main() {
    let first = thread::spawn(|| sum_range(1, 10));
    let second = thread::spawn(move || sum_range(11, 20));
    let total = first.join().expect("first panicked") + second.join().expect("second panicked");
    println!("total: {total}");
}
```

55 + 155 = `total: 210`. Order cannot matter: `join` synchronizes both before the final addition.

## Mini Quiz

```yaml
- question: "What does `thread::spawn` return?"
  options:
    - "The result directly"
    - "A `JoinHandle` for waiting and retrieving"
    - "A process ID"
    - "Nothing"
  answer: 1
  explanation: "`join()` blocks until done and yields the return value."
- question: "Why do spawned closures use `move`?"
  options:
    - "Faster"
    - "Threads may outlive the frame, so captures must be owned"
    - "Otherwise no return value"
    - "Style only"
  answer: 1
  explanation: "`move` satisfies the `'static` bound `spawn` requires."
- question: "`main` returns while workers run?"
  options:
    - "Workers continue"
    - "Process exits; workers die mid-computation"
    - "Auto-join"
    - "Deadlock"
  answer: 1
  explanation: "No implicit join exists — join handles first."
```
