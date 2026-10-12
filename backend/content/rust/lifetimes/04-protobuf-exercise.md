---
slug: protobuf-exercise
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: lifetimes
chapterTitle: "Chapter 16 — Lifetimes"
chapterOrder: 16
title: "Exercise: Protobuf Parsing"
difficulty: advanced
order: 4
estimatedMinutes: 20
tags: [rust, exercise, lifetimes, parsing]
---

## Learning Objective

By the end of this lesson, you will parse protobuf-style varint fields with zero-copy borrowed slices.

## Explanation

**Protobuf** (compact binary encoding) writes each field as a tag (field number plus **wire type**, here `Varint` for integers) plus payload. **Varint** packs integers 7 bits per byte, top bit marking continuation: `[0x10, 0x2a]` is field 2, value 42. Zero-copy parsing borrows input slices instead of copying, so remainders are tied by lifetimes to the buffer.

## Why It Matters

Slices in, views out is the pattern behind real decoders; lifetimes tie views to buffers.

## Example

Tag `0x10` unpacks to field 2, wire type 0; the varint `0x2a` reads as 42 with an empty remainder.

## Code

```rust
fn parse_varint(data: &[u8]) -> (u64, &[u8]) {
    let mut v = 0u64;
    for (i, b) in data.iter().enumerate() {
        v |= ((*b & 0x7f) as u64) << (7 * i);
        if *b & 0x80 == 0 { return (v, &data[i + 1..]); }
        if i == 6 { break; } }
    panic!("bad varint"); }
fn main() {
    let (v, rest) = parse_varint(&[0x2a]);
    assert_eq!((v, rest.len()), (42, 0));
}
```

## Code Explanation

- Each byte contributes 7 bits; a clear top bit ends the number, and the tail slice starts right after — value and remainder both borrow the input.
- Broken variant (does not compile): `struct Field { data: &[u8] }` without a lifetime fails with `error[E0106]: missing lifetime specifier`.

## Common Mistakes

- **Copying payloads**: borrow slices instead; always thread the remainder forward or fields misalign.

## Best Practices

- Return `(parsed, rest)` pairs and loop until input is empty; panic only in exercises, ship `Result` decoders instead.

## Exercise

Run `cargo new proto_exercise`, fill the `todo!()` stubs, and pass `cargo run` (prints `proto ok`; tag `0x10` gives field 2, value 42; slices only):

```rust
// Stub: fill the todo!() bodies (panics until filled)
fn parse_varint(data: &[u8]) -> (u64, &[u8]) { todo!() }
fn parse_id(data: &[u8]) -> (u64, &[u8]) { todo!() }
fn main() {
    let (v, rest) = parse_varint(&[0x2a]);
    assert_eq!((v, rest.len()), (42, 0));
    let (id, rest) = parse_id(&[0x10, 0x2a]);
    assert_eq!((id, rest.len()), (42, 0));
    println!("proto ok");
}
```

## Solution

Accumulate 7 bits per byte, then route on the tag:

```rust
fn parse_varint(data: &[u8]) -> (u64, &[u8]) {
    let mut v = 0u64;
    for (i, b) in data.iter().enumerate() {
        v |= ((*b & 0x7f) as u64) << (7 * i);
        if *b & 0x80 == 0 { return (v, &data[i + 1..]); }
        if i == 6 { break; } }
    panic!("bad varint"); }
fn parse_id(data: &[u8]) -> (u64, &[u8]) {
    let (tag, rest) = parse_varint(data);
    assert_eq!(tag, 0x10);
    parse_varint(rest)
}
fn main() {
    let (id, rest) = parse_id(&[0x10, 0x2a]);
    assert_eq!((id, rest.len()), (42, 0));
    println!("proto ok");
}
```

## Mini Quiz

```yaml
- question: "What does tag 0x10 encode?"
  options:
    - "Field 2, wire type Varint"
    - "Field 16, wire type Len"
    - "Length 16 only"
    - "Checksum 10"
  answer: 0
  explanation: "Number routes the field; wire type shapes the payload."
- question: "Why do remainders carry a lifetime?"
  options:
    - "Heap allocation"
    - "Ties borrows to the buffer's span"
    - "Mutation"
    - "Skipping parse"
  answer: 1
  explanation: "Slices must not outlive viewed bytes."
- question: "How does varint mark continuation?"
  options:
    - "Top bit set means more bytes follow"
    - "Zero separators"
    - "Length prefix"
    - "Newlines"
  answer: 0
  explanation: "7 payload bits plus one continue flag per byte."
```
