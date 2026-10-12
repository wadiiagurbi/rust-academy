---
slug: traits
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: methods-traits
chapterTitle: "Chapter 8 — Methods and Traits"
chapterOrder: 8
title: "Traits"
difficulty: beginner
order: 2
estimatedMinutes: 10
tags: [rust, traits, impl, polymorphism]
---

## Learning Objective

By the end of this lesson, you will define a trait, implement it, and accept it as a parameter.

## Explanation

A **trait** is a shared interface: method signatures types can implement. A **method signature** is name plus types without a body. An **`impl Trait for Type`** block gives one type its behavior. A **default method** carries a body in the trait that implementors may keep or override.

## Why It Matters

One function can preview articles, videos, and podcasts through a single `Summary` API without knowing each type's details.

## Example

A news app previews any summarizable item through a shared default `preview` built on a required `headline`.

## Code

```rust
trait Summary {
    fn headline(&self) -> String;
    fn preview(&self) -> String {
        format!("{}...", self.headline())
    }
}

struct Article {
    title: String,
}

impl Summary for Article {
    fn headline(&self) -> String {
        self.title.clone()
    }
}

fn show(item: &impl Summary) {
    println!("{}", item.preview());
}

fn main() {
    show(&Article { title: String::from("Rust ships") });
}
```

## Code Explanation

- `headline` is required; `preview` defaults to headline plus `"..."`.
- `&impl Summary` borrows any implementing type; `clone` duplicates the title.
- Output is `Rust ships...`.

## Common Mistakes

- **Skipping a required method**: errors with `not all trait items implemented`. Fix: implement each one lacking a default.
- **Passing by value**: `show(piece)` moves. Fix: pass `&piece` to match `&impl`.

## Best Practices

- Keep traits small; provide defaults for logic built on required methods.

## Exercise

Predict the output, then add a `Video` struct (fields `title: String`, `length: u32`) implementing `Summary` with headline `"TITLE (Ls)"`. Reuse the trait and `show` from the Code section and display one video in `main`.

## Solution

It prints `Rust ships...`. The second type reuses the interface:

```rust
trait Summary {
    fn headline(&self) -> String;
    fn preview(&self) -> String { format!("{}...", self.headline()) }
}
struct Video { title: String, length: u32 }
impl Summary for Video {
    fn headline(&self) -> String { format!("{} ({}s)", self.title, self.length) }
}
fn show(item: &impl Summary) { println!("{}", item.preview()); }
fn main() { show(&Video { title: String::from("Borrow checker"), length: 90 }); }
```

## Mini Quiz

```yaml
- question: "What is a trait?"
  options:
    - "Shared method signatures types can implement"
    - "A struct with privates"
    - "A module alias"
    - "A test function"
  answer: 0
  explanation: "Each type supplies its own behavior for the interface."
- question: "What does `&impl Summary` accept?"
  options:
    - "A borrow of any Summary implementor"
    - "An owned String"
    - "A function pointer"
    - "A static integer"
  answer: 0
  explanation: "`impl Trait` accepts any implementing type."
- question: "What is a default method?"
  options:
    - "A trait method with a body, overridable"
    - "A private helper"
    - "The main function"
    - "A panicking method"
  answer: 0
  explanation: "Types inherit it unless custom logic is needed."
```
