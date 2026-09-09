+++
title = "Constants"
date = 2026-04-08
description = "How to declare and use constants in Rust to replace magic values in your code."
categories = ["Fundamentals"]
tags = ["constants"]
draft = false
aliases = ["/2026-04-08-rust-fundamentals-constants"]
+++

In the [previous article](@/blog/2026-04-07-rust-fundamentals-comments.md) we covered comments. Now let's look at constants, which are one of the simplest but most practical features in any programming language.

### What Constants Are For

Constants help you avoid what are sometimes called "magic values," which are raw numbers or strings hardcoded directly into your code. Imagine a program that uses the number 299792458 in a dozen different calculations. If you later realize you need to adjust that value, or if someone else reads your code and has no idea what that number represents, things get messy quickly.

A constant gives that value a meaningful name and a single place where it's defined. If it ever needs to change, you update it in one spot and every use of it throughout your program picks up the new value.

### Declaring a Constant

Here's how to declare the speed of light as a constant in Rust:

```rust
const SPEED_OF_LIGHT: u32 = 299_792_458;
```

The `const` keyword starts the declaration, followed by the name. Rust's naming convention for constants is `SCREAMING_SNAKE_CASE`, meaning all uppercase letters with words separated by underscores. A type annotation is required after the name, since the compiler won't infer types for constants. Finally, the value is bound with the `=` operator.

Notice the underscores in the number. Rust lets you use underscores as visual separators in numeric literals to make large numbers more readable. `299_792_458` is the same value as `299792458`, just easier on the eyes.

### How Constants Differ from Variables

Constants might sound like immutable variables, and it's natural to wonder why both exist. There are a few important differences.

Constants must have a value that can be determined at compile time. You can't compute a constant from something that's only known when the program runs, like user input or the current time. The value has to be fixed before the program is ever executed.

Constants don't just default to immutable the way variables do. They are always immutable, with no option to add `mut`. This makes sense given that their purpose is to represent values that genuinely never change.

Constants can be declared in any scope, including at the global level outside of any function. This makes them available across your entire program, which is exactly what you want for shared values like configuration limits, mathematical constants, or conversion factors.

```rust
const MAX_CONNECTIONS: u32 = 100;
const PI: f64 = 3.14159265358979;

fn main() {
    println!("Maximum allowed connections: {}", MAX_CONNECTIONS);
    println!("Pi is approximately: {}", PI);
}
```

In contrast, variables created with `let` must live inside a function and are scoped to the block they're declared in. Constants are a way to give a name to a value that has meaning across your whole program, while variables are for values that are part of the logic within a specific function.

### Where to Go from Here

Constants are a small feature, but using them well is a habit that keeps your code readable and maintainable as it grows. The next article covers variables, mutability, and shadowing, which will give you a much fuller picture of how Rust handles data.

### Resources

[The Rust Programming Language, Chapter 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
