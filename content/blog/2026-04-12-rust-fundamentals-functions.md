+++
title = "Functions"
date = 2026-04-12
description = "Declaring functions in Rust, with parameters, return values, and the difference between statements and expressions."
categories = ["Fundamentals"]
tags = ["functions"]
draft = false
aliases = ["/2026-04-12-rust-fundamentals-functions"]
+++

Functions are how you organize code in Rust. They let you break a program into smaller, named pieces, each with a clear job, which makes everything easier to read, test, and change later. We've already been using one function in every program so far, the `main` function, which serves as the entry point for every Rust binary. Now it's time to write our own.

### Declaring a Function

A function is declared with the `fn` keyword, followed by a name, a pair of parentheses for parameters, and a body wrapped in curly braces:

```rust
fn remaining_lyrics() {
    println!("...give me that which I desire!");
}

fn main() {
    println!("Give me fuel, give me fire...");
    remaining_lyrics();
}
```

```bash
Output:
Give me fuel, give me fire...
...give me that which I desire!
```

The `remaining_lyrics` function takes no parameters and doesn't return anything. It just prints a line to the console. The `main` function prints the first part of the lyric, then calls `remaining_lyrics()` to finish it.

You'll notice I've declared `remaining_lyrics` above `main`. This is a personal preference. Unlike some languages, Rust doesn't care about the order you declare your functions in. The compiler can see all of them regardless of where they appear in the file.

Rust uses `snake_case` for function names, meaning lowercase words separated by underscores. The compiler will actually warn you if you use a different convention, so it's best to follow it from the start.

### Parameters

Functions become more useful when they can accept input. Values passed into a function are called parameters, and each one must include a type annotation. Rust won't infer parameter types for you the way it does with `let` bindings, so this is one place where you always need to be explicit.

Let's expand our example to accept some parameters:

```rust
fn remaining_lyrics(band: &str, album: &str, song: &str) {
    println!("...give me that which I desire! {} - {} - {}", band, album, song);
}

fn main() {
    println!("Give me fuel, give me fire...");
    remaining_lyrics("Metallica", "Reload", "Fuel");
}
```

```
Output:
Give me fuel, give me fire...
...give me that which I desire! Metallica - Reload - Fuel
```

The function now accepts three string slices as parameters. Each one has the `&str` type annotated after a colon. When we call the function from `main`, we pass the values inside the parentheses in the same order the parameters are declared. If you forget a type annotation or pass the wrong number of arguments, the compiler will catch it for you.

### Return Values

Functions can also return a value back to the code that called them. To indicate that a function returns something, you add `->` followed by the return type after the parentheses:

```rust
fn year_released() -> u32 {
    1991
}

fn main() {
    println!("The Black Album, by Metallica, was released in: {}", year_released());
}
```

```
Output:
The Black Album, by Metallica, was released in: 1991
```

The `year_released` function declares that it returns a `u32` (an unsigned 32-bit integer). The body of the function contains just `1991` with no semicolon. This is intentional, and it leads to one of Rust's more important concepts.

### Statements and Expressions

Rust draws a clear line between statements and expressions, and understanding the difference is essential to working with functions effectively.

A statement performs an action but does not produce a value. Variable bindings are statements, as are calls to `println!`:

```rust
let x = 5;           // statement - binds a value, produces nothing
println!("{}", x);    // statement - prints to console, produces nothing
```

An expression evaluates to a value. Numbers, math operations, function calls, and blocks enclosed in curly braces are all expressions:

```rust
5           // expression - evaluates to 5
x + 1       // expression - evaluates to the sum
year_released()  // expression - evaluates to whatever the function returns
```

The practical consequence shows up in how functions return values. In the `year_released` function above, `1991` is an expression that evaluates to an integer, and because it's the last thing in the function body, that value becomes what the function returns. If you were to add a semicolon after `1991`, it would turn the expression into a statement, the function would no longer return a value, and the compiler would give you an error.

This is a rule worth committing to memory: the last expression in a function body, without a semicolon, is the return value. The `return` keyword exists in Rust and is useful for returning early from a function before reaching the end, but for the common case of returning the final result, leaving off the semicolon is the idiomatic approach.

### Where to Go from Here

Functions provide the structure that makes programs manageable. The next couple of articles will cover conditional logic and loops, which give you the tools to control what happens inside your functions and when. Combined, these concepts form the core of writing useful programs in Rust.

### Resources

[The Rust Programming Language, Chapter 3.3](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)