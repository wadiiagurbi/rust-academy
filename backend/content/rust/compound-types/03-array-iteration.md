---
slug: array-iteration
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: compound-types
chapterTitle: "Chapter 4 — Compound Types"
chapterOrder: 4
title: "Array Iteration"
difficulty: beginner
order: 3
estimatedMinutes: 8
tags: [rust, arrays, iteration, for]
---

## Learning Objective

By the end of this lesson, you will loop arrays with `for` and check
items with asserts.

## Explanation

**Iteration** visits each element once. `for elem in array` walks
`Copy` integers by value via the **`IntoIterator` trait** (the
iterator interface, details later). Ranges like `2..prime` exclude
the end. `assert_ne!` **panics** (halts) when values wrongly match;
`assert_eq!` checks equality. `for` is bounds-safe with no indexes to
mistype.

## Why It Matters

Check-every-item loops express primes and totals in two lines with no
index bookkeeping.

## Example

Primes `[2, 3, 5, 7, 11]` test divisors `2..prime`; a composite like
9 panics on `9 % 3 == 0`.

## Code

```rust
fn main() {
    let primes = [2, 3, 5, 7, 11];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0, "{prime} % {i}");
        }
    }
    println!("all prime");
}
```

## Code Explanation

- Outer copies each `i32` into `prime`; inner tests divisors.
- `%` is remainder; message shows only on failure.

## Common Mistakes

- **`2..=prime` for primes**: tests `prime % prime` and always fails.
- **Index loops unneeded**: iterate values when position is unused.

## Best Practices

- Iterate values directly; attach messages naming bad items.
- Move assertion loops into `#[test]` functions when ready.

## Exercise

How many checks for `[2, 3]`? Validate this list:

```rust
fn main() {
    let nums = [2, 3, 5];
    for n in nums {
        println!("n: {n}");
    }
}
```

## Solution

`2` runs zero, `3` runs one:

```rust
fn main() {
    for prime in [2, 3, 5] {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
        println!("{prime} ok");
    }
}
```

Prints `2 ok`, `3 ok`, `5 ok`. With 9 it would panic.

## Mini Quiz

```yaml
- question: "for x in int array does what?"
  options:
    - "Borrows whole array"
    - "Visits each by value"
    - "Sorts in place"
    - "Returns length"
  answer: 1
  explanation: "Copy integers arrive one per round."
- question: "2..prime yields?"
  options:
    - "Through prime"
    - "Up to excluding prime"
    - "Only prime"
    - "Infinite"
  answer: 1
  explanation: ".. excludes the end."
- question: "Failed assert_ne!?"
  options:
    - "Warns on"
    - "Panics with message"
    - "Skips round"
    - "Formats code"
  answer: 1
  explanation: "Asserts halt naming values."
```
