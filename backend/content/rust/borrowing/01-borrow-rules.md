---
slug: borrow-rules
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: borrowing
chapterTitle: "Chapter 15 — Borrowing"
chapterOrder: 15
title: "Borrow Rules"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, borrowing, references, aliasing]
---

## Learning Objective

By the end of this lesson, you will lend values with `&` and state the aliasing rule for overlapping borrows.

## Explanation

**Borrowing** lends use without ownership: `&p1` (a **shared reference**, `&T`) gives read-only access while the caller keeps ownership. A borrow ends at its last use (**non-lexical lifetimes**), so later mutation is often fine. The **aliasing rule** caps overlaps: many shared (`&T`) borrows **or** exactly one exclusive (`&mut T`) borrow — never both at once. That one sentence outlaws data races and dangling reads.

## Why It Matters

Borrowing lets functions read without moves or clones; the aliasing rule is the borrow checker in one line.

## Example

`add(&p1, &p2)` returns a fresh owned `Point`; both inputs stay usable — ownership never moved.

## Code

```rust
#[derive(Debug)]
struct Point(i32, i32);
fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}
fn main() {
    let (p1, p2) = (Point(3, 4), Point(10, 20));
    println!("{:?} + {:?} = {:?}", p1, p2, add(&p1, &p2));
}
```

## Code Explanation

- `&Point` parameters borrow read-only; the returned `Point` is new and owned, while both inputs remain valid afterwards.
- Broken variant (does not compile): `let b = &a;` plus `&mut a` with overlapping use fails with
  error[E0502]: cannot borrow `a` as mutable because it is also borrowed as immutable.

## Common Mistakes

- **Mixing `&` and `&mut`**: overlapping shared plus exclusive borrows are rejected — sequence them instead.
- **Assuming scope-long borrows**: they end at last use, so mutation right after often compiles.

## Best Practices

- Default parameters to `&T`; when the checker complains, list the live borrows at that line.

## Exercise

Does this compile? Quote the error, then sequence the borrows:

```rust
// DOES-NOT-COMPILE
fn main() {
    let mut a = 10;
    let b = &a;
    let c = &mut a;
    *c = 20;
    println!("{b}");
}
```

## Solution

No (`cannot borrow a as mutable ... also borrowed as immutable`):

```rust
fn main() {
    let mut a = 10;
    println!("{}", &a);
    let c = &mut a;
    *c = 20;
    println!("{c}");
}
```

The shared read ends before `c` starts, so borrows never overlap.

## Mini Quiz

```yaml
- question: "What does &T lend?"
  options:
    - "Ownership"
    - "Read-only access without ownership"
    - "A heap copy"
    - "Mutable access"
  answer: 1
  explanation: "Shared borrows read without moving ownership."
- question: "State the aliasing rule."
  options:
    - "Free overlap"
    - "Many shared OR one exclusive at a time"
    - "One shared ever"
    - "Unlimited exclusive"
  answer: 1
  explanation: "Shared xor one-exclusive is the checker's core."
- question: "When does a borrow end?"
  options:
    - "At scope end"
    - "At its last use"
    - "At exit"
    - "Never"
  answer: 1
  explanation: "Non-lexical lifetimes end borrows after final use."
```
