+++
title = "Aliasing Xor Mutability: A New Way to Learn Rust?"
date = 2026-11-16
description = "Learning Rust via the notion that aliasing and mutability are mutually exclusive"
categories = ["Beginner Guides"]
tags = ["rust", "memory"]
draft = true
+++

It's time to get away from projects, AI, courses, and tutorial hell.

Let's learn Rust *from first principles*. The inspiration for this thought is [here](https://blog.kodewerx.org/2025/08/are-we-teaching-rust-effectively.html). I'm not sure I'm the most qualified person to take you on this journey, but if you're willing to be patient, maybe we can learn a fresh angle of thought together.

<!-- more -->

Rust is the programming language which enables everyone to create reliable and efficient software. This mantra is part of what drew me too it. How does Rust achieve this lofty goal? As we all know, there is no garbage collector, you're following a set of rules, enforced by the compiler, which eliminate entire classes of memory related bugs.

What are these rules?

I went through the basics in [The Memory Doesn't Remain](/2026-04-20-rust-fundamentals-memory-ownership-borrowing/), so I won't repeat them here. You can try to learn these things individually, perhaps in combination, but the result is what many refer to as "fighting the borrow checker".

What if there was another way, some guiding principle you could follow that adds texture and makes the rules easier to *reason about*?

## Enter Aliasing Xor Mutability

What the heck am I talking about. Aliasing Xor Mutability (AxM) is *the* law that makes Rust safe:

> "You can have many readers OR one writer, but not both at the same time."

When you think in these terms, the fight with the borrow checker kind of turns one sided, with you delivering right hooks, rather than the other way around.

### A Definition

Let's see if we can define this AxM business with a bit more care. At any point in time, a piece of data may have:

- any number of immutable (read-only) aliases OR
- exactly one mutable (write) reference, BUT never both at once

Let's break this down some more.

- Aliasing -> multiple references pointing to the same memory, be it stack or heap.
  - example: `let a = &x; let b = &x;` -> both the variables a and b are *immutable* aliases
  - it's as if you build something, put it on a shelf, and there it sits for anyone to pick up and inspect, *but not change*

- Mutability -> the ability to *change* the value at a memory location, be it on the stack or heap.
  - example: `let c = &mut x;` -> the variable c can modify the value of x.

- XOR (exclusive or) -> you *must* pick one mode:
  - alias mode: many readers, no writers.
  - mutability mode: one writer, no readers.

A further way to think of this is phrase it as *readers* OR *one writer*, but not both at the same time.

### Why It's Important

When you think about it, this one principle is the very foundation that Rust sits on. It:

- prevents data races (you just can't have concurrent read/write chaos).
- prevents undefined behaviour from aliasing mutable pointers.
- forces you the programmer to *think through* sharing/ownership and make it explicit, so then the compiler can do it's thing and guarantee safety.

### Simple Examples

Let's wrap it up with some super simple examples:

✅ Legal (many aliases, no mutation):

```rust
let x = 5;
let a = &x;
let b = &x;  // fine, many immutable borrows
println!("{}, {}", a, b);
```

❌ Illegal (alias + mut):

```rust
let mut x = 5;
let a = &x;
let b = &mut x; // ERROR: can't borrow `x` as mutable because it’s also borrowed as immutable
```

✅ Legal (mut exclusive):

```rust
let mut x = 5;
let b = &mut x; // fine, one mutable borrow
*b += 1;
```

## Back to Ownership and Borrowing

Let's get back to a couple of the familar rules. A piece of data in Rust can only have one owner. When the owner goes out of scope, the value is dropped and can't be used again. Fundamentally, ownership answers: "Who is responsible for cleaning up?"

Ownership enables borrowing. The owner decides whether to lend out immutable (`&T`) or mutable (`&mut T`) references. Borrowing *enforces* AxM. If the owner lends out a reference to it's data, others can read it but they *cannot* change it. If the owner lends out a mutable reference, then only the lendee (so to speak) is able to make changes. Both of these concepts are enforced by the compiler. Enforcement of the ownership rules prevents messy, dangling references (unfreed memory, which can be exploited) and then AxM prevents unsafe aliasing (changes being made when you don't want/expect them).

If you had the notion of ownership, without AxM, you'd prevent memory from not being properly cleaned up, but you'd still allow potential race conditions. 

If you had AxM without the notion of ownership, you'd prevent unauthorized use of memory, but you'd have not ability to clean up.

In short, ownership answers: *"Who owns this memory and when does it get freed?"*

AxM answers: *"While it's alive, how can a piece of memory be safely accessed?"*

## How Does this Make it Easier to Learn Rust?

AxM is kind of like the glue or missing piece that brings everything into context. It goes unmentioned in introductory talk about Rust's memory model. I hope you find it helpful to think of as a distinct "thing".

Keeping the concept of AxM in mind, is a good way of beating the borrow check at it's own game.
