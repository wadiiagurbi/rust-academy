---
slug: extension-traits
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: idiomatic
chapterTitle: "Chapter 23 — Idiomatic Rust"
chapterOrder: 23
title: "Extension Traits"
difficulty: advanced
order: 4
estimatedMinutes: 12
tags: [rust, traits, extension]
---

## Learning Objective

By the end of this lesson, you will add methods to types you do not own via an **extension trait** (a trait implemented for a foreign type) within the **orphan rule** limits.

## Explanation

The **orphan rule** says you may implement a trait only if your crate owns the trait or the type — so you cannot add an inherent method to `str`. An **extension trait** works around this: define your own trait (owned, so legal) and implement it for `str` or `Vec<T>`. Callers `use` the trait into scope and call the method as if native. Because the method resolves through a trait import, name it distinctively (`truncate_middle`, not `truncate`) to avoid clashing with future standard methods.

## Why It Matters

String helpers, iterator adapters, and test conveniences belong on the type they serve. Extension traits give that call-site shape without forks or wrapper types.

## Example

A `preview()` that shortens long text with an ellipsis, callable on any `&str`.

## Code

```rust
trait Preview {
    fn preview(&self, max: usize) -> String;
}

impl Preview for str {
    fn preview(&self, max: usize) -> String {
        if self.len() <= max {
            return self.to_string();
        }
        format!("{}...", &self[..max])
    }
}

fn main() {
    println!("{}", "hello brave world".preview(5));
    println!("{}", "hi".preview(5));
}
```

## Code Explanation

- `impl Preview for str` is legal: the trait is ours. `&self[..max]` is a **string slice** (borrowed view of bytes `0..max`, safe here as ASCII).
- Prints `hello...` then `hi`: short inputs pass through unchanged.

## Common Mistakes

- **Forgetting the import**: methods are invisible without `use crate::Preview;` in the caller's file.
- **Generic-sounding names**: `process()` collides mentally (and maybe really) with other traits. Use specific names.
- **Slicing mid-character**: `&self[..max]` panics on non-UTF-8-boundary `max`. Use `chars()` for untrusted text.

## Best Practices

- One focused trait per capability; document it must be imported to take effect.
- Prefer returning owned `String` for transformed text; borrow only for pure views.

## Exercise

Add `fn shout(&self) -> String` returning the uppercased text plus `!`. Assert `"hey".shout() == "HEY!"` and print a shouted preview.

## Solution

```rust
trait Preview {
    fn preview(&self, max: usize) -> String;
    fn shout(&self) -> String;
}

impl Preview for str {
    fn preview(&self, max: usize) -> String {
        if self.len() <= max {
            return self.to_string();
        }
        format!("{}...", &self[..max])
    }
    fn shout(&self) -> String {
        format!("{}!", self.to_uppercase())
    }
}

fn main() {
    assert_eq!("hey".shout(), "HEY!");
    println!("{}", "hello brave world".preview(5).shout());
}
```

Prints `HELLO...!`: trait methods chain like inherent ones once imported.

## Mini Quiz

```yaml
- question: "What does the orphan rule allow?"
  options:
    - "Any impl for any type"
    - "An impl only if your crate owns the trait or the type"
    - "No external impls at all"
    - "Impls only in the std crate"
  answer: 1
  explanation: "Your own trait for str is legal because the trait is local."
- question: "Why must callers import the trait?"
  options:
    - "To pay a fee"
    - "Method resolution only sees traits in scope"
    - "To enable unsafe"
    - "It reorders code"
  answer: 1
  explanation: "Without use, the method does not exist at the call site."
- question: "Why a distinctive method name?"
  options:
    - "Longer names run faster"
    - "Avoids clashes with current or future inherent methods"
    - "The compiler demands prefixes"
    - "Short names are reserved"
  answer: 1
  explanation: "Specific names keep the extension unambiguous as std grows."
```
