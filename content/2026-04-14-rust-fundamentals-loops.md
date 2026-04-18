+++
title = "Loops"
date = 2026-04-14
description = "Rust's three loop types: loop, while, and for, including returning values from loops and using ranges."
categories = ["Fundamentals"]
tags = ["control-flow"]
draft = false
+++

In the previous article on [Conditional Logic](@/2026-04-13-rust-fundamentals-conditional-logic.md), we looked at how `if` and `else if` expressions let a program choose between different code paths. Loops take things further by allowing a program to repeat a block of code until some condition is met or a collection has been fully processed. Rust provides three kinds of loops: `loop`, `while`, and `for`.

### loop

The `loop` keyword tells Rust to execute a block of code repeatedly until you explicitly tell it to stop with the `break` keyword. At its simplest, a `loop` with no `break` runs forever:

```rust
fn main() {
    loop {
        println!("Help, I'm stuck in a loop!");
    }
}
```

This will print the same message endlessly until you press `ctrl+c` in the terminal to kill the program. Not very useful on its own, but `loop` becomes interesting when you combine it with `break` to exit at the right moment.

What makes `loop` distinct from Rust's other looping constructs is that it can return a value. This is useful when you need to compute something through repeated iterations and then hand the result back to the rest of your program. You do this by placing the value you want to return right after the `break` keyword:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

Here we increment `counter` on each pass through the loop. When it reaches 10, we break out and return `counter * 2`, which means `result` ends up with the value 20. The whole `loop` block acts as an expression that gets bound to the variable `result`. This ability to return values from a loop is something you won't find in many other languages, and it comes in handy more often than you might expect.

### while

A `while` loop repeats for as long as a condition remains true. If you find yourself writing a `loop` with an `if` check and a `break` just to control when to stop, a `while` loop is almost always the cleaner choice.

Here's a simple countdown:

```rust
fn main() {
    let mut number = 10;
    while number != 0 {
        println!("Counting down...{}!", number);
        number -= 1;
    }

    println!("Liftoff!!!");
}
```

We start with `number` set to 10. On each pass through the loop, we print the current value and subtract 1. Once `number` reaches 0, the condition `number != 0` is no longer true and execution moves on to the final message. The intent is immediately clear from reading the `while` line, which is the main advantage over using a bare `loop` for this kind of task.

### for

The `for` loop is used to iterate over a collection of items. Rather than managing a counter variable and checking a condition yourself, you hand `for` something to iterate over and it processes each element in turn. This is the loop you'll reach for most often in Rust.

Here's an example that iterates over an array:

```rust
fn main() {
    let albums = ["Coat of Arms", "The Last Stand", "The Great War"];

    for album in albums {
        println!("Now playing: {}", album);
    }
}
```

The `for` loop walks through each element in the `albums` array, binding it to the variable `album` for each pass. There's no index to manage and no risk of accidentally going past the end of the array. Rust handles all of that for you.

The `for` loop also works with ranges, which are created using the `..` syntax. This is handy when you need to do something a specific number of times or generate a sequence of numbers:

```rust
fn main() {
    for number in (1..10).rev() {
        println!("{}!", number);
    }
    println!("We have liftoff!!!");
}
```

There are a couple of things worth noting here. The range `1..10` generates numbers from 1 up to but not including 10, so you get 1 through 9. The upper bound being exclusive is something that trips people up at first, so keep it in mind. The `.rev()` method reverses the range so that it counts down from 9 to 1 instead of up. This is your first glimpse of method chaining on an iterator, which is a pattern that shows up everywhere in Rust once you start working with collections.

### Where to Go from Here

These three loop types, combined with the conditional logic from the previous article, give you the tools to control how your program flows through its instructions. Over the next couple of articles, I'll put these fundamentals into practice by building a few small programs. It will be a chance to see how variables, functions, conditionals, loops, and Rust's standard library all work together to create something real.

### Resources

[The Rust Programming Language, Chapter 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html)