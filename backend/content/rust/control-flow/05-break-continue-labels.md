---
slug: break-continue
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: control-flow
chapterTitle: "Chapter 3 — Control Flow"
chapterOrder: 3
title: "Break, Continue, Labels"
difficulty: beginner
order: 5
estimatedMinutes: 8
tags: [rust, break, continue, labels]
---

## Learning Objective

By the end of this lesson, you will exit with `break`, skip with
`continue`, and target outer loops by label.

## Explanation

**`break`** exits the loop; in `loop` it may carry a value (`break
i;`), making `loop` an expression. **`continue`** skips to the next
**iteration** (one pass). A **label** (`'outer:`) names a loop so
`break 'outer;` exits outer nesting, not just the inner loop. Define
with colon (`'outer: for`), use without (`break 'outer;`).

## Why It Matters

Searches stop at hits (`break`); filters skip items (`continue`).
Labels avoid flag variables in grid scans.

## Example

Odds under five print via `continue` on evens. A 3x3 search for 10
uses `'outer` to stop both loops at once.

## Code

```rust
fn main() {
    let mut i = 0;
    let done = loop {
        i += 1;
        if i % 2 == 0 {
            continue;
        }
        if i >= 5 {
            break i;
        }
    };
    println!("done: {done}");
}
```

## Code Explanation

- Evens `continue`; first odd past five `break`s `5` into `done`. Only `loop` returns such values.

## Common Mistakes

- **Value from while/for**: only `loop` takes `break value`.
- **Colon misplaced**: colon only at definition, not at use.

## Best Practices

- Show exit conditions near the top for visibility.
- Use labels only for two-plus nesting levels.

## Exercise

What prints?

```rust
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("odd: {i}");
    }
}
```

## Solution

Prints odds 1, 3, 5:

```rust
fn main() {
    let grid = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut checked = 0;
    'outer: for row in grid {
        for cell in row {
            checked += 1;
            if cell == 10 {
                break 'outer;
            }
        }
    }
    println!("checked: {checked}");
}
```

Prints `checked: 6`. Plain `break` would exit inner only.

## Mini Quiz

```yaml
- question: "What does continue do?"
  options:
    - "Exits all loops"
    - "Skips to next iteration"
    - "Restarts program"
    - "Waits for input"
  answer: 1
  explanation: "continue starts the next round."
- question: "Which loop returns break values?"
  options:
    - "while"
    - "for"
    - "loop"
    - "None"
  answer: 2
  explanation: "Only loop exits solely via break."
- question: "Labels are for?"
  options:
    - "Variables"
    - "Targeting outer loops"
    - "Strings"
    - "Imports"
  answer: 1
  explanation: "Labels reach outer nesting levels."
```
