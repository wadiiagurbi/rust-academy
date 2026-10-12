---
slug: default-trait
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: std-traits
chapterTitle: "Chapter 12 — Standard Library Traits"
chapterOrder: 12
title: "Default Trait"
difficulty: beginner
order: 3
estimatedMinutes: 8
tags: [rust, traits, default]
---

## Learning Objective

By the end of this lesson, you will supply defaults via `Default` and struct update syntax.

## Explanation

**`Default`** provides `fn default() -> Self`, a sensible starting value. **Struct update syntax** (`..Default::default()`) fills unlisted fields from it. **`unwrap_or_default`** substitutes the inner default for `None`. Deriving works when every field has its own default (zero, empty string, empty vec).

## Why It Matters

Ten-field configs with one tweak become one-liners: set what differs, default the rest.

## Example

A server defaults to localhost; one deployment overrides only the port.

## Code

```rust
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    retries: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self { host: String::from("local"), port: 80, retries: 3 }
    }
}

fn main() {
    println!("{:?}", Config { port: 8080, ..Default::default() });
    println!("{}", Option::<u32>::None.unwrap_or_default());
}
```

## Code Explanation

- Custom `default()` sets each baseline; update syntax swaps port only.
- `u32` default is `0`; output shows the config then `0`.

## Common Mistakes

- **Expecting domain defaults from derive**: derive gives zero/empty. Fix: hand-write `impl Default`.
- **Untyped `None`**: `None.unwrap_or_default()` is ambiguous. Fix: write `Option::<u32>::None`.

## Best Practices

- Implement `Default` for configs and builders; keep `default()` always usable as-is.

## Exercise

Using the Code section's `Config`, predict the port line, then build one overriding only `host` with `"prod"` and print it.

## Solution

Port reads `8080` with the rest defaulted. Host-only override:

```rust
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    retries: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self { host: String::from("local"), port: 80, retries: 3 }
    }
}

fn main() {
    println!("{:?}", Config { host: String::from("prod"), ..Default::default() });
}
```

## Mini Quiz

```yaml
- question: "What does Default provide?"
  options:
    - "A starting value via default()"
    - "A random value"
    - "A harness"
    - "A sort order"
  answer: 0
  explanation: "Baselines others customize."
- question: "What does `..Default::default()` do?"
  options:
    - "Fills remaining fields from default"
    - "Deletes struct"
    - "Calls main"
    - "Ignores fields"
  answer: 0
  explanation: "Listed fields set; rest defaulted."
- question: "What is `unwrap_or_default` on None?"
  options:
    - "The inner type's default"
    - "Always zero"
    - "A panic"
    - "None again"
  answer: 0
  explanation: "It substitutes Default::default()."
```
