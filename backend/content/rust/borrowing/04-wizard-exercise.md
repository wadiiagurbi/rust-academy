---
slug: wizard-inventory
track: rust
trackTitle: "Rust"
trackOrder: 1
chapter: borrowing
chapterTitle: "Chapter 15 — Borrowing"
chapterOrder: 15
title: "Exercise: Wizard Inventory"
difficulty: beginner
order: 4
estimatedMinutes: 15
tags: [rust, exercise, borrowing, ownership]
---

## Learning Objective

By the end of this lesson, you will move spells into a wizard's inventory and mutate them without tripping the checker.

## Explanation

`add_spell(&mut self, spell: Spell)` moves each spell into the `Vec`, which becomes the owner. `cast_spell(&mut self, name: &str)` (**`&str`** is a borrowed string slice) needs exclusivity: it lowers `uses`, spends `mana`, and may `remove` the spent spell. `position` (index search with a predicate) plus short borrows keeps the checker happy: never hold an element borrow across a `remove`.

## Why It Matters

"Find, check, update, maybe remove" shapes game state, carts, and queues; sequencing borrows here pays off everywhere.

## Example

One-shot `Fire` (10 mana) from 20 mana: after casting, mana is 10 and the spell is gone.

## Code

```rust
struct Spell { name: String, cost: u32, uses: u32 }
fn main() {
    let mut spells = vec![Spell { name: "Fire".into(), cost: 10, uses: 1 }];
    spells[0].uses -= 1;
    println!("uses left: {}", spells[0].uses);
}
```

## Code Explanation

- Each index statement borrows briefly and ends before the next starts; the owned spell mutates in place with no clone.
- Broken variant (does not compile): holding `let s = &mut self.spells[i];`, calling `self.spells.remove(i)`, then reading `s.uses`
  fails with error[E0499]: cannot borrow `self.spells` as mutable more than once at a time.

## Common Mistakes

- **Borrow then remove**: copy `cost`/`uses` out (or re-index) before `remove`; both methods need `&mut self` since they mutate the wizard.

## Best Practices

- Store owned data (`Vec<Spell>`), query by borrow (`&str`); remove by index only after element borrows end.

## Exercise

Run `cargo new wizard_exercise`, fill the `todo!()` stubs, and pass `cargo run` (spends mana, lowers `uses`, removes at zero, ignores unknown/short-mana casts):

```rust
// Stub: fill the todo!() bodies (panics until filled)
struct Spell { name: String, cost: u32, uses: u32 }
struct Wizard { spells: Vec<Spell>, mana: u32 }
impl Wizard {
    fn new(mana: u32) -> Self { Self { spells: vec![], mana } }
    fn add_spell(&mut self, spell: Spell) { todo!() }
    fn cast_spell(&mut self, name: &str) { todo!() }
}
fn main() {
    let mut w = Wizard::new(20);
    w.add_spell(Spell { name: "Fire".into(), cost: 10, uses: 1 });
    w.cast_spell("Fire");
    assert_eq!((w.mana, w.spells.len()), (10, 0));
}
```

## Solution

Index, copy small fields, mutate with fresh borrows:

```rust
struct Spell { name: String, cost: u32, uses: u32 }
struct Wizard { spells: Vec<Spell>, mana: u32 }
impl Wizard {
    fn new(mana: u32) -> Self { Self { spells: vec![], mana } }
    fn add_spell(&mut self, spell: Spell) { self.spells.push(spell); }
    fn cast_spell(&mut self, name: &str) { let Some(i) = self.spells.iter().position(|s| s.name == name) else { return };
        let (cost, uses) = (self.spells[i].cost, self.spells[i].uses);
        if uses == 0 || self.mana < cost { return; }
        self.mana -= cost;
        self.spells[i].uses -= 1;
        if self.spells[i].uses == 0 { self.spells.remove(i); } }
}
fn main() {
    let mut w = Wizard::new(20);
    w.add_spell(Spell { name: "Fire".into(), cost: 10, uses: 1 });
    w.cast_spell("Fire");
    assert_eq!((w.mana, w.spells.len()), (10, 0)); println!("mana: {}, spells: {}", w.mana, w.spells.len());
}
```

## Mini Quiz

```yaml
- question: "Why does add_spell take Spell by value?"
  options:
    - "To borrow briefly"
    - "To move ownership into the vector"
    - "To clone"
    - "To drop it"
  answer: 1
  explanation: "The Vec becomes the new owner."
- question: "Why &mut self on cast_spell?"
  options:
    - "It only reads"
    - "It mutates mana/uses and may remove"
    - "Always required"
    - "It returns a borrow"
  answer: 1
  explanation: "Writes plus removal need exclusivity."
- question: "How to remove while satisfying the checker?"
  options:
    - "Hold the borrow across remove"
    - "End borrows first, remove by index"
    - "Clone the vector"
    - "Use raw pointers"
  answer: 1
  explanation: "No live borrow may span the mutation."
```
