---
slug: uart-driver
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: bare-metal
chapterTitle: "Chapter 25 — Bare-Metal Rust"
chapterOrder: 25
title: "Writing a UART Driver"
difficulty: advanced
order: 3
estimatedMinutes: 15
tags: [rust, embedded, uart]
---

## Learning Objective

By the end of this lesson, you will implement a polled **UART** (serial hardware sending bytes one at a time) driver against a mock peripheral.

## Explanation

UART sends bytes over a **TX line** (transmit wire) at a **baud rate** (bits per second both ends agree on) with no shared clock. Firmware uses **MMIO registers** (memory addresses wired to hardware): a status register reports `TX_READY` (transmitter free), a data register takes the next byte. A **polled driver** loops on the status bit, then writes. Real MMIO needs the chip target and will not link on host:

```text
EXCERPT-ONLY (needs a chip target; will not link on host):
while uart.sr.read().tx_ready().bit_is_clear() {}
uart.dr.write(|w| w.data().bits(byte));
```

## Why It Matters

Serial logs are the embedded `println!`: bring-up, GPS, and Bluetooth modules all speak UART first. A polled driver is the smallest working step.

## Example

A mock UART with a ready flag and a transmit buffer, driven by `write_byte`.

## Code

```rust
struct MockUart { ready: bool, sent: Vec<u8> }
impl MockUart {
    fn new() -> Self { Self { ready: true, sent: Vec::new() } }
    fn write_byte(&mut self, byte: u8) -> Result<(), &'static str> {
        if !self.ready { return Err("transmitter busy"); }
        self.sent.push(byte); Ok(())
    }
    fn write_str(&mut self, text: &str) -> Result<(), &'static str> {
        for b in text.bytes() { self.write_byte(b)?; } Ok(())
    }
}
fn main() {
    let mut uart = MockUart::new();
    uart.write_str("hi").expect("ready"); println!("{:?}", uart.sent);
}
```

## Code Explanation

- `?` is the **propagation operator**: on `Err` it returns early; the mock's `ready` mirrors the status bit and `sent` mirrors bytes leaving TX. Prints `[104, 105]`.

## Common Mistakes

- **No busy check**: writing while busy drops bytes silently. Poll ready (or return busy) per byte.
- **Blocking forever**: `while !ready {}` with dead hardware hangs the device. Add timeouts on firmware.
- **Baud mismatch**: both ends must agree; a wrong divisor yields garbage that looks like broken code.

## Best Practices

- One byte-level primitive (`write_byte`) plus a small loop for slices; surface busy as an error, not a hang.
- Keep the driver `core`-only so host mocks and firmware share the exact logic.

## Exercise

Add `fn write_line(&mut self, text: &str)` appending `\r\n` after the text. Send `"ok"` and assert `sent` equals `b"ok\r\n"`.

## Solution

```rust
struct MockUart { ready: bool, sent: Vec<u8> }
impl MockUart {
    fn new() -> Self { Self { ready: true, sent: Vec::new() } }
    fn write_byte(&mut self, byte: u8) -> Result<(), &'static str> {
        if !self.ready { return Err("busy"); }
        self.sent.push(byte); Ok(())
    }
    fn write_line(&mut self, text: &str) -> Result<(), &'static str> {
        for b in text.bytes().chain([b'\r', b'\n']) { self.write_byte(b)?; } Ok(())
    }
}
fn main() {
    let mut uart = MockUart::new();
    uart.write_line("ok").expect("ready"); assert_eq!(uart.sent, b"ok\r\n");
    println!("{:?}", uart.sent);
}
```

Prints `[111, 107, 13, 10]`: line discipline lives above the byte primitive.

## Mini Quiz

```yaml
- question: "What does the TX_READY bit mean?"
  options:
    - "A byte arrived"
    - "The transmitter accepts the next byte"
    - "The bus is powered off"
    - "Baud rate changed"
  answer: 1
  explanation: "Polled drivers wait for ready, then write the data register."
- question: "What is the baud rate?"
  options:
    - "Bytes of RAM"
    - "Agreed bits-per-second both UART ends use"
    - "CPU frequency"
    - "Pin voltage"
  answer: 1
  explanation: "Timing agreement replaces a shared clock wire."
- question: "Why one write_byte primitive plus loops?"
  options:
    - "Style only"
    - "Busy handling lives in one place; layers compose above it"
    - "Loops are faster"
    - "Registers need it"
  answer: 1
  explanation: "A single audited primitive keeps error paths consistent."
```
