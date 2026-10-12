---
slug: binary-tree
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: smart-pointers
chapterTitle: "Chapter 14 — Smart Pointers"
chapterOrder: 14
title: "Exercise: Binary Tree"
difficulty: intermediate
order: 4
estimatedMinutes: 15
tags: [rust, exercise, box, binary-tree]
---

## Learning Objective

By the end of this lesson, you will implement an owned binary search tree with `Box` so ordering checks pass.

## Explanation

A **binary search tree** keeps smaller values left, larger right, each stored once. `Subtree(Option<Box<Node>>)` (a **newtype** naming the optional child) keeps links pointer-sized and owned. Guards like `Some(n) if v < n.v` steer insertion and lookup.

## Why It Matters

Trees combine every ownership idea: `Box` for recursion, `&mut` to insert, `&` to look up.

## Example

Inserting `2, 1, 3, 2` gives length 3 (duplicate kept once); `has(3)` is true, `has(9)` false.

## Code

```rust
struct Subtree(Option<Box<i32>>);
fn main() { let t = Subtree(Some(Box::new(1))); assert!(t.0.is_some()); }
```

## Code Explanation

- `Option<Box<i32>>` is an owned, possibly-absent heap link; the full tree below nests this pattern with left/right children.

## Common Mistakes

- **Direct `Node` fields**: infinite size — box every recursive link; on equal values do nothing so each stays unique.

## Best Practices

- Put `insert`/`has`/`len` on a `Subtree` newtype; recurse with `&mut` to insert, `&` to look up.

## Exercise

Run `cargo new tree_exercise`, implement the methods, and pass `cargo test` (BST order, no duplicates, `has`/`len` correct):

```rust
// Stub: fill the todo!() bodies (panics until filled)
#[derive(Debug)]
struct Node { v: i32, left: Subtree, right: Subtree }
#[derive(Debug)]
struct Subtree(Option<Box<Node>>);
impl Subtree {
    fn insert(&mut self, v: i32) { todo!() }
    fn has(&self, v: i32) -> bool { todo!() }
    fn len(&self) -> usize { todo!() }
}
fn main() {}
```

## Solution

Left when smaller, right when larger, ignore equal:

```rust
#[derive(Debug)]
struct Node { v: i32, left: Subtree, right: Subtree }
#[derive(Debug)]
struct Subtree(Option<Box<Node>>);
impl Subtree {
    fn insert(&mut self, v: i32) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node { v, left: Subtree(None), right: Subtree(None) })),
            Some(n) if v < n.v => n.left.insert(v),
            Some(n) if v > n.v => n.right.insert(v),
            _ => {} } }
    fn has(&self, v: i32) -> bool {
        match &self.0 {
            None => false,
            Some(n) if v < n.v => n.left.has(v),
            Some(n) if v > n.v => n.right.has(v),
            _ => true, } }
    fn len(&self) -> usize { match &self.0 { None => 0, Some(n) => 1 + n.left.len() + n.right.len() } }
}
fn main() {
    let mut r = Subtree(None);
    for v in [2, 1, 3, 2] { r.insert(v); }
    assert_eq!((r.len(), r.has(3), r.has(9)), (3, true, false));
}
```

## Mini Quiz

```yaml
- question: "Why Option<Box<Node>> children?"
  options:
    - "Null raw pointers"
    - "Possibly-absent owned child with fixed size"
    - "Sharing between trees"
    - "Skipping order"
  answer: 1
  explanation: "Option covers absence; Box gives size plus ownership."
- question: "insert on equal values must?"
  options:
    - "Insert left"
    - "Do nothing, one copy only"
    - "Panic"
    - "Replace"
  answer: 1
  explanation: "Each value is stored once."
- question: "How are drops handled?"
  options:
    - "Manual free calls"
    - "Boxes cascade from the root automatically"
    - "A collector frees them"
    - "They leak"
  answer: 1
  explanation: "Owned boxes drop their children in turn."
```
