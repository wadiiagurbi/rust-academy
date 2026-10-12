---
slug: thiserror-anyhow
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: error-handling
chapterTitle: "Chapter 20 — Error Handling"
chapterOrder: 20
title: "Thiserror and Anyhow"
difficulty: intermediate
order: 4
estimatedMinutes: 12
tags: [rust, errors, thiserror, anyhow]
---

## Learning Objective

By the end of this lesson, you will split duties: `thiserror` for library error types, `anyhow` for application context.

## Explanation

**`thiserror`** serves libraries: `#[derive(Error)]` (auto-implements the `Error` trait) with `#[error("...")]` format strings builds `Display` + `From` from a compact enum — structured errors minus boilerplate. **`anyhow`** serves applications: `anyhow::Result<T>` erases the concrete error into a boxed catch-all, and `.context("...")` (attaches a what-were-you-doing message) builds cause chains. Rule: libraries expose matchable `thiserror` variants; binaries report with `anyhow`. Install via `cargo add thiserror anyhow`.

> Needs-cargo-dependency: excerpt — verify in a cargo project with both crates in `Cargo.toml`, not bare `rustc`.

## Why It Matters

Hand-written impls cost dozens of lines per type; these crates shrink that to attributes, and `context()` turns "not found" into "failed to load config: not found".

## Example

A typed config error (`thiserror`), consumed with context (`anyhow`) at the application edge.

## Code

```rust
// Cargo.toml: thiserror = "1", anyhow = "1"
use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("config value `{value}` is not a number: {reason}")]
    BadNumber { value: String, reason: String },
    #[error("missing required key `{0}`")]
    MissingKey(String),
}

fn read_port(text: &str) -> Result<u16, ConfigError> {
    let line = text.lines().next().ok_or(ConfigError::MissingKey("port".into()))?;
    line.trim().parse().map_err(|e: std::num::ParseIntError| ConfigError::BadNumber {
        value: line.into(),
        reason: e.to_string(),
    })
}

fn main() -> Result<()> {
    let text = std::fs::read_to_string("app.conf").context("failed to load app.conf")?;
    let port = read_port(&text).map_err(|e| anyhow::anyhow!(e))?;
    println!("port: {port}");
    Ok(())
}
```

## Code Explanation

- `#[derive(Error)]` generates `Display`/`Error` per variant; `{value}`/`{0}` interpolate fields.
- `map_err` (transforms a `Result`'s error side) wraps parse failures with the offending value; `.context()` lifts I/O errors with a breadcrumb.

## Common Mistakes

- **`anyhow` in library APIs**: callers lose matchable variants. Keep `thiserror` public; `anyhow` for binaries/tests.
- **Bare `anyhow!("bad port")`**: discards the source. Prefer `.context()`/`map_err` so `Caused by:` survives.

## Best Practices

- Libraries: precise variants with fields for matchable values.
- Applications: `anyhow::Result` plus `.context()` at each boundary; print with `{err:?}` for the full chain.

## Exercise

Read a missing file through `.context("failed to load nope.conf")` and run it. What shows under `Caused by:`?

## Solution

```rust
// Cargo.toml: thiserror = "1", anyhow = "1"
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let text = std::fs::read_to_string("nope.conf").context("failed to load nope.conf")?;
    println!("{text}");
    Ok(())
}
```

Prints `Error: failed to load nope.conf` plus `Caused by: No such file or directory` — operation named, reason preserved.

## Mini Quiz

```yaml
- question: "`thiserror` vs `anyhow`?"
  options:
    - "thiserror for matchable library types; anyhow for application context chains"
    - "Identical; pick randomly"
    - "anyhow for libraries"
    - "Neither for production"
  answer: 0
  explanation: "Libraries owe variants; applications owe readable reports."
- question: "What does `.context(...)` do?"
  options:
    - "Retries"
    - "Layers a what-were-you-doing message over the source error"
    - "Converts warnings"
    - "Hides the error"
  answer: 1
  explanation: "Rendered as `Error:` plus `Caused by:` lines."
- question: "`#[derive(Error)]` generates?"
  options:
    - "Only Debug"
    - "Display text plus standard Error plumbing"
    - "A main fn"
    - "Serialization"
  answer: 1
  explanation: "Display, Error, and From conversions from the attributes."
```
