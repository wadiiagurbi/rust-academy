---
slug: typestate
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: idiomatic
chapterTitle: "Chapter 23 — Idiomatic Rust"
chapterOrder: 23
title: "The Typestate Pattern"
difficulty: advanced
order: 5
estimatedMinutes: 15
tags: [rust, typestate, generics]
---

## Learning Objective

By the end of this lesson, you will encode states as types so calling `send()` before `connect()` fails to compile.

## Explanation

A **typestate** makes each state a distinct type: `Client<Disconnected>` and `Client<Connected>` differ, sharing one struct. The state is a **generic** (a placeholder the compiler fills per use); **marker structs** (empty types existing only as labels) name each state, carried in a `PhantomData<S>` field (a zero-size marker meaning "acts as if owning an `S`"). Methods split per state: `connect()` only on `Disconnected`, `send()` only on `Connected`, and transitions consume `self` — old states cannot be reused.

## Why It Matters

Runtime flags (`if !connected { return Err }`) are forgotten under pressure. Typestates turn wrong-order calls into compiler errors with zero runtime checks.

## Example

A tiny client: `connect()` moves to `Connected`, and only then does `send()` exist.

## Code

```rust
use std::marker::PhantomData;
struct Disconnected;
struct Connected;
struct Client<S> { addr: String, _state: PhantomData<S> }
impl Client<Disconnected> {
    fn new(addr: &str) -> Self { Self { addr: addr.into(), _state: PhantomData } }
    fn connect(self) -> Client<Connected> {
        Client { addr: self.addr, _state: PhantomData }
    }
}
impl Client<Connected> {
    fn send(&self, msg: &str) { println!("{} <- {msg}", self.addr); }
}
fn main() {
    Client::new("chat.example").connect().send("hello");
}
```

## Code Explanation

- `PhantomData<S>` holds no data; it only brands the type. `connect(self)` consumes the old value, so `Disconnected` is gone after the call.
- `send` exists solely on `Connected`. Prints `chat.example <- hello`.

## Common Mistakes

- **`&self` transitions**: `fn connect(&self)` keeps the old state usable. Take `self` to move forward irreversibly.
- **One shared impl block**: `impl<S> Client<S>` methods run in every state. Split blocks per state.
- **Exposed state field**: a public field lets callers forge states. Keep fields private.

## Best Practices

- Consume on transition; keep states as uninhabited marker structs with clear names.
- Document the allowed path (`new -> connect -> send`) on the type itself.

## Exercise

Add a `Quitting` state: `disconnect(self) -> Client<Quitting>` on `Connected`, and `fn bye(&self)` only on `Quitting`. Drive all three states.

## Solution

```rust
use std::marker::PhantomData;
struct Disconnected;
struct Connected;
struct Quitting;
struct Client<S> { addr: String, _state: PhantomData<S> }
impl Client<Disconnected> {
    fn new(addr: &str) -> Self { Self { addr: addr.into(), _state: PhantomData } }
    fn connect(self) -> Client<Connected> { Client { addr: self.addr, _state: PhantomData } }
}
impl Client<Connected> {
    fn disconnect(self) -> Client<Quitting> { Client { addr: self.addr, _state: PhantomData } }
}
impl Client<Quitting> {
    fn bye(&self) { println!("closed {}", self.addr); }
}
fn main() {
    Client::new("chat.example").connect().disconnect().bye();
}
```

Prints `closed chat.example`: each method exists only in its state, so order is enforced.

## Mini Quiz

```yaml
- question: "What is a typestate?"
  options:
    - "A runtime boolean flag"
    - "Each state as a distinct type; methods per state"
    - "A global variable"
    - "An exception class"
  answer: 1
  explanation: "Wrong-order calls fail to compile instead of failing at runtime."
- question: "What is PhantomData<S> for?"
  options:
    - "Heap storage for S"
    - "A zero-size marker branding the struct with S"
    - "A thread pool"
    - "A test double"
  answer: 1
  explanation: "It carries the state parameter without storing anything."
- question: "Why do transitions take self by value?"
  options:
    - "It is faster"
    - "The old state is consumed and cannot be reused"
    - "Borrowing is forbidden"
    - "To clone implicitly"
  answer: 1
  explanation: "Moving forward destroys the previous state value."
```
