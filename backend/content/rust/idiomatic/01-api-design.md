---
slug: api-design
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: idiomatic
chapterTitle: "Chapter 23 — Idiomatic Rust"
chapterOrder: 23
title: "Designing Small, Honest APIs"
difficulty: intermediate
order: 1
estimatedMinutes: 12
tags: [rust, api-design, builder]
---

## Learning Objective

By the end of this lesson, you will shape a small **builder** (a helper collecting settings step by step) so missing settings become `Result` errors.

## Explanation

An **API** is the public types and functions others use. A **builder** constructs a value in stages: each call sets one field and returns the builder for chaining. Required fields stay `Option` inside and surface as `Err` from `build()`; optional fields carry a **default** (a fallback used when the caller says nothing). Setters take `&str` (a borrowed string view) and store `String` (an owned string) so callers pass literals without allocating first.

## Why It Matters

Most boundary bugs are misuse, not logic errors. Builders, defaults, and `Result` turn misuse into messages at build time instead of crashes later.

## Example

A server needs a host (required) and a port (optional, defaults to 8080).

## Code

```rust
struct Server { host: String, port: u16 }
struct Builder { host: Option<String>, port: u16 }
impl Builder {
    fn new() -> Self { Self { host: None, port: 8080 } }
    fn host(mut self, h: &str) -> Self { self.host = Some(h.into()); self }
    fn port(mut self, p: u16) -> Self { self.port = p; self }
    fn build(self) -> Result<Server, String> {
        Ok(Server { host: self.host.ok_or("call host() first")?, port: self.port })
    }
}
fn main() {
    let s = Builder::new().host("127.0.0.1").port(3000).build().expect("cfg");
    println!("{}:{}", s.host, s.port);
}
```

## Code Explanation

- `ok_or` converts missing host into `Err`. Prints `127.0.0.1:3000`.
- Setters take `mut self` by value so calls chain; `&str` input keeps call sites light.

## Common Mistakes

- **Every field optional**: forgetting one yields a half-built value. Check required fields in `build()`.
- **`String` parameters**: force callers to allocate. Take `&str` and convert once inside.
- **Panicking builders**: `build()` should return `Result`, not panic, so callers choose how to fail.

## Best Practices

- One required-field check per `build()` with a message naming the missing setter.
- Keep `new()` as the documented entry point; derive `Default` where a zero-like state exists.

## Exercise

Add optional `timeout_secs: u64` (default 30) with a `timeout()` setter. Build with and without it; print both values.

## Solution

```rust
struct Server { host: String, timeout_secs: u64 }
struct Builder { host: Option<String>, timeout_secs: u64 }
impl Builder {
    fn new() -> Self { Self { host: None, timeout_secs: 30 } }
    fn host(mut self, h: &str) -> Self { self.host = Some(h.into()); self }
    fn timeout(mut self, s: u64) -> Self { self.timeout_secs = s; self }
    fn build(self) -> Result<Server, String> {
        Ok(Server { host: self.host.ok_or("call host() first")?, timeout_secs: self.timeout_secs })
    }
}
fn main() {
    let a = Builder::new().host("x").build().expect("a");
    let b = Builder::new().host("x").timeout(5).build().expect("b");
    println!("{} {}", a.timeout_secs, b.timeout_secs);
}
```

Prints `30 5`: the default applies unless the setter overrides it.

## Mini Quiz

```yaml
- question: "Where should a missing required field surface?"
  options:
    - "As a panic deep in production"
    - "As Err from build() naming the missing setter"
    - "As a warning nobody reads"
    - "It should not surface"
  answer: 1
  explanation: "build() returns Result so misuse becomes a readable error at the boundary."
- question: "Why take &str in setters and store String?"
  options:
    - "It is slower but traditional"
    - "Callers pass literals cheaply; one allocation happens inside"
    - "It avoids all allocations"
    - "&str lives longer than String"
  answer: 1
  explanation: "Borrowed input keeps call sites light; the owned copy is made once."
- question: "What is a builder?"
  options:
    - "A compiler plugin"
    - "A helper collecting settings step by step before build()"
    - "A test harness"
    - "A macro for loops"
  answer: 1
  explanation: "Staged setters plus one checked build() guide correct construction."
```
