+++
title = "A Greeting Program"
date = 2026-04-15
description = "Build a small Rust program that takes user input from the terminal and constructs a greeting, using functions for separation of concerns."
categories = ["Fundamentals"]
tags = ["standard-library", "strings"]
draft = false
+++

Over the past several articles I've covered the foundational building blocks of Rust, from variables and data types through to functions and loops. All of that theory is useful, but at some point you need to put it into practice. This post does exactly that by building a small program that asks for your name and prints a greeting back to you.

The output will look like this:

```bash
What is your name? Jeff
Hello, Jeff, nice to meet you!
```

It's not going to win any awards, but this tiny program surfaces several new concepts that are worth understanding. You'll get your first taste of Rust's standard library, learn how to read input from the terminal, work with owned strings, and practice organizing your code into separate functions with clear responsibilities.

### Planning the Program

I'm terrible for jumping straight into code without thinking first, so let me try to be better here. Our program needs to do four things:

1. Ask for the user's name
2. Read the input from the terminal
3. Build a greeting string that includes the name
4. Print the greeting to the console

We'll keep each of these responsibilities in its own function. This is sometimes called "separation of concerns" and it's a habit worth building early, even in tiny programs like this one. Each function does one thing, which makes the code easier to read and change later.

### Bringing the Standard Library into Scope

Before we write any functions, there's a new concept to introduce. Our program needs to read from the terminal and write to it, and those capabilities live in Rust's standard library under `std::io`. To use them, we add a `use` statement at the top of our file:

```rust
use std::io::{self, Write};
```

This brings `io` into scope so we can call `io::stdin()` and `io::stdout()` later. The `Write` trait is needed because we'll be calling `.flush()` on stdout, and that method comes from `Write`. Don't worry too much about traits right now, I'll cover them in a future article. For the moment, just know that this line gives our program access to the tools it needs for terminal input and output.

### Reading the User's Name

```rust
fn get_name() -> String {
    print!("What is your name? ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
```

This function takes no parameters and returns a `String`. It starts by using the `print!` macro (not `println!`) to display the prompt. The difference matters here because `print!` doesn't add a newline, which means the cursor stays on the same line and waits for the user to type right after the question mark.

There's a subtlety with `print!` though. The text might not actually appear on screen immediately because terminal output can be buffered. Calling `io::stdout().flush()` forces the text out so the user sees the prompt before they're expected to type.

`String::new()` creates a new, empty, owned string. This is different from the string slices (`&str`) we've been using in earlier articles. An owned `String` can grow and change, which is what we need here because `read_line` is going to write the user's input into it. That's also why the variable is declared with `mut`, since `read_line` needs to modify the buffer.

The `.read_line(&mut buffer)` call reads everything the user types (up to when they hit Enter) and stores it in our buffer. This includes the newline character from pressing Enter, which we'll deal with later.

You'll notice `.unwrap()` appears twice. Both `flush()` and `read_line()` return a `Result` type, which is Rust's way of saying "this operation might fail." Calling `.unwrap()` tells Rust we're assuming success, and if something does go wrong, the program will crash. This is fine for a small learning exercise, but proper error handling is something I'll cover in a future post.

### Building the Greeting

```rust
fn build_greeting(name: String) -> String {
    let mut greeting = "Hello, ".to_owned();
    greeting.push_str(name.trim());
    greeting.push_str(", nice to meet you!");
    greeting
}
```

This function takes the name string as a parameter and returns a complete greeting. We start by creating a mutable owned string with the value "Hello, ". The `.to_owned()` call converts a string slice into an owned `String` so that we can append to it.

The `.push_str()` method appends a string slice onto the end of our growing string. Notice that when we push the name, we call `.trim()` on it first. This removes the trailing newline character that came along when the user pressed Enter. Without `.trim()`, our output would break across two lines in an awkward way.

After appending the final part of the greeting, the function returns the completed string.

### Printing the Result

```rust
fn print_greeting(greeting: String) {
    println!("{}", greeting);
}
```

The simplest of our three functions. It takes the greeting string and prints it to the console. You could argue this doesn't need to be its own function, and for a program this small that's fair. The habit of separating output from logic is worth practicing though, because in larger programs you might want to send that greeting somewhere other than the console.

### Putting it All Together

Here's the complete program:

```rust
use std::io::{self, Write};

fn get_name() -> String {
    print!("What is your name? ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn build_greeting(name: String) -> String {
    let mut greeting = "Hello, ".to_owned();
    greeting.push_str(name.trim());
    greeting.push_str(", nice to meet you!");
    greeting
}

fn print_greeting(greeting: String) {
    println!("{}", greeting);
}

fn main() {
    let name = get_name();
    let greeting = build_greeting(name);
    print_greeting(greeting);
}
```

The `main` function calls our three functions in sequence, using intermediate variables to make the data flow clear. Each variable holds the result of one step, which makes it easy to follow what's happening: get the name, build the greeting, print it.

You might be tempted to nest everything into a single line like `print_greeting(build_greeting(get_name()))`, and that does work. For a program this size it's fine either way, but intermediate variables tend to be easier to read and debug as things get more complex.

### Where to Go from Here

This program works, but it has some rough edges that are worth thinking about. What happens if the user just hits Enter without typing a name? The program happily prints "Hello, , nice to meet you!" which isn't great. What if reading from stdin fails? Right now the program just crashes due to our use of `.unwrap()`.

These are the kinds of problems that Rust's type system and error handling are designed to solve, and they're topics I'll be covering in upcoming articles. For now, try extending this program on your own. You could validate that the name isn't empty, or add a second question like "Where are you from?" and include that in the greeting. Small experiments like these are one of the best ways to solidify what you're learning.

### Resources

[Exercises for Programmers, by Brian P. Hogan](https://pragprog.com/titles/bhwb/exercises-for-programmers/)

[The Rust Standard Library: std::io](https://doc.rust-lang.org/std/io/index.html)