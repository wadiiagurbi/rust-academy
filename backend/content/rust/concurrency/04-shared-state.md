---
slug: shared-state
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: concurrency
chapterTitle: "Chapter 22 — Concurrency"
chapterOrder: 22
title: "Shared State with Arc and Mutex"
difficulty: intermediate
order: 4
estimatedMinutes: 12
tags: [rust, concurrency, arc, mutex]
---

## Learning Objective

By the end of this lesson, you will share mutable state via `Arc<Mutex<T>>` and scope lock guards tightly.

## Explanation

For threads mutating the same data: **`Arc<Mutex<T>>`**. `Arc` shares ownership; **`Mutex`** (mutual exclusion: one holder at a time) guards the inner `T`. **`.lock()`** blocks until free, returning a **guard** (a handle dereferencing to the data, releasing on drop) — mutate through `*guard`, drop promptly, since rivals block meanwhile. If a holder panicked mid-section, the lock is **poisoned** (tainted; `lock()` yields `Err`) — examples `expect`, servers recover deliberately. Keep critical sections (code held locked) tiny: compute first, lock only for the update.

## Why It Matters

`Arc<Mutex<T>>` powers shared counters, caches, registries. Guard discipline prevents races (impossible) and self-inflicted deadlocks (very possible).

## Example

Four threads adding 25 each to a shared counter; `main` joins all, prints 100.

## Code

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let worker = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = worker.lock().expect("mutex poisoned");
            *guard += 25;
        }));
    }
    for handle in handles {
        handle.join().expect("worker panicked");
    }
    println!("counter: {}", counter.lock().expect("mutex poisoned"));
}
```

## Code Explanation

- Each worker clones the `Arc`, locks, adds 25, drops the guard at closure end.
- `handles` joins all four before the final read; `{}` prints the guarded `i32`. Output: `counter: 100`.

## Common Mistakes

- **Locking before `join`**: main holding the lock while joining workers needing it deadlocks. Lock late, drop early.
- **Ignoring poisoning**: `expect` is fine for examples; servers `match` the `Err` and recover or shut down.
- **Cloning data, not `Arc`**: share via `Arc::clone(&x)` — the handle, not the contents.

## Best Practices

- Scope guards (`{ let mut g = ...; *g += 1; }`) so release points show.
- Never run unknown code (callbacks, blocking sends) while holding a guard.

## Exercise

Use 10 threads adding 10. Predicted total? Then move `println!` before the join loop — what wrong values can appear?

## Solution

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let worker = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            *worker.lock().expect("mutex poisoned") += 10;
        }));
    }
    for handle in handles {
        handle.join().expect("worker panicked");
    }
    let snapshot = *counter.lock().expect("mutex poisoned");
    println!("counter: {snapshot}");
}
```

10 × 10 = `counter: 100`. Reading before joining races workers: any multiple of 10 below 100. Copying into `snapshot` demos the tight-guard idiom.

## Mini Quiz

```yaml
- question: "`lock()` returns?"
  options:
    - "The raw value"
    - "A guard: exclusive access, released on drop"
    - "A new thread"
    - "A sender"
  answer: 1
  explanation: "The guard derefs to the data; its lifetime is the critical section."
- question: "Mutex poisoning?"
  options:
    - "A deadlock"
    - "Taint from a holder's panic; later locks report it"
    - "Forgotten lock"
    - "A lint"
  answer: 1
  explanation: "Flags possibly-inconsistent data; holders choose recovery."
- question: "Why join before reading?"
  options:
    - "Decoration"
    - "Early reads race unfinished workers, showing partial totals"
    - "Mutex demands it"
    - "Frees memory"
  answer: 1
  explanation: "`join` synchronizes completion before the final read."
```
