---
slug: cxx-interop
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: beyond-backend
chapterTitle: "Chapter 26 — Rust Beyond the Backend"
chapterOrder: 26
title: "C++ Interop with cxx"
difficulty: advanced
order: 4
estimatedMinutes: 15
tags: [rust, cpp, cxx, ffi]
---

## Learning Objective

By the end of this lesson, you will apply the **opaque-pointer pattern** (a `Handle(u32)` id standing in for a C++ object) behind bridges like **`cxx`** (the standard Rust↔C++ shim generator).

## Explanation

C++ classes cannot cross FFI directly (constructors, vtables, and exceptions have no C equivalent), so bridges pass **opaque handles**: small integers naming objects that stay on their home side. `cxx` automates this with a `#[cxx::bridge]` module describing shared items, generating type-checked shims. Without the crate (unavailable in this offline lesson), the same idea is hand-rolled: a registry maps ids to owned values, and only the id crosses. The bridge sketch is excerpt-only; the registry compiles on host:

```text
EXCERPT-ONLY (needs the cxx crate; not compiled here):
#[cxx::bridge]
mod ffi { extern "Rust" { fn rust_double(v: u32) -> u32; } }
```

## Why It Matters

Game engines, browsers, and trading stacks are C++; handles plus generated shims let Rust join them without sharing object layouts.

## Example

A registry of counters addressed by handle ids — the hand-rolled core of every bridge.

## Code

```rust
#[derive(Debug, Clone, Copy)] struct Handle(u32);
struct Registry { values: Vec<u32> }
impl Registry { fn new() -> Self { Self { values: Vec::new() } }
    fn insert(&mut self, value: u32) -> Handle {
        self.values.push(value); Handle(self.values.len() as u32 - 1)
    }
    fn double(&mut self, h: Handle) -> Result<u32, &'static str> {
        let slot = self.values.get_mut(h.0 as usize).ok_or("bad handle")?;
        *slot = slot.saturating_mul(2); Ok(*slot)
    }
}
fn main() { let mut reg = Registry::new(); let h = reg.insert(21);
    println!("{h:?} {:?}", reg.double(h));
    println!("{:?}", reg.double(Handle(99)));
}
```

## Code Explanation

- `Handle(u32)` is an **opaque id**: only the registry maps it to data, so no layout is ever shared; `get_mut` bounds-checks, turning stale handles into `Err`. Prints `Handle(0) Ok(42)` then `Err("bad handle")`.

## Common Mistakes

- **Sharing layouts or pointers**: `#[repr(C)]` structs drift between compilers and raw C++ pointers dangle silently. Prefer handles; move ids, never pointers.
- **Exceptions across FFI**: C++ exceptions through Rust abort. Catch at the C++ shim and return codes.

## Best Practices

- Validate every incoming handle and saturate arithmetic so bridge functions stay panic-free; graduate hand-rolled shims to `cxx` once available.

## Exercise

Add `fn get(&self, h: Handle) -> Result<u32, &'static str>` reading without mutation. Insert 7, double it, assert the read is 14.

## Solution

```rust
#[derive(Debug, Clone, Copy, PartialEq)] struct Handle(u32);
struct Registry { values: Vec<u32> }
impl Registry { fn new() -> Self { Self { values: Vec::new() } }
    fn insert(&mut self, value: u32) -> Handle {
        self.values.push(value); Handle(self.values.len() as u32 - 1)
    }
    fn get(&self, h: Handle) -> Result<u32, &'static str> {
        self.values.get(h.0 as usize).copied().ok_or("bad handle")
    }
    fn double(&mut self, h: Handle) -> Result<u32, &'static str> {
        let s = self.values.get_mut(h.0 as usize).ok_or("bad handle")?;
        *s = s.saturating_mul(2); Ok(*s)
    }
}
fn main() { let mut reg = Registry::new(); let h = reg.insert(7);
    reg.double(h).expect("live handle"); assert_eq!(reg.get(h), Ok(14)); println!("ok {:?}", reg.get(h));
}
```

Prints `ok Ok(14)`: reads and writes agree through the validated handle.

## Mini Quiz

```yaml
- question: "Why opaque handles instead of shared objects?"
  options:
    - "Handles are trendier"
    - "C++ layouts, ctors, and exceptions have no C equivalent"
    - "Handles run faster always"
    - "Objects cannot be named"
  answer: 1
  explanation: "Ids cross safely; ownership stays on the home side."
- question: "What does cxx generate?"
  options:
    - "Random tests"
    - "Type-checked shims for both sides of the bridge"
    - "A new compiler"
    - "Documentation only"
  answer: 1
  explanation: "The bridge module keeps Rust and C++ signatures in sync."
- question: "How are stale handles handled?"
  options:
    - "Dereferenced anyway"
    - "Bounds-checked into Err, never dereferenced blindly"
    - "Ignored silently"
    - "By panicking"
  answer: 1
  explanation: "Validation converts forged ids into clean errors."
```
