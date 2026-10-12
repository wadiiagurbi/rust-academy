---
slug: luhn
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: testing
chapterTitle: "Chapter 19 — Testing"
chapterOrder: 19
title: "Exercise: Luhn Validation"
difficulty: beginner
order: 4
estimatedMinutes: 15
tags: [rust, testing, exercise]
---

## Learning Objective

By the end of this lesson, you will implement the Luhn checksum in a cargo project and drive it green with tests.

## Explanation

This is a **cargo task**: `cargo new luhn_lab`, implement `luhn_check` in `src/main.rs`, satisfy three tests with `cargo test`. The **Luhn algorithm** (a checksum catching typos in card numbers) runs right-to-left: last digit undoubled, double every second digit, subtract 9 above 9, sum up — valid when the total is a multiple of 10. `char::to_digit(10)` converts a character to its value (`None` for non-digits). `todo!()` compiles while tests fail loudly until the body lands.

## Why It Matters

Checksums are real input hygiene (payments, IDs, barcodes); test-driving one practises red-fix-green.

## Example

```text
test accepts_known_valid_number ... ok
test rejects_single_digit_change ... ok
test rejects_non_digits ... ok
```

## Code

```rust
fn luhn_check(code: &str) -> bool { todo!() }

fn main() { println!("{}", luhn_check("4539578763621486")); }

#[test]
fn accepts_known_valid_number() { assert!(luhn_check("4539578763621486")); }

#[test]
fn rejects_single_digit_change() { assert!(!luhn_check("4539578763621487")); }

#[test]
fn rejects_non_digits() { assert!(!luhn_check("4539-5787")); }
```

## Code Explanation

- `&str` borrow: card numbers exceed integer ranges, so text is the right input. The criteria pin the contract: valid passes, one-digit typo fails, non-digits fail.

## Common Mistakes

- **Doubling from the left**: anchor at the rightmost digit via `code.chars().rev()`.
- **Skipping invalid characters**: return `false` on the first non-digit instead.
- **Skipping subtract-9**: doubled digits above 9 must drop by 9, or valid numbers fail.

## Best Practices

- Toggle doubling with a `bool` flag — clearer than index arithmetic. Reject empty input explicitly (empty sum `0` would pass `0 % 10 == 0`).

## Exercise

Implement `luhn_check` so all three tests pass and `cargo run` prints `true`.

## Solution

Replace `todo!()` with the body below, keeping the stub's `main` and three tests in place:

```rust
fn luhn_check(code: &str) -> bool {
    if code.is_empty() { return false; }
    let mut sum = 0u32;
    let mut double = false;
    for ch in code.chars().rev() {
        let Some(mut digit) = ch.to_digit(10) else { return false; };
        if double {
            digit *= 2;
            if digit > 9 { digit -= 9; }
        }
        sum += digit;
        double = !double;
    }
    sum % 10 == 0
}

fn main() { println!("{}", luhn_check("4539578763621486")); }
```

All three stub tests now pass: valid `true`, typo `false`, `-` input early `false`.

## Mini Quiz

```yaml
- question: "What makes a number Luhn-valid?"
  options:
    - "Ascending digits"
    - "Right-to-left doubling (minus 9 over 9) sums to a multiple of 10"
    - "Exactly 16 digits"
    - "Even first digit"
  answer: 1
  explanation: "Process from the right; valid means total % 10 == 0."
- question: "`ch.to_digit(10)` for `'-'`?"
  options:
    - "0"
    - "`None`, signalling a non-digit"
    - "Its ASCII code"
    - "It panics"
  answer: 1
  explanation: "`to_digit` yields `Option<u32>` — `None` for non-digits."
- question: "Why `code.chars().rev()`?"
  options:
    - "Faster hashing"
    - "Doubling parity is anchored at the rightmost digit"
    - "It skips the first char"
    - "Rust requires it"
  answer: 1
  explanation: "Left-to-right iteration would misalign the pattern."
```
