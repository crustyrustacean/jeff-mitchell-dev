+++
title = "De-mystifying Generics"
date = 2026-11-08
description = "One struct definition, many concrete types: where Message<String> comes from, and why it won't share a Vec with Message<i32>."
categories = ["Field Notes"]
tags = ["rust", "syntax", "generics"]
draft = false
+++

Generic types are something I struggle with. I sort of get the concept: you define a container that can accommodate many kinds of data. What I don't have is enough experience to know when to make something generic. It doesn't come to me as an obvious choice.

This past week, a little of it came into focus. Mostly because I had one detail backwards.

Here's the struct I was playing with:

```rust
struct Message<T> {
    id: i32,
    content: T,
}
```

The `<T>` on the struct declaration is what makes `Message` generic; `content: T` only *uses* that parameter. That distinction is the part I kept missing, because a field typed `T` looks like the source of the genericness. It isn't. `content: T` is legal only because the struct already declared `T`. The `id` field has nothing to do with it; that one is an `i32` in every `Message` you'll ever build.

## Where I was backwards

To use it, I wrote this:

```rust
fn main() {
    let msg = Message {
        id: 1,
        content: "This is some content".to_string(),
    };

    println!("Message id: {}", msg.id);
    println!("Message content: {}", msg.content);
}
```

And for a long time I would have told you that somewhere in here, I *told* the compiler that `T` was a `String`.

I didn't. There's no `Message::<String>` and no type annotation anywhere in that code. The compiler looked at the expression on the right of `content:`, saw a `String`, and inferred `T = String` from it. The type parameter gets declared once, on the struct; each value you construct pins it down. You almost never say it out loud.

That's the correction I needed. I had been imagining generics as a form of annotation, when they're closer to a hole that the constructor fills in. Once I saw it that way, `Vec<T>`, `Option<T>`, and `Result<T, E>` all stopped looking like special cases and became the same move: declared once, decided at use.

The two `println!` calls above only work because `T` happens to be `String`, by the way. Printing with `{}` requires `Display`, and `String` has it. That's not generics doing anything clever. It's just a type that's already on board.

## Why bother

This example is a little silly. If I know the content is a `String`, I can write `content: String` and be done with it. The generic version buys me nothing *yet*.

Where it earns its keep is when the content isn't always text. Watch what happens when I build two:

```rust
let text = Message { id: 1, content: "hello".to_string() };
let count = Message { id: 2, content: 42 };
```

That's a single struct definition producing two concrete types: `Message<String>` and `Message<i32>`. Not "a `Message` that could be anything." Those are the actual types the compiler is working with, decided separately for each value. The generic parameter isn't a placeholder left open at runtime; it's a slot that gets filled in at compile time, once per instantiation.

Because they're different types, they don't mix:

```rust
let mut messages = vec![text];
messages.push(count);
// error[E0308]: mismatched types
//   expected `Message<String>`, found `Message<i32>`
```

I hit that and initially read it as the compiler being difficult. It's the opposite. `messages` is a `Vec<Message<String>>`, and it has to be homogeneous, because every element occupies the same layout. The error is the type system telling me the truth about what I built: two distinct types that happen to share a definition. If I want one list of both, I have to unify them deliberately, either by making the content a shared type or by wrapping it in an enum that names each case.

That's the actual payoff of generics, and it's narrower than "future-proofing." It's *one definition, many distinct types, all checked*. I write `Message` once; I don't write `MessageString`, `MessageI32`, and `MessagePrice`. The compiler generates the variants for me and keeps them apart.

## Adding methods

One more piece, because this is where the syntax gets noisy and where I used to give up.

```rust
impl<T> Message<T> {
    fn id(&self) -> i32 {
        self.id
    }
}
```

The `<T>` on the `impl` has to match the struct's parameter, or the block doesn't belong to it. This method returns the `id`, which is always an `i32`, so it works for every `Message` no matter what `T` is. No bounds needed.

The moment a method's body needs something *about* `T` that the declaration doesn't guarantee, you have to say so:

```rust
impl<T: std::fmt::Display> Message<T> {
    fn show(&self) {
        println!("[{}] {}", self.id, self.content);
    }
}
```

Without that bound, `println!("{}", self.content)` won't compile. `T` could be anything, and "anything" doesn't promise to be printable. The compiler's error message says exactly this (`T` doesn't implement `std::fmt::Display`) and suggests the bound, which is how I learned it. This second `impl` only applies to `Message<T>` where `T: Display`; the `id()` method above still applies to all of them.

Two `impl` blocks for one struct, one unbounded and one bounded, is normal, not a trick. It reads as: here's what every `Message` can do, and here's what a printable `Message` can additionally do.

## Rules to keep

The type parameter is declared once, on the struct or the function; each use fills the hole and gets a concrete type of its own. Generics don't defer the decision to runtime and they don't make a thing "any type" — they let one piece of source text stand for many distinct types, and they keep those types from being confused for each other. When I catch myself writing a second struct that differs only in one field's type, that's the signal. One definition, and let the compiler hand out the variants.

## References

- [The Rust Programming Language, Chapter 10.1: Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [The Rust Programming Language, Chapter 10.2: Traits — Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust By Example: Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
