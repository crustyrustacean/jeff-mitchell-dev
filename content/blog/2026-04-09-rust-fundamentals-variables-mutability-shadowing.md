+++
title = "Variables, Mutability, and Shadowing"
date = 2026-04-09
description = "How variables work in Rust, why they're immutable by default, and how shadowing lets you transform values."
categories = ["Fundamentals"]
tags = ["variables-and-bindings"]
draft = false
aliases = ["/2026-04-09-rust-fundamentals-variables-mutability-shadowing"]
+++

Variables are fundamental to any programming language, and Rust handles them in a way that might surprise you if you're coming from another language. The core idea is straightforward, but Rust layers on some important safety features that are worth understanding from the start.

### Binding Variables

In Rust, creating a variable and giving it a value is called binding. You use the `let` keyword:

```rust
let album = "Attero Dominatus";
```

This binds the string slice "Attero Dominatus" to a variable named `album`. The term "binding" rather than "assignment" is deliberate in Rust, and the distinction becomes important once you learn about ownership in later articles. For now, just know that `let` creates a variable and associates a value with it.

Rust can infer the type of most variables from the value you give them, but you can also be explicit with a type annotation:

```rust
let year: u32 = 2006;
```

This tells the compiler that `year` is an unsigned 32-bit integer. In most cases the inference is fine and you don't need the annotation, but being explicit can help with readability.

### Immutability by Default

Here's where Rust differs from most languages. Variables are immutable by default, meaning once you bind a value, you can't change it. If you try to reassign `album` later in your program, the compiler will stop you with an error.

This might feel restrictive at first, but it's one of Rust's strongest safety features. In many languages, bugs come from variables changing when you didn't expect them to. Maybe a value gets modified in a function you didn't realize was being called, or another thread changes something while you're reading it. Immutability by default eliminates an entire category of these bugs by making you be deliberate about which values are allowed to change.

When a variable genuinely needs to change, you opt in with the `mut` keyword:

```rust
let mut band = "Sabaton";
band = "Metallica";
```

The `mut` keyword signals to anyone reading your code that this variable is expected to change. It's a small thing, but it makes the intent of your code much clearer. When you see a `let` without `mut`, you know that value won't change for the rest of its scope.

### Shadowing

Rust allows you to declare a new variable with the same name as an existing one, which is called shadowing. The new variable "shadows" the previous one, and any subsequent code sees the new value:

```rust
let x = 5;
let x = x + 1;
println!("The value of x is: {}", x);
```

The first line binds 5 to `x`. The second line creates a new variable also called `x`, taking the original value and adding 1. The output is 6.

At first glance, shadowing looks a lot like mutability, and you might wonder why both exist. The key difference is that shadowing creates an entirely new variable, it just happens to reuse the name. This means you can change the type of the value, which `mut` doesn't allow:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

The first `spaces` is a string slice. The second `spaces` is an integer (the length of that string). This is perfectly valid because we're not modifying the original variable, we're creating a new one that shadows it. If you tried this with `mut` instead, the compiler would give you a type mismatch error because you'd be trying to store an integer in a variable that was declared as a string slice.

Shadowing is particularly useful when you need to transform a value through several steps but don't need the intermediate versions. Each new `let` binding is immutable, so you get the safety of immutability while still being able to work through a transformation.

### Where to Go from Here

Understanding variables, mutability, and shadowing gives you the foundation for working with data in Rust. The next article covers scalar data types, which determine what kinds of values your variables can hold.

### Resources

[The Rust Programming Language, Chapter 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)