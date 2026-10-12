---
slug: rtc-driver
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: bare-metal
chapterTitle: "Chapter 25 — Bare-Metal Rust"
chapterOrder: 25
title: "Exercise: An RTC Driver"
difficulty: advanced
order: 5
estimatedMinutes: 20
tags: [rust, embedded, rtc, exercise]
---

## Learning Objective

By the end of this exercise, you will model an **RTC** (real-time clock: battery-backed timekeeping) with validated sets and rollover reads.

## Explanation

An RTC keeps wall time across reboots on a small battery; firmware reads hour/minute/second registers and sets them through bounded writes. Reads and writes must validate ranges (hours `0..24`, minutes and seconds `0..60`) because glitches produce values like `25:99`. A **mock** (a host stand-in for the registers) proves the logic on a PC; the same checks run unchanged against real MMIO later. Only the mock program below compiles on host.

## Why It Matters

Timestamps order sensor data and schedule wake-ups. Validated RTC access turns silent clock corruption into catchable errors.

## Example

Read back a set time, and reject an out-of-range write before it touches hardware.

## Code

```rust
#[derive(Debug, PartialEq)]
struct Time { hour: u8, min: u8, sec: u8 }
struct MockRtc { time: Time }
impl MockRtc {
    fn new() -> Self { Self { time: Time { hour: 0, min: 0, sec: 0 } } }
    fn set(&mut self, t: Time) -> Result<(), &'static str> {
        if t.hour >= 24 { return Err("hour out of range"); }
        if t.min >= 60 || t.sec >= 60 { return Err("min/sec out of range"); }
        self.time = t; Ok(())
    }
}
fn main() {
    let mut rtc = MockRtc::new();
    rtc.set(Time { hour: 12, min: 30, sec: 45 }).expect("valid");
    println!("{:02}:{:02}:{:02}", rtc.time.hour, rtc.time.min, rtc.time.sec); println!("{:?}", rtc.set(Time { hour: 25, min: 0, sec: 0 }));
}
```

## Code Explanation

- `{:02}` is a **format spec** printing two digits, zero-padded; `set` validates before storing, mirroring how firmware guards register writes. Prints `12:30:45` then the hour error.

## Common Mistakes

- **Writing then validating**: storing first corrupts the clock on bad input. Check every field before touching state.
- **Ignoring read validation**: glitch reads can exceed ranges too. Validate on read when re-reads are possible.
- **`u8` complacency**: `u8` holds to 255, not 23 — the type alone never enforces clock ranges.

## Best Practices

- Validate all three fields up front; return which unit failed so logs pinpoint the caller bug.
- Keep `Time` small and comparable; add tick/rollover helpers only after set/read are solid.

## Exercise

Add `fn tick(&mut self)` advancing one second with rollover (`23:59:59 -> 00:00:00`). From `23:59:58`, tick twice and assert midnight.

## Solution

```rust
#[derive(Debug, PartialEq)]
struct Time { hour: u8, min: u8, sec: u8 }
struct MockRtc { time: Time }
impl MockRtc {
    fn new() -> Self { Self { time: Time { hour: 0, min: 0, sec: 0 } } }
    fn set(&mut self, t: Time) -> Result<(), &'static str> {
        if t.hour >= 24 || t.min >= 60 || t.sec >= 60 { return Err("out of range"); }
        self.time = t; Ok(())
    }
    fn tick(&mut self) {
        self.time.sec += 1;
        if self.time.sec == 60 { self.time.sec = 0; self.time.min += 1; }
        if self.time.min == 60 { self.time.min = 0; self.time.hour += 1; }
        if self.time.hour == 24 { self.time.hour = 0; }
    }
}
fn main() {
    let mut rtc = MockRtc::new();
    rtc.set(Time { hour: 23, min: 59, sec: 58 }).expect("valid");
    rtc.tick(); rtc.tick();
    assert_eq!(rtc.time, Time { hour: 0, min: 0, sec: 0 }); println!("midnight ok");
}
```

Prints `midnight ok`: cascaded rollover handled unit by unit.

## Mini Quiz

```yaml
- question: "What does an RTC do?"
  options:
    - "Counts CPU cycles only"
    - "Tracks wall time across reboots on battery power"
    - "Manages wifi"
    - "Allocates heap"
  answer: 1
  explanation: "Battery backing keeps hours/minutes/seconds alive without main power."
- question: "Why validate before storing?"
  options:
    - "Validation is slower after"
    - "Bad input must never reach hardware state"
    - "The compiler requires it"
    - "Reads are impossible"
  answer: 1
  explanation: "Check-then-write keeps the clock trustworthy on every path."
- question: "Why is u8 not enough for hours?"
  options:
    - "u8 cannot hold 23"
    - "u8 allows 24..=255, which are invalid hours"
    - "u8 is signed"
    - "u8 is slower"
  answer: 1
  explanation: "Range checks, not width, enforce clock semantics."
```
