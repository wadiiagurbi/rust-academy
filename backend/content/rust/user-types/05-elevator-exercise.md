---
slug: elevator-events
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: user-types
chapterTitle: "Chapter 6 — User-Defined Types"
chapterOrder: 6
title: "Exercise: Elevator Events"
difficulty: beginner
order: 5
estimatedMinutes: 15
tags: [rust, exercise, enums, structs]
---

## Learning Objective

By the end of this lesson, you will model elevator events with nested
enums in a Cargo project so `main` prints each case.

## Explanation

An **enum** lists allowed cases (**variants**); variants may carry
data as tuple payloads (`CarArrived(Floor)`) or unit cases with none
(`CarDoorOpened`). **Nesting** puts one enum inside another
(`ButtonPressed(Button)` holds a `Button`), grouping related ideas:
lobby calls versus car presses both count as buttons.

A **type alias** (`type Floor = i32;`) nicknames `i32` for readability
without adding checks. `#[derive(Debug)]` auto-generates `{:?}`
printing; `#![allow(dead_code)]` (a crate-level opt-out) silences
"never constructed" notes while the exercise builds data only. A
**Cargo task** here is `cargo new`, editing `src/main.rs`, and proving
with `cargo run` plus assertions.

Constructor helpers (`fn car_arrived(floor: i32) -> Event`) wrap raw
values in the right variant so callers never spell the nesting.

## Why It Matters

Event hierarchies drive UIs, servers, and controllers: one `Event`
type covers every input while staying exhaustive for later `match`.
Building them now prepares pattern-matching chapters that consume
these values.

## Example

A lobby press on floor 0 going up becomes `ButtonPressed(LobbyCall(Up,
0))`; arrival, door open/close, and car press follow. Printing each
with `{:?}` shows the nested path explicitly.

## Code

```rust
#![allow(dead_code)]
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
enum Event {
    CarArrived(i32),
    CarDoorOpened,
}

fn main() {
    let e = Event::CarArrived(0);
    println!("{e:?}");
}
```

## Code Explanation

- `#![allow(dead_code)]` applies to the whole file (**inner
  attribute**, note `!`), quieting unused-variant lints during
  construction-only tasks.
- `#[derive(Debug)]` on each enum enables `{:?}` printing of tags
  and payloads without hand-written formatting.
- `CarArrived(i32)` is a tuple variant carrying a floor; `Up` and
  `Down` are unit variants of `Direction`.
- `Event::CarArrived(0)` builds the value; `println!("{e:?}")`
  prints `CarArrived(0)`.

## Common Mistakes

- **Missing `derive(Debug)`**: printing then fails with "does not
  implement Debug". Derive on every enum in the hierarchy.
- **Flattening instead of nesting**: separate top variants for each
  button kind work but scatter logic; a nested `Button` groups them.
- **SwappingLobbyCall args**: keep `(Direction, Floor)` order
  consistent between definition and constructors to avoid flips.

## Best Practices

- Model "kind plus data" with nested enums rather than bare ints and
  strings, keeping invalid combinations unrepresentable.
- Provide tiny constructors (`car_arrived`, `lobby_call_pressed`)
  so call sites read as domain actions.
- Run `cargo fmt` and `cargo clippy` after enum edits to keep variant
  layout and lints clean.

## Exercise

Create the Cargo task and define events so `main` runs:

```bash
cargo new elevator_exercise
cd elevator_exercise
cargo run
```

Replace `src/main.rs` with this stub, then add variants and bodies:

```rust
#![allow(dead_code)]
#[derive(Debug)]
enum Event {
    // TODO: add variants
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

fn car_arrived(floor: i32) -> Event {
    todo!()
}

fn car_door_opened() -> Event {
    todo!()
}

fn car_door_closed() -> Event {
    todo!()
}

fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    todo!()
}

fn car_floor_button_pressed(floor: i32) -> Event {
    todo!()
}

fn main() {
    println!("{:?}", lobby_call_button_pressed(0, Direction::Up));
    println!("{:?}", car_arrived(0));
    println!("{:?}", car_door_opened());
    println!("{:?}", car_floor_button_pressed(3));
    println!("{:?}", car_door_closed());
    println!("{:?}", car_arrived(3));
}
```

Criteria: `cargo run` prints six lines without panic; every helper
returns the matching variant.

## Solution

Nest `Button` inside `Event` with an alias for floors:

```rust
#![allow(dead_code)]
#[derive(Debug)]
enum Event {
    ButtonPressed(Button),
    CarArrived(Floor),
    CarDoorOpened,
    CarDoorClosed,
}

type Floor = i32;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
enum Button {
    LobbyCall(Direction, Floor),
    CarFloor(Floor),
}

fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

fn main() {
    println!("{:?}", lobby_call_button_pressed(0, Direction::Up));
    println!("{:?}", car_arrived(0));
    println!("{:?}", car_door_opened());
    println!("{:?}", car_floor_button_pressed(3));
    println!("{:?}", car_door_closed());
    println!("{:?}", car_arrived(3));
}
```

It prints the lobby call, arrival, door opened, car press, door
closed, and arrival in order. Tuple variants carry floors while unit
variants mark door states.

## Mini Quiz

```yaml
- question: "Why nest `Button` inside `Event`?"
  options:
    - "To slow compilation"
    - "To group related button cases under one hierarchy"
    - "To allow null values"
    - "To avoid derive(Debug)"
  answer: 1
  explanation: "Nesting keeps lobby and car presses together as button kinds."
- question: "What does `type Floor = i32` give you?"
  options:
    - "A checked new type"
    - "A readable nickname for i32"
    - "A constant value"
    - "A new crate"
  answer: 1
  explanation: "Aliases document intent without changing type checking."
- question: "What does #![allow(dead_code)] do here?"
  options:
    - "Publishes the crate"
    - "Quiets unused warnings while building data only"
    - "Runs the tests"
    - "Formats the code"
  answer: 1
  explanation: "Construction-only exercises trigger dead-code notes otherwise."
```
