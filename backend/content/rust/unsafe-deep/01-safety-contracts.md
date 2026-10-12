---
slug: safety-contracts
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: unsafe-deep
chapterTitle: "Chapter 24 — Unsafe Deep Dive"
chapterOrder: 24
title: "Safety Contracts and Invariants"
difficulty: advanced
order: 1
estimatedMinutes: 12
tags: [rust, unsafe, safety]
---

## Learning Objective

By the end of this lesson, you will write a **safety contract** (the documented conditions a caller must uphold) for one `unsafe` function and enforce it in a safe wrapper.

## Explanation

Every `unsafe fn` carries a contract: inputs it trusts blindly (pointer validity, alignment, aliasing, lifetime). An **invariant** is one such fact that must hold on every call — e.g. "index is in bounds". Callers of `unsafe fn` must write their own `unsafe` block plus a `// SAFETY:` comment proving each clause. The matching pattern is a **safe wrapper**: a normal `fn` whose signature makes violation impossible (takes `&[T]` and an index it checks), holding the single `unsafe` operation inside. Audit reads contracts, not hope.

## Why It Matters

`get_unchecked`, FFI, and driver code all trust their callers. Written contracts plus narrow wrappers keep that trust checkable instead of tribal.

## Example

Fast element access: the wrapper checks bounds, then uses the unchecked operation once.

## Code

```rust
/// Returns `items[index]`, or `None` when out of bounds.
fn get_fast(items: &[i32], index: usize) -> Option<i32> {
    if index >= items.len() {
        return None;
    }
    // SAFETY: `index < len` checked above; `items` is a valid,
    // aligned, live shared borrow for this call.
    Some(unsafe { *items.get_unchecked(index) })
}

fn main() {
    let v = vec![10, 20, 30];
    println!("{:?} {:?}", get_fast(&v, 1), get_fast(&v, 9));
}
```

## Code Explanation

- `get_unchecked` is an **`unsafe` method**: no bounds check, so callers prove validity. `&[i32]` is a **shared slice borrow** (valid, aligned, live by construction).
- Prints `Some(20) None`: the check converts misuse into a value, not corruption.

## Common Mistakes

- **Contract on the wrong item**: documenting the safe wrapper with SAFETY while the `unsafe fn` stays silent. The contract belongs on the `unsafe` item.
- **Proving liveness weakly**: "pointer is not null" is not enough — validity, alignment, aliasing, and lifetime all need a line.
- **Two unchecked paths**: duplicating the operation in two blocks doubles audit. One block, one proof.

## Best Practices

- Phrase contracts as checkable clauses ("`index < len`", "exclusive for 'a"); keep each `unsafe` block to one operation.
- Make the safe wrapper's types carry the proof (`&[T]`, `NonNull<T>`) so misuse cannot compile.

## Exercise

Write `fn first(items: &[i32]) -> Option<i32>` using `get_unchecked(0)` behind an emptiness check. Predict `first(&[])` and `first(&[7])`.

## Solution

```rust
fn first(items: &[i32]) -> Option<i32> {
    if items.is_empty() {
        return None;
    }
    // SAFETY: non-empty means index 0 is in bounds; `items`
    // is a valid, aligned, live shared borrow.
    Some(unsafe { *items.get_unchecked(0) })
}

fn main() {
    let empty: Vec<i32> = vec![];
    println!("{:?} {:?}", first(&empty), first(&[7]));
}
```

Prints `None Some(7)`: the length proof precedes the unchecked read.

## Mini Quiz

```yaml
- question: "What is a safety contract?"
  options:
    - "A CI badge"
    - "Documented conditions an unsafe fn trusts its caller to uphold"
    - "A private module"
    - "A benchmark result"
  answer: 1
  explanation: "Validity, alignment, aliasing, lifetime — stated, then proven per call."
- question: "Where does the SAFETY comment go?"
  options:
    - "At the top of the file"
    - "At each unsafe block or unsafe fn, proving its clauses"
    - "In the README only"
    - "Nowhere; code is proof"
  answer: 1
  explanation: "Proof lives where risk is taken so reviewers check it locally."
- question: "What is the safe-wrapper pattern?"
  options:
    - "Hiding warnings"
    - "A normal fn whose types make violation impossible, one unsafe op inside"
    - "Disabling unsafe"
    - "A test mock"
  answer: 1
  explanation: "Narrow types plus one audited block concentrate all risk."
```
