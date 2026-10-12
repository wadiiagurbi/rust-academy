---
slug: tuples
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: compound-types
chapterTitle: "Chapter 4 — Compound Types"
chapterOrder: 4
title: "Tuples"
difficulty: beginner
order: 2
estimatedMinutes: 8
tags: [rust, tuples, types]
---

## Learning Objective

By the end of this lesson, you will build mixed-type tuples and read
fields by index.

## Explanation

A **tuple** groups fixed values of differing types, like `(i8,
bool)`. Length and each slot type are fixed. Fields read by **index
access** (`t.0`, `t.1`). `()` is the **unit type** meaning no value;
functions without `->` return it. Tuples suit short returns; structs
suit named data. `for` cannot iterate tuples since types differ.

## Why It Matters

Tuples return two results without a struct while keeping arity fixed
and checked.

## Example

Sensor `(7, true)` pairs code with alarm; `.0` and `.1` pull parts
for logging.

## Code

```rust
fn main() {
    let reading: (i8, bool) = (7, true);
    println!("code: {} alarm: {}", reading.0, reading.1);
}
```

## Code Explanation

- `(i8, bool)` fixes slot types; `(7, true)` fills them in order.
- `.0` reads first, `.1` reads second.

## Common Mistakes

- **`reading[0]`**: tuples use dots, not brackets.
- **`for` over tuple**: types differ, so no item type fits.

## Best Practices

- Keep to two or three fields; larger needs a struct.
- Destructure when using both parts often.

## Exercise

What prints? Split into two bindings:

```rust
fn main() {
    let pair: (u32, &str) = (404, "missing");
    println!("{} {}", pair.0, pair.1);
}
```

## Solution

Prints `404 missing`:

```rust
fn main() {
    let pair: (u32, &str) = (404, "missing");
    let (code, msg) = pair;
    println!("{code} {msg}");
}
```

## Mini Quiz

```yaml
- question: "Read first tuple field how?"
  options:
    - "t[0]"
    - "t.0"
    - "t.first()"
    - "No fields"
  answer: 1
  explanation: "Tuples use numeric dot access."
- question: "What is ()?"
  options:
    - "Empty array"
    - "Unit meaning no value"
    - "Syntax error"
    - "Zero"
  answer: 1
  explanation: "Missing returns and ;-blocks yield ()."
- question: "Why no for over tuples?"
  options:
    - "Always empty"
    - "Types differ per slot"
    - "Heap only"
    - "Strings only"
  answer: 1
  explanation: "for needs one uniform item type."
```
