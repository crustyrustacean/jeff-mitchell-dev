+++
title = "Error Handling"
date = 2026-09-24
description = "An introduction to error handling in Rust"
categories = ["fundamentals"]
tags = ["error-handling"]
draft = false
aliases = ["/2026-09-24-rust-fundamentals-error-handling"]
+++

Today, we look at error handling in Rust. The sophistication of error handling is one of the language's biggest advantages. It's easy to gracefully handle whatever errors may arise. It can also be a source of frustration, because the compiler lets you get away with nearly nothing, expecting all paths, including error paths, to be handled. Compared to a dynamically typed language, this can feel a bit straitjacket-ish. The benefit is fewer surprises when things go wrong. By thinking just that little bit harder up front, and leveraging the tools Rust provides, the output is something you'll have to worry about less.

Let's take a look more deeply.

## To Recover or Not to Recover...That is the Question

It's a fact of life that operations can fail. When things go wrong, what can we do? Errors come in two flavours when programming in Rust, unrecoverable and recoverable.

_Unrecoverable Errors_

Sometimes, things go so wrong that there's just nothing that can be done. In these instances, we halt execution of our program and provide some final message or output that will allow you the programmer to (hopefully) figure out what went wrong. The built-in way of doing this in Rust is to use the `panic!` macro, which halts execution, cleans up the stack and dumps you out to the terminal with a terse error message. That terse message doesn't have to be the end of the story: set the `RUST_BACKTRACE=1` environment variable when you run your program and you'll get a full backtrace showing exactly where things went wrong.

Rust also gives us an escape hatch when we don't want to deal with an error at all, via the `.unwrap()` method. If the result is the `Ok` variant, we get the value and carry on. If it's the `Err` variant, the program panics and terminates. In effect, `unwrap()` takes what could have been a recoverable error and makes it unrecoverable. There are times when that's appropriate, mainly when a) we're confident the failure path won't be hit or b) as a starting point. It can frequently be a stepping stone to unwrap on something, then later handling the error more gracefully.

A variation on `.unwrap()` is `.expect()` which panics, but allows you to attach context which is output when the panic happens.

All of the above have one thing in common, program execution is over when they happen.

_Recoverable Errors_

There are many instances where, yes, something can go wrong, but whatever it was could be recovered from such that we pick ourselves up, dust off, and carry on. We can communicate some bit of information to the user such that they can fix a typo in their input or make an alternate choice that lets the program continue.

Fortunately, Rust provides a nice way for us to, once again, leverage the type system and recover from errors if we choose.

So how do you choose between panicking and recovering? A few rules of thumb that work for me:

- If the code is a prototype, example or test, `unwrap()` and `expect()` are fine. That code is throwaway.
- If a failure means your own logic is broken — a bug, a violated assumption — let it panic. There's nothing sensible to recover.
- If the caller could plausibly do something about the failure — retry it, ask the user again, pick a different file — return a `Result` and let them decide.

## Grace, Too...The Result<T, E> Type

I wrote about the [Result](@/blog/2026-04-23-rust-fundamentals-result-type.md) but let's have another go with a small program to read text from a file and print it back out. Here's the code:

```Rust
// src/main.rs

// dependencies
use std::fs::File;
use std::io::{Read, Result};

// main function, returns a std::io::Result<()> type in case of error
fn main() -> Result<()> {

    // open the file called test.txt, located in the project root
    // the File::open method returns a result, which could be either the file handle, or an error
    // we use match to determine the courses of action
    let mut input_file = match File::open("test.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to read the input file: {}", e);
            return Err(e);
        }
    };

    // create an empty, mutable string variable to store the file contents in
    let mut data = String::new();

    // the read_to_string() method could fail, so again we match
    // print out the data contained in the file to stdout
    // if there was some error in the read operation, we print it out to stderr
    match input_file.read_to_string(&mut data) {
        Ok(_) => println!("{}", data),
        Err(e) => {
            eprintln!("Unable to read the file contents as a string: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
```

## The ? Operator

The above is a little verbose. Let's do better:

```Rust
// src/main.rs

// dependencies
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut input_file = File::open("test.txt")?;
    let mut data = String::new();

    input_file.read_to_string(&mut data)?;

    println!("{}", data);

    Ok(())
}
```

Here we use the Rust `?` operator, which is some syntactic sugar which does roughly the equivalent of the code in the `match {}` blocks shown above. This special operator:

- evaluates the `Result<T, E>` type that comes back from the IO related operations.
- if it's the error path, we do an early return, propagating the error value back to the caller.
- if it's the happy path, execution continues as we expect, the file is read into the variable `input_file` and later the data contained within the file, is output.

One thing to notice: we didn't write a single `eprintln!`, yet running this program with a missing `test.txt` still prints an error and exits with a non-zero code. That's a little gift from Rust: when `main` returns a `Result` and it turns out to be the `Err` variant, the runtime prints the error for us and terminates with a failure code.

Use of the `?` operator makes your Rust programs a whole lot more concise. In your mind, when you see it, you can think "possible failure here, we either succeed and carry on, or we error out".

And since we're on the subject of concise: the standard library quietly bundles our open-then-read ceremony into a single call, so the whole program is really just:

```Rust
fn main() -> std::io::Result<()> {
    let data = std::fs::read_to_string("test.txt")?;

    println!("{}", data);

    Ok(())
}
```

Knowing the standard library beats writing the ceremony yourself.

When the `?` is used in a function body, the function signature must include returning a Result type. In this instance we say we are "propagating" the error back to the caller, which could potentially be the main function. It will be up to the main function to handle the returned result type in some final, possibly different, way.

When the Result type is used, program termination is more under the programmers control. The type can be used to set pathways that allow recovery and continued program operation. The example program above does not handle errors any more gracefully per se, but using the Result type, together with embellished error information (the actual error is included with the overall message) the programmer gains more control over the messaging.

## Closing Thoughts

The Rust Book is a little...obtuse, lacking real-world examples on the matter of error handling. Here I've attempted to break it down in the way I've started to understand and use it. It can be one of the more difficult language aspects to grasp, but once you've got it, it's hard to go without it.

One caveat: once your programs grow and errors from different crates start piling up, rolling your own error handling for all of them gets tedious. That's when the community crates `anyhow` (for applications) and `thiserror` (for libraries) earn their keep. I name-dropped these back in the Result type article — they're a post for another day.

## References

- [The Rust Programming Language: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [The simplest guide to error handling in Rust](https://kerkour.com/rust-error-handling)
