---
slug: hashmap
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: std-types
chapterTitle: "Chapter 11 — Standard Library Types"
chapterOrder: 11
title: "HashMap"
difficulty: beginner
order: 4
estimatedMinutes: 10
tags: [rust, hashmap, collections]
---

## Learning Objective

By the end of this lesson, you will insert, look up, and update a `HashMap` with `entry`.

## Explanation

A **`HashMap<K, V>`** maps **keys** (labels) to **values** (data) for fast lookup. **`insert`** adds or replaces; **`get`** returns an `Option` borrow. The **`entry` API** (`entry(k).or_insert(v)`) finds or creates a slot in one lookup, returning a mutable borrow to update in place.

## Why It Matters

Counters, caches, and indexes need keyed lookup in one hash instead of find-then-insert.

## Example

A bookstore counts copies per title and looks up stock without scanning lists.

## Code

```rust
use std::collections::HashMap;

fn stock_up(store: &mut HashMap<String, u32>, title: &str, count: u32) {
    *store.entry(title.to_string()).or_insert(0) += count;
}

fn main() {
    let mut store: HashMap<String, u32> = HashMap::new();
    stock_up(&mut store, "dune", 3);
    stock_up(&mut store, "dune", 2);
    println!("{:?} {:?}", store.get("dune"), store.get("mystery"));
}
```

## Code Explanation

- `to_string()` owns the key; `or_insert(0)` creates missing slots; `*... +=` counts in place.
- Output is `Some(5)` then `None`.

## Common Mistakes

- **Missing import**: `HashMap` needs `use std::collections::HashMap;`. Fix: add it.
- **Expecting order**: iteration order is arbitrary. Fix: sort keys when order matters.

## Best Practices

- Own `String` keys for maps outliving calls; read through `get`, update through `entry`.

## Exercise

Predict the output, then stock `"hyperion"` with 4 and print its lookup.

```rust
use std::collections::HashMap;

fn stock_up(store: &mut HashMap<String, u32>, title: &str, count: u32) {
    *store.entry(title.to_string()).or_insert(0) += count;
}

fn main() {
    let mut store: HashMap<String, u32> = HashMap::new();
    stock_up(&mut store, "dune", 3);
    println!("{:?}", store.get("dune"));
}
```

## Solution

It prints `Some(3)`. Adding the title:

```rust
use std::collections::HashMap;

fn stock_up(store: &mut HashMap<String, u32>, title: &str, count: u32) {
    *store.entry(title.to_string()).or_insert(0) += count;
}

fn main() {
    let mut store: HashMap<String, u32> = HashMap::new();
    stock_up(&mut store, "dune", 3);
    stock_up(&mut store, "hyperion", 4);
    println!("{:?} {:?}", store.get("dune"), store.get("hyperion"));
}
```

## Mini Quiz

```yaml
- question: "What does `get` return?"
  options:
    - "Option borrow: Some or None"
    - "The value directly"
    - "A new map"
    - "Nothing"
  answer: 0
  explanation: "Absence becomes None safely."
- question: "What does `or_insert(0)` do?"
  options:
    - "Inserts 0 when new, returns the slot"
    - "Deletes the key"
    - "Counts keys"
    - "Sorts"
  answer: 0
  explanation: "Find-or-create in one lookup."
- question: "Why own String keys?"
  options:
    - "So the map outlives input borrows"
    - "Faster hashing"
    - "Borrows banned"
    - "No reason"
  answer: 0
  explanation: "Owned data avoids lifetime ties."
```
