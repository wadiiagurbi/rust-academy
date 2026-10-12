---
slug: structs
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: user-types
chapterTitle: "Chapter 6 — User-Defined Types"
chapterOrder: 6
title: "Structs"
difficulty: beginner
order: 1
estimatedMinutes: 10
tags: [rust, structs, types]
---

## Learning Objective

By the end of this lesson, you will define structs and pass them by
reference.

## Explanation

A **struct** bundles named **fields** with types, as in `struct Person
{ name: String, age: u8 }`. Build with literals `Person { name, age
}`; **shorthand init** reuses matching variable names. Read with
**field access** (`person.name`). Pass `&Person` (shared borrow) so
owned `String` stays with callers. No inheritance exists; structs
compose instead.

## Why It Matters

Named fields beat `.0` indexing and flag missing data at compile
time.

## Example

`Peter` ages 27→28 via `mut`; `Avery` builds via shorthand. One
`describe(&person)` helper prints both.

## Code

```rust
struct Person {
    name: String,
    age: u8,
}

fn describe(p: &Person) {
    println!("{} is {}", p.name, p.age);
}

fn main() {
    let p = Person { name: String::from("Peter"), age: 27 };
    describe(&p);
}
```

## Code Explanation

- `String::from` converts `&str` to owned text for the field.
- `&Person` borrows; dot access reads fields for display.

## Common Mistakes

- **Moving `p.name` out**: borrow or clone to keep ownership.
- **Missing field**: literals need all fields; compiler names gaps.

## Best Practices

- Borrow in readers; own only when keeping values.
- Use shorthand when locals match field names.

## Exercise

Build `Avery` via shorthand and age `Peter`:

```rust
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Avery");
    let age = 39;
}
```

## Solution

Shorthand plus `mut` birthday:

```rust
struct Person {
    name: String,
    age: u8,
}

fn describe(p: &Person) {
    println!("{} is {}", p.name, p.age);
}

fn main() {
    let mut peter = Person { name: String::from("Peter"), age: 27 };
    peter.age = 28;
    describe(&peter);
    let name = String::from("Avery");
    let age = 39;
    describe(&Person { name, age });
}
```

## Mini Quiz

```yaml
- question: "What is a struct field?"
  options:
    - "Index like .0"
    - "Named value with fixed type"
    - "Function pointer"
    - "Dependency"
  answer: 1
  explanation: "Fields pair names and types."
- question: "Person { name, age } needs?"
  options:
    - "Nothing"
    - "Matching vars name and age"
    - "Empty struct"
    - "Release mode"
  answer: 1
  explanation: "Shorthand reuses in-scope names."
- question: "Why &Person for describe?"
  options:
    - "Move ownership"
    - "Borrow keeping ownership"
    - "Allow null"
    - "Grow struct"
  answer: 1
  explanation: "Borrows inspect without moves."
```
