+++
title = "Methods and Associated Functions"
date = 2026-04-19
description = "How to add behavior to structs in Rust with methods and associated functions using impl blocks."
categories = ["Fundamentals"]
tags = ["method-syntax"]
draft = false
+++

In previous articles I covered [structs](@/2026-04-16-rust-fundamentals-struct-type.md) and [enums](@/2026-04-17-rust-fundamentals-enum-type.md), which let you define custom types to model your data. On their own, though, these types just hold data. To make them do things, you use `impl` blocks to define methods and associated functions. This is where your types start to come alive.

### What is a Method?

A method is a function that's attached to a specific type. Instead of calling it on its own, you call it on an instance of that type using dot notation, like `circle.area()`. If you've used object-oriented languages before, this will feel familiar.

The key difference between a method and a regular function is that a method always receives a reference to the instance it's being called on as its first parameter, written as `&self`.

### Defining Methods with impl

Let's build a practical example. We'll create a `Circle` type and give it methods to calculate its area, diameter, and circumference. As a reminder, the formulas we need are:

- Area: π × r²
- Diameter: 2 × r
- Circumference: 2 × π × r

where r is the radius.

```rust
use std::f32::consts::PI;

struct Circle {
    radius: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        PI * (self.radius * self.radius)
    }

    fn diameter(&self) -> f32 {
        2.0 * self.radius
    }

    fn circumference(&self) -> f32 {
        2.0 * PI * self.radius
    }
}

fn main() {
    let circle = Circle { radius: 15.0 };
    println!("Area: {:.1} square metres", circle.area());
    println!("Diameter: {:.1} metres", circle.diameter());
    println!("Circumference: {:.1} metres", circle.circumference());
}
```

```bash
Area: 706.9 square metres
Diameter: 30.0 metres
Circumference: 94.2 metres
```

There's a lot going on here, so let's walk through it piece by piece.

We pull in the constant `PI` from Rust's standard library with a `use` statement so we don't have to hardcode it. Then we define a `Circle` struct with a single field, `radius`. I've used a named struct here rather than a tuple struct so that `self.radius` reads clearly in the method bodies.

The `impl Circle` block is where the methods live. Everything inside this block is associated with the `Circle` type. The block's name must match the struct's name exactly.

Each method takes `&self` as its first parameter. This is a reference to the instance the method is being called on. When we write `circle.area()`, Rust automatically passes `&circle` as the `self` parameter. Using `&self` means the method borrows the instance without taking ownership of it, so you can keep using it afterward. I haven't covered ownership and borrowing in depth yet, but the practical takeaway for now is that `&self` lets you read data from the struct without consuming it.

Inside each method, we access the struct's fields through `self`. So `self.radius` gives us the radius value for whichever instance the method was called on.

In `main`, calling the methods is clean and readable. You use dot notation on the instance, just like accessing a field, but with parentheses to indicate a function call. All the math is tucked away inside the `impl` block, and the calling code stays focused on what it wants to know rather than how to calculate it.

### The &self Parameter

You'll encounter three forms of `self` in method signatures:

`&self` borrows the instance immutably. The method can read the data but can't change it. This is what you'll use most of the time.

`&mut self` borrows the instance mutably. The method can read and modify the data. You'd use this for methods that need to update a field.

`self` takes ownership of the instance. After calling a method that takes `self`, the original variable can no longer be used. This is rare and is typically reserved for methods that transform the type into something else.

For our circle methods, `&self` is the right choice because we're only reading the radius to perform calculations.

### Associated Functions

Not everything in an `impl` block has to be a method. Functions that don't take `self` as a parameter are called associated functions. They're attached to the type itself rather than to an instance of it, and you call them with `::` syntax instead of dot notation.

The most common use of an associated function is as a constructor:

```rust
impl Circle {
    fn new(radius: f32) -> Circle {
        Circle { radius }
    }

    fn area(&self) -> f32 {
        PI * (self.radius * self.radius)
    }

    fn diameter(&self) -> f32 {
        2.0 * self.radius
    }

    fn circumference(&self) -> f32 {
        2.0 * PI * self.radius
    }
}
```

The `new` function takes a radius value and returns a `Circle` instance. Notice it doesn't have `&self` in its parameter list because there's no existing instance yet. You call it like this:

```rust
let circle = Circle::new(15.0);
```

You've actually been using associated functions throughout these articles without realizing it. `String::from("hello")` is an associated function on the `String` type. Now you know what's happening under the hood.

Rust doesn't have constructors as a built-in language feature the way some languages do. The convention of defining a `new` associated function serves the same purpose, and you'll see it on nearly every type in the standard library and in community crates.

### Where to Go from Here

Methods and associated functions give your custom types behavior, not just data. Everything related to a `Circle` lives in one place, making the code organized and easy to navigate. As your types grow more complex, you can have multiple `impl` blocks for the same type, which becomes useful when implementing traits. Traits are the next major concept on the horizon, and they build directly on what you've learned here.

### Resources

[The Rust Programming Language, Chapter 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)