+++
title = "Setting Up Rust and Your First Program"
date = 2026-04-06
description = "Install Rust, create your first project with Cargo, and run a hello world program step by step."
categories = ["Fundamentals"]
tags = ["getting-started", "cargo", "toolchain"]
draft = false
aliases = ["/2026-04-06-rust-setting-up-your-first-rust-program"]
+++

Late in 2020, I decided I needed to properly learn to code. I explored several options before settling on Rust at the beginning of 2022, and I haven't looked back. Rust is performant, has excellent tooling, and its compiler gives you some of the most helpful error messages of any language. It runs on Windows, Linux, and macOS, and can be used to build just about any kind of software you can think of.

The learning curve is steep, no question about it. But the payoff is worth the effort. This article will help you get off the ground by installing Rust, creating your first project, and running a simple program.

### Before You Start

This article assumes you have a solid foundation of general computer skills and are comfortable using the command line on your platform of choice. Experience with another programming language will help, though Rust can be your first language if you're prepared for a steeper climb.

### Compiled vs Interpreted

Rust is a compiled language, which means the programs you write are translated into machine code before they can run. This is different from interpreted languages like Python or JavaScript, where your code is executed more or less directly. Compilation adds a step to your workflow, but it's also where Rust catches many bugs before your program ever runs. If you'd like a deeper explanation of the difference, [this article on freeCodeCamp](https://www.freecodecamp.org/news/compiled-versus-interpreted-languages/) is a great starting point.

### Cargo

`Cargo` is Rust's build tool and package manager. It handles creating projects, managing dependencies (external libraries, called "crates" in Rust), compiling your code, running tests, and more. You technically can write Rust without Cargo by compiling individual files with the `rustc` compiler, but there's no good reason to. Cargo makes everything easier and I'll use it exclusively in everything I write going forward.

### Installation

If you haven't already installed Rust, head over to [rustup.rs](https://rustup.rs/) and follow the instructions for your platform. The installer will set up `rustc` (the compiler), `cargo`, and the Rust standard library. Once installation is complete, you can verify everything is working by opening a terminal and running:

```bash
rustc --version
cargo --version
```

If both commands print a version number, you're good to go.

### Your First Program

It's a cliche, but let's start with a program that says hello. Open your terminal, navigate to a directory where you'd like to keep your projects, and run:

```bash
cargo new hello_name
```

This creates a new Rust project in a directory called `hello_name`. Cargo sets up the project structure for you, including a `src` directory with a `main.rs` file and a `Cargo.toml` file that describes your project and its dependencies.

Rust projects come in two flavors: binaries and libraries. A binary is a standalone program that can be run on its own. A library is meant to be used as a dependency by other Rust code. Cargo creates a binary project by default, which is what we want.

Change into the project directory:

```bash
cd hello_name
```

Open the project in your editor of choice, then look at `src/main.rs`. Cargo has already generated a working program for you:

```rust
fn main() {
    println!("Hello, world!");
}
```

Let's modify it to greet you by name:

```rust
fn main() {
    let name = "Jeff";
    println!("Hello, {}!", name);
}
```

Change "Jeff" to your own name, of course. There are a few things worth noticing in this tiny program:

The `fn main()` line declares the main function, which is the entry point of every Rust binary. When you run the program, this is where execution begins.

The `let name = "Jeff"` line creates a variable called `name` and binds the string "Jeff" to it. The `let` keyword is how you create variables in Rust, and we'll explore this in much more detail in a later article.

The `println!` line is a macro (the `!` at the end is what distinguishes a macro from a regular function) that prints text to the console. The `{}` acts as a placeholder that gets replaced with the value of `name`.

Every statement ends with a semicolon, and the body of the function is wrapped in curly braces. These curly braces define a scope, which becomes very important later when we get into ownership, one of Rust's core concepts.

Now let's run it. Back in your terminal:

```bash
cargo run
```

The first time you run this, Cargo will compile your program and then execute it. You should see:

```bash
Hello, Jeff!
```

Congratulations. No matter how simple, you've written and run a Rust program. There are many concepts ahead, and the learning curve is real, but this first step is behind you now.

### Resources

[The Rust Programming Language](https://doc.rust-lang.org/book/)

[The Cargo Book](https://doc.rust-lang.org/cargo/index.html)

[Rustup](https://rustup.rs/)
