---
slug: embedded-hal
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: bare-metal
chapterTitle: "Chapter 25 — Bare-Metal Rust"
chapterOrder: 25
title: "Microcontrollers and the HAL"
difficulty: advanced
order: 2
estimatedMinutes: 12
tags: [rust, embedded, hal]
---

## Learning Objective

By the end of this lesson, you will describe a **HAL** (hardware abstraction layer) and drive a mock pin through its two operations.

## Explanation

A **microcontroller** combines CPU, RAM, and peripherals (GPIO pins, timers, serial ports) in one chip. Firmware flips **registers** (fixed memory addresses controlling hardware) to toggle pins. A **PAC** (peripheral access crate) maps those addresses to typed structs; a **HAL** builds portable traits on top — `embedded-hal` defines `set_high`/`set_low` so drivers work across chips. Real register code needs a board target and will not link on host:

```text
EXCERPT-ONLY (needs a board target; will not link on host):
uart.cr.write(|w| w.enable().set_bit());
```

## Why It Matters

HAL traits separate portable logic from chip specifics: write once against `OutputPin`, run on STM32, nRF, and ESP32 via different HAL crates.

## Example

Blink logic against a mock pin implementing the two HAL operations.

## Code

```rust
trait OutputPin { fn set_high(&mut self); fn set_low(&mut self); fn is_high(&self) -> bool; }
struct MockPin { level: bool, flips: u32 }
impl MockPin { fn new() -> Self { Self { level: false, flips: 0 } } }
impl OutputPin for MockPin {
    fn set_high(&mut self) { self.level = true; self.flips += 1; }
    fn set_low(&mut self) { self.level = false; self.flips += 1; }
    fn is_high(&self) -> bool { self.level }
}
fn blink(pin: &mut impl OutputPin, times: u32) {
    for _ in 0..times { pin.set_high(); pin.set_low(); }
}
fn main() {
    let mut pin = MockPin::new();
    blink(&mut pin, 2);
    println!("high={} flips={}", pin.is_high(), pin.flips);
}
```

## Code Explanation

- `impl OutputPin` accepts any implementor — the portable-driver shape HAL crates use. Prints `high=false flips=4`.
- The mock records transitions instead of touching hardware, so host tests verify blink logic exactly.

## Common Mistakes

- **Coding to one chip's PAC**: register-level drivers cannot move chips. Code to HAL traits; adapt at the edge.
- **Ignoring pin roles**: driving a pin the hardware owns (e.g. a debug line) can brown out the board. Check the datasheet first.
- **Blocking delays everywhere**: busy-waiting stalls all tasks. Prefer timers or async waits on firmware.

## Best Practices

- Accept `impl OutputPin` in drivers; construct with concrete pins in board setup.
- Unit-test drivers against mocks on host; reserve flashing for integration checks.

## Exercise

Add `fn pulse(pin: &mut impl OutputPin)` setting high then low once. Assert the pin ends low and `flips` grew by 2.

## Solution

```rust
trait OutputPin { fn set_high(&mut self); fn set_low(&mut self); fn is_high(&self) -> bool; }
struct MockPin { level: bool, flips: u32 }
impl MockPin { fn new() -> Self { Self { level: false, flips: 0 } } }
impl OutputPin for MockPin {
    fn set_high(&mut self) { self.level = true; self.flips += 1; }
    fn set_low(&mut self) { self.level = false; self.flips += 1; }
    fn is_high(&self) -> bool { self.level }
}
fn pulse(pin: &mut impl OutputPin) { pin.set_high(); pin.set_low(); }
fn main() {
    let mut pin = MockPin::new();
    pulse(&mut pin);
    assert!(!pin.is_high());
    assert_eq!(pin.flips, 2);
    println!("ok flips={}", pin.flips);
}
```

Prints `ok flips=2`: one pulse, exactly two transitions, ending low.

## Mini Quiz

```yaml
- question: "What is a HAL?"
  options:
    - "A compiler backend"
    - "A portable trait layer over chip registers"
    - "A kind of heap"
    - "A test runner"
  answer: 1
  explanation: "Drivers code to traits like OutputPin; chips provide impls."
- question: "What is a PAC?"
  options:
    - "A package manager"
    - "Typed structs mapping hardware register addresses"
    - "A panic handler"
    - "A pinout diagram PDF"
  answer: 1
  explanation: "The PAC is the chip-specific bottom layer under the HAL."
- question: "Why test drivers against mocks on host?"
  options:
    - "Mocks run the hardware"
    - "Logic is verified fast without flashing boards"
    - "Hardware tests are impossible"
    - "It avoids writing drivers"
  answer: 1
  explanation: "Host tests catch logic bugs; hardware runs confirm wiring."
```
