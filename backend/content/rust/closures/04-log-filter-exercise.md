---
slug: log-filter
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: closures
chapterTitle: "Chapter 10 — Closures"
chapterOrder: 10
title: "Log Filter Exercise"
difficulty: beginner
order: 4
estimatedMinutes: 18
tags: [rust, closures, exercise, iterators]
---

## Learning Objective

By the end of this lesson, you will filter and map logs with closures and test the pipeline.

## Explanation

A **log entry** holds a `level` plus `text`. **`filter`** keeps items where the closure (**predicate**, true/false function) passes. **`map`** reshapes kept items; **`collect`** gathers results into a `Vec` (growable list). Chaining them reads "keep warnings, format them" with no manual loop.

## Why It Matters

Filter-map cleans real data: warnings from thousands of lines, formatted for alerts, each stage testable.

## Example

A monitor keeps `warn` rows and formats them as `"WARN: disk low"` strings.

## Code

```rust
struct Entry { level: String, text: String }

fn warnings(entries: &[Entry]) -> Vec<String> {
    entries.iter().filter(|e| e.level == "warn").map(|e| format!("WARN: {}", e.text)).collect()
}

fn main() {
    let logs = vec![Entry { level: String::from("warn"), text: String::from("disk low") }];
    println!("{:?}", warnings(&logs));
}
```

## Code Explanation

- Borrows a slice; keeps warnings; formats each owned `String`; collects. Output is `["WARN: disk low"]`.

## Common Mistakes

- **Moving from the slice**: `e.text` by value moves. Fix: borrow inside `format!` as shown.
- **Filtering after mapping**: transforms wasted rows. Fix: `filter` before `map`.

## Best Practices

- Order `filter` first; keep each closure to one condition or format call.

## Exercise

Create a cargo project `log_filter` with `cargo new log_filter`. Implement `warnings` for the criteria.

```rust
// src/main.rs starter — replace todo!() with your code:
struct Entry { level: String, text: String }

fn warnings(entries: &[Entry]) -> Vec<String> { todo!() }

fn main() { todo!() }
```

Your `cargo test` criteria: one warn among mixed gives one string; none gives empty; two warns keep order.

## Solution

```rust
struct Entry { level: String, text: String }

fn warnings(entries: &[Entry]) -> Vec<String> {
    entries.iter().filter(|e| e.level == "warn").map(|e| format!("WARN: {}", e.text)).collect()
}

fn entry(level: &str, text: &str) -> Entry {
    Entry { level: level.to_string(), text: text.to_string() }
}

fn main() { println!("{:?}", warnings(&[entry("warn", "x")])); }

#[test]
fn keeps_only_warnings() {
    let logs = vec![entry("info", "boot"), entry("warn", "disk low")];
    assert_eq!(warnings(&logs), vec![String::from("WARN: disk low")]);
}

#[test]
fn keeps_order() {
    let logs = vec![entry("warn", "a"), entry("warn", "b")];
    assert_eq!(warnings(&logs), vec![String::from("WARN: a"), String::from("WARN: b")]);
}
```

## Mini Quiz

```yaml
- question: "What does `filter` do?"
  options:
    - "Keeps items where the closure is true"
    - "Sorts items"
    - "Deletes the vector"
    - "Counts bytes"
  answer: 0
  explanation: "The predicate decides membership."
- question: "What does `map` do here?"
  options:
    - "Formats each kept entry"
    - "Filters again"
    - "Sorts"
    - "Writes disk"
  answer: 0
  explanation: "`map` converts each item's shape."
- question: "Why `collect()`?"
  options:
    - "Iterators are lazy; it builds results"
    - "It dedups"
    - "It prints"
    - "It exits"
  answer: 0
  explanation: "Nothing runs until consumed."
```
