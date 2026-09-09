+++
title = "The Result Type"
date = 2026-04-23
description = "How Rust handles recoverable errors with the Result type, including pattern matching, the ? operator, and common methods like map_err and unwrap_or."
categories = ["Fundamentals"]
tags = ["result-type"]
draft = false
aliases = ["/2026-04-23-rust-fundamentals-result-type"]
+++

In the [previous article](@/blog/2026-04-22-rust-fundamentals-option-type.md) we looked at `Option`, which represents a value that might not exist. `Result` is its close companion. Where `Option` answers the question "is there a value?", `Result` answers the question "did this operation succeed, and if not, why?"

`Result` is Rust's primary tool for error handling, and it replaces the try/catch exception model used by languages like Java, Python, and JavaScript. Instead of throwing an error and hoping something catches it, Rust functions return a `Result` that the caller must deal with. The compiler won't let you ignore it.

### What is Result?

`Result` is an enum in the standard library with two variants:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Like `Option`, it uses generic type parameters. `T` is the type of the success value, wrapped in `Ok`. `E` is the type of the error value, wrapped in `Err`. And like `Option`, `Result` is in the prelude, so you never need to import it.

The key difference from `Option` is that `Err` carries information about what went wrong, not just the fact that something failed. This makes `Result` much more useful for error handling because the caller can inspect the error and decide how to respond.

### A Basic Example

Let's revisit the division example from the Option article, but this time using `Result` so we can explain why the operation failed:

```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let result = divide(10.0, 0.0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
```

The `divide` function returns `Result<f64, String>`. On success, it returns the calculation wrapped in `Ok`. On failure, it returns an error message wrapped in `Err`. The caller uses `match` to handle both cases.

Compare this with the `Option` version from the previous article, where failure returned `None` with no explanation. With `Result`, the caller knows not just that something failed, but exactly what went wrong. In a real program, the error type would usually be something more structured than a `String`, but this illustrates the concept.

### Result vs Option

Since these two types are so similar, it's worth being clear about when to use each:

Use `Option` when absence is normal and expected, not an error. A search function that might not find a match, an optional configuration field, a user's middle name. There's nothing wrong with the absence, there's just nothing there.

Use `Result` when failure means something went wrong and the caller needs to know why. File I/O, network requests, parsing, division by zero. The operation was attempted and it didn't work, and the error value explains the reason.

If your function can fail and the caller would reasonably ask "why did it fail?", use `Result`. If your function might simply have nothing to return and that's fine, use `Option`.

### The ? Operator

Pattern matching on every `Result` works, but in practice most functions call several operations that could fail. Writing a `match` for each one makes the code deeply nested and hard to read. This is where the `?` operator comes in, and it's the single most important thing to learn about `Result` in practice.

The `?` operator does two things: if the `Result` is `Ok`, it unwraps the value and lets execution continue. If it's `Err`, it immediately returns the error from the current function. Here's what code looks like without it:

```rust
use std::fs;
use std::io;

fn read_and_count(path: &str) -> Result<usize, io::Error> {
    let contents = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return Err(e),
    };

    Ok(contents.len())
}
```

And here's the same function with `?`:

```rust
use std::fs;
use std::io;

fn read_and_count(path: &str) -> Result<usize, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.len())
}
```

The `?` after `fs::read_to_string(path)` replaces the entire `match` block. If the file read succeeds, the string is unwrapped and bound to `contents`. If it fails, the error is returned from `read_and_count` immediately. The calling function then handles the error however it sees fit.

There's one requirement: the function using `?` must itself return a `Result` (or `Option`) because `?` needs somewhere to propagate the error to. You'll get a compiler error if you try to use `?` in a function that returns something else.

The `?` operator is what makes Rust's error handling practical at scale. Without it, every fallible call would need its own `match` block and the code would be more error handling than actual logic. With it, the happy path reads cleanly from top to bottom and errors propagate automatically.

### Chaining with ?

The real power of `?` shows up when a function calls multiple fallible operations:

```rust
use std::fs;
use std::io;

fn first_line_length(path: &str) -> Result<usize, io::Error> {
    let contents = fs::read_to_string(path)?;
    let first_line = contents.lines().next().unwrap_or("");
    Ok(first_line.len())
}
```

Each `?` is an early exit point. If any step fails, the function returns immediately with that error. If everything succeeds, execution flows straight through to the final `Ok`. This keeps the code flat and readable even when there are many potential failure points.

### Common Result Methods

Like `Option`, `Result` has a set of methods that make common operations more concise.

**unwrap() and expect()**

These work the same as their `Option` counterparts. `unwrap()` returns the `Ok` value or panics on `Err`. `expect()` does the same but with a custom panic message:

```rust
let value: Result<i32, String> = Ok(42);
let n = value.unwrap(); // 42

let bad: Result<i32, String> = Err(String::from("failed"));
let n = bad.expect("This should have worked"); // panics with your message
```

The same advice applies: use these in prototypes and tests, not in production code where you should handle errors properly.

**unwrap_or() and unwrap_or_default()**

Provide a fallback value instead of panicking:

```rust
let result: Result<i32, String> = Err(String::from("failed"));
let value = result.unwrap_or(0); // 0
```

**map() and map_err()**

Transform the success value or the error value without unwrapping:

```rust
let result: Result<i32, String> = Ok(5);
let doubled = result.map(|n| n * 2); // Ok(10)

let result: Result<i32, String> = Err(String::from("not found"));
let mapped = result.map_err(|e| format!("Error: {}", e)); // Err("Error: not found")
```

`map()` transforms the `Ok` value and passes through `Err` unchanged. `map_err()` does the opposite: it transforms the `Err` value and passes through `Ok` unchanged. These are useful when you need to convert between error types or adjust a result before passing it along.

**is_ok() and is_err()**

Simple boolean checks:

```rust
let good: Result<i32, String> = Ok(42);
let bad: Result<i32, String> = Err(String::from("failed"));

println!("{}", good.is_ok());  // true
println!("{}", bad.is_err());  // true
```

### Where to Go from Here

`Option` and `Result` together form the foundation of how Rust handles the absence of values and the possibility of failure. The pattern is consistent: wrap your success case in `Some` or `Ok`, wrap your failure case in `None` or `Err`, and let the type system ensure the caller handles both possibilities.

As your programs grow, you'll find yourself wanting to define custom error types rather than using `String` for everything. You might also encounter the `anyhow` and `thiserror` crates, which are widely used in the Rust community to make error handling more ergonomic. These are topics for future articles, but the fundamentals covered here will carry you through a lot of real-world Rust code.

### Resources

[The Rust Programming Language, Chapter 9.2](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

[Rust Standard Library: Result](https://doc.rust-lang.org/std/result/enum.Result.html)

[Rust By Example: Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
