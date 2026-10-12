---
slug: dining-philosophers
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: concurrency
chapterTitle: "Chapter 22 — Concurrency"
chapterOrder: 22
title: "Exercise: Dining Philosophers"
difficulty: advanced
order: 5
estimatedMinutes: 20
tags: [rust, concurrency, exercise]
---

## Learning Objective

By the end of this lesson, you will seat five philosophers on threads with `Mutex` forks, dodge deadlock via lock ordering, and verify with `cargo test`.

## Explanation

This is a **cargo task**: `cargo new philo_lab`, implement `eat_meals` in `src/main.rs`, pass with `cargo test`. The **dining philosophers** problem (five thinkers, five shared forks, two adjacent forks per meal) is classic **contention** — threads competing for overlapping resources. A **fork** is an `Arc<Mutex<()>>` (shared lock, no data; locking *is* holding). The trap: all five grab left forks at once and nobody eats (**deadlock**: a cycle of waits). The escape is **lock ordering** (lowest-numbered fork first), making cycles impossible. Philosopher `id` uses forks `id` and `(id + 1) % 5` (modulo wraps 4 back to fork 0).

## Why It Matters

Forks today, database rows tomorrow: lock ordering is the portable deadlock-free discipline.

## Example

```text
test each_ate_three_meals ... ok
total meals: 15
```

## Code

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn eat_meals(id: usize, forks: Vec<Arc<Mutex<()>>>, meals: usize) -> usize { todo!() }

fn main() {
    let forks: Vec<_> = (0..5).map(|_| Arc::new(Mutex::new(()))).collect();
    let handles: Vec<_> = (0..5).map(|id| { let held = forks.clone(); thread::spawn(move || eat_meals(id, held, 3)) }).collect();
    let total: usize = handles.into_iter().map(|h| h.join().expect("panicked")).sum();
    println!("total meals: {total}");
}

#[test]
fn each_ate_three_meals() {
    let forks: Vec<_> = (0..5).map(|_| Arc::new(Mutex::new(()))).collect();
    assert_eq!(eat_meals(0, forks, 3), 3);
}
```

## Code Explanation

- `todo!()` compiles but panics, failing the test; `join` doubles as a liveness check (deadlock hangs instead of passing). Guards must drop between meals: lock, eat, release, every time.

## Common Mistakes

- **Left-then-right for all**: everyone holds left, blocks on right — textbook deadlock. Order by fork index.
- **`forks[id + 1]`**: panics for philosopher 4. The `% 5` closes the ring.

## Best Practices

- One global lock order, documented where locks are taken; repeat the suite — interleavings differ per run.

## Exercise

Implement `eat_meals` with lower-index-first ordering: test green plus `cargo run` printing `total meals: 15`.

## Solution

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn eat_meals(id: usize, forks: Vec<Arc<Mutex<()>>>, meals: usize) -> usize {
    // Lock ordering: lowest-numbered fork first — cycles impossible.
    let right = (id + 1) % 5;
    let (first, second) = if id < right { (id, right) } else { (right, id) };
    let mut eaten = 0;
    for _ in 0..meals {
        let _f = forks[first].lock().expect("poisoned");
        let _s = forks[second].lock().expect("poisoned");
        eaten += 1;
    }
    eaten
}

fn main() {
    let forks: Vec<_> = (0..5).map(|_| Arc::new(Mutex::new(()))).collect();
    let handles: Vec<_> = (0..5).map(|id| { let held = forks.clone(); thread::spawn(move || eat_meals(id, held, 3)) }).collect();
    let total: usize = handles.into_iter().map(|h| h.join().expect("panicked")).sum();
    println!("total meals: {total}");
}
```

Guards drop each iteration, freeing both forks between meals. 5 × 3 = `total meals: 15`.

## Mini Quiz

```yaml
- question: "What causes the classic deadlock here?"
  options:
    - "Too few threads"
    - "Everyone holds one fork while waiting on another — a wait cycle"
    - "Too many mutexes"
    - "Printing from threads"
  answer: 1
  explanation: "Circular hold-and-wait never resolves without an ordering rule."
- question: "How does lowest-first ordering help?"
  options:
    - "Faster eating"
    - "No circular held-then-wanted chain can form"
    - "Fewer forks"
    - "Pinned cores"
  answer: 1
  explanation: "A global acquisition order is acyclic by construction."
- question: "Why repeat a concurrency suite?"
  options:
    - "Binaries degrade"
    - "Interleavings vary; repetition exposes rare orderings"
    - "Cargo caches wrongly"
    - "Never useful"
  answer: 1
  explanation: "Scheduling is nondeterministic; repeated greens build confidence."
```
