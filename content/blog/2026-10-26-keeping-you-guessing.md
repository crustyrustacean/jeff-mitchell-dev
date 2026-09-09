+++
title = "Building Something: Keeping You Guessing"
date = 2026-10-26
description = "A variation on the guessing game program from Chapter 2 of the Rust Book"
categories = ["How-To"]
tags = ["rust", "game", "guessing game"]
draft = true
aliases = ["/2026-10-26-keeping-you-guessing"]
+++

If you've been following my writings at all, you'll notice a lot of what I think of as beginner oriented content about the Rust programming language. It's my attempt to put my understanding of the topic at hand into words and ideas that I can go back to later. It may not always be complete or provide the best picture, but it supplements learning I do elsewhere.

<!-- more -->

They say after all the best way to understand something is to try to explain it to someone else. I actually wrote all of what I've posted to date about two years ago. I'm bringing it over to a platform where potentially I can reach more people. Let's face it, in 2022 the free internet was dead...in 2025 it's even worse. If you're not on some kind of curated platform you're basically knowhere.

In this vein, I'm doubling down a bit on basics. I started reading [The Rust Book](https://doc.rust-lang.org/book/title-page.html), probably two years ago, but never finished. I find it a little...obtuse. They don't write with very many concrete examples that are representitive of what you'd see in real life, as such I have a hard time learning from it. I'm the type of learner that needs to do. Abstract concepts don't mean much to me until I actively try and make something real.

The unfortunate side effect of this is a tendency to gloss over the basics, as I don't find them interesting (understandable?) on their own. I seek out some thing or some project, usually way too big for me, and try to run before I walk. Over time, and with lots of repetition, things sink in.

I kind of consider it a bit of a learning disability, and I know I'm probably too harsh on myself. I wish amongst all the wishes that I could learn to slow down and take joy in understanding all the basic bits.

I don't see myself changing anytime soon. It's best to just go with it and not fight.

Anyhow, enough navel gazing. I said a moment ago that I'd returned in 2025 to The Rust Book. Yesterday I finished Chapter 2, which is where the authors walk you through creation of a Guessing Game.

Today I challenged myself to do it from memory.

Did it, mostly, with some fumbling. Here is the super overengineered version:

```Rust
// src/main.rs

// Guessing Game project from Chapter 2 of "The Rust Book"

// dependencies
use color_eyre::Result;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

// function which writes an output message
fn write_msg(msg: &str) -> Result<()> {
    stdout().write_all(msg.as_bytes())?;
    Ok(())
}

// main function
fn main() -> Result<()> {
    // initialize color_eyre
    color_eyre::install()?;

    // initialize with a random number that the user will guess
    let mut rng = thread_rng();
    let number = rng.gen_range(1..=100);

    // game event loop
    loop {
        // user input
        let msg = "Please enter a number between 1 and 100...\n";
        write_msg(msg)?;
        let mut guess = String::new();
        stdin().read_line(&mut guess)?;
        let msg = format!("You guessed: {}", guess);
        write_msg(msg.as_str())?;

        // compare the user's input with the random numb;
        let guess: i32 = guess.trim().parse()?;

        match guess.cmp(&number) {
            Ordering::Less => {
                let msg = "Too small!\n";
                write_msg(msg)?;
            }
            Ordering::Greater => {
                let msg = "Too big!\n";
                write_msg(msg)?;
            }
            Ordering::Equal => {
                let msg = "You got it! You win\n";
                write_msg(msg)?;
                break;
            }
        }
    }

    Ok(())
}
```

Things you'll notice which are way beyond what's introduced in Chapter 2:

- use of the [color_eyre](https://crates.io/crates/color-eyre) crate for error handling. This crate enables use of the `?` operator which simplifies error handling for you and reduces the amount of boilerplate you're resposible for.
- no macros...for some reason I'm on a no macros kick, as I want to understand what's going on underneath. You'll see a `write_msg()` function, which accepts a string slice and calls the `stdout()` function to create a handle to stdout, giving us something that implements the Write trait so that we can write a message to the console.

Other than that, it's pretty much the same as from Chapter 2. I'd thought briefly about turning this thing into some kind of API, but in the end just kept it to the some direction as in the book.

This is a very trivial program, but it highlights a lot of important concepts in Rust and is a great jumping off point for your own exploration. I share here because I want to put my work out there, perhaps get some constructive feedback, and perhaps help others in their own Rust journeying.

Good night and thanks for reading!

References:

- [The Rust Book: Chapter 2, Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
