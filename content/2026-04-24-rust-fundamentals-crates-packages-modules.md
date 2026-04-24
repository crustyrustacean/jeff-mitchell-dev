+++
title = "Organizing Code: Crates, Packages, and Modules"
date = 2026-04-24
description = "How Rust programs are organized with crates, packages, and modules, including file structure, visibility, paths, and the use keyword."
categories = ["Fundamentals"]
tags = ["code-organization"]
draft = false
+++

As your Rust programs grow beyond a single file, you need a way to organize code into logical pieces. Rust provides three levels of organization: crates, packages, and modules. I'll be honest, I found this system confusing when I first encountered it. The terminology overlaps in ways that aren't immediately obvious, and the relationship between modules and files took me a while to internalize. But once it clicks, it's a clean and powerful system for keeping code manageable.

### Crates

A crate is the smallest unit of compilation in Rust. When the compiler builds your code, it thinks in terms of crates. Even a single file with a `main` function counts as a crate:

```rust
fn main() {
    println!("I am a crate.");
}
```

Every crate has a root, which is the file the compiler starts from when building. For a binary crate, the root is typically `src/main.rs`. For a library crate, it's `src/lib.rs`.

There are two kinds of crates:

A **binary crate** has a `main` function and compiles into an executable you can run. Every program you've written so far in this series has been a binary crate.

A **library crate** doesn't have a `main` function and can't run on its own. It provides functionality meant to be used by other crates. When you add a dependency like `rand` or `serde` to your project, you're pulling in a library crate.

A common pattern for organizing larger Rust programs is to split them into both: a binary crate in `src/main.rs` that handles startup and initialization, and a library crate in `src/lib.rs` that contains the core logic. The binary crate then depends on the library crate. For small programs this is overkill, but for anything substantial it keeps things modular and makes the library portion independently testable.

### Packages

A package is a bundle of one or more crates that are built together. A package is defined by the presence of a `Cargo.toml` file at the root of the project directory. Every time you run `cargo new my_project`, you create a package.

The rules for what a package can contain are straightforward: it can have at most one library crate, and it can have as many binary crates as you like. It must contain at least one crate of either kind.

By default, Cargo follows these conventions:

- `src/main.rs` is the crate root for a binary crate
- `src/lib.rs` is the crate root for a library crate
- Additional binary crates can live in `src/bin/`, one file per crate

The package name comes from the `[package]` section in `Cargo.toml`, and dependencies are listed under `[dependencies]`. You've been working with packages throughout this series, you just might not have thought of them in those terms.

### Modules

Modules are how you organize code within a crate. They let you group related functions, structs, enums, and other items together, and they control what's visible to code outside the module.

You define a module with the `mod` keyword:

```rust
mod audio {
    pub fn play(track: &str) {
        println!("Playing: {}", track);
    }

    fn load_file(path: &str) -> String {
        format!("Loading {}", path)
    }
}

fn main() {
    audio::play("Fuel");
    // audio::load_file("song.mp3"); // This won't compile, load_file is private
}
```

The `audio` module contains two functions. The `play` function is marked `pub`, making it accessible from outside the module. The `load_file` function has no `pub` keyword, so it's private and can only be called from within the `audio` module itself.

This is an important default in Rust: everything inside a module is private unless you explicitly make it public. This encourages you to think about what should be part of your public interface versus what's an internal implementation detail.

### Modules as Separate Files

Putting all your modules in one file works for small programs, but it quickly becomes unwieldy. Rust provides a clean mapping from modules to files and directories.

When the compiler sees `mod audio;` (with a semicolon instead of curly braces), it looks for the module's code in a separate file. Here's what a simple project looks like:

```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── audio.rs
    └── playlist.rs
```

And in `src/main.rs`:

```rust
mod audio;
mod playlist;

fn main() {
    audio::play("Fuel");
    playlist::show();
}
```

Each `mod` declaration tells the compiler to look for the corresponding file and treat its contents as a module. The code in `src/audio.rs` doesn't need its own `mod audio { }` wrapper because the file itself is the module.

### Module File Conventions: New vs Old

When a module has submodules, you need a directory. This is where Rust has two conventions, and it's worth understanding both because you'll encounter each in the wild.

**The modern convention** (introduced in the 2018 edition) names the parent module file to match the directory:

```
src/
├── main.rs
├── audio.rs
├── audio/
│   ├── player.rs
│   └── decoder.rs
└── playlist.rs
```

Here, `src/audio.rs` serves as the root of the `audio` module, and the `src/audio/` directory contains its submodules. Inside `src/audio.rs`:

```rust
pub mod player;
pub mod decoder;

pub fn play(track: &str) {
    println!("Playing: {}", track);
}
```

**The older convention** uses a `mod.rs` file inside the directory instead:

```
src/
├── main.rs
├── audio/
│   ├── mod.rs
│   ├── player.rs
│   └── decoder.rs
└── playlist.rs
```

In this layout, `src/audio/mod.rs` does the same job that `src/audio.rs` does in the modern style. The contents of the file would be identical.

The `mod.rs` approach still compiles and isn't technically deprecated, but it's considered the older style and has a practical downside: in any project of reasonable size, you end up with a dozen tabs in your editor all named `mod.rs`, and you can't tell which module you're looking at without checking the file path. The modern convention avoids this problem entirely because each file has a unique, descriptive name.

If you're starting a new project, use the modern convention. If you're reading someone else's code and see `mod.rs` files everywhere, now you know what's going on.

The `pub mod` declarations make the submodules visible to code outside the `audio` module.

### Paths and the use Keyword

To refer to an item inside a module, you use a path with the `::` operator:

```rust
audio::player::start();
```

Paths can be absolute (starting from the crate root) or relative (starting from the current module):

```rust
// Absolute path, starting from the crate root
crate::audio::player::start();

// Relative path, starting from the current module
audio::player::start();
```

The `self` keyword refers to the current module, and `super` refers to the parent module. `super` is particularly useful when a submodule needs to call something in its parent:

```rust
// Inside src/audio/player.rs
pub fn start() {
    let track = super::get_current_track();
    println!("Starting: {}", track);
}
```

Typing full paths gets tedious when you use something frequently. The `use` keyword brings an item into scope so you can refer to it by its short name:

```rust
use audio::player;

fn main() {
    player::start();
}
```

You can also bring specific items directly into scope:

```rust
use audio::player::start;

fn main() {
    start();
}
```

The convention in Rust is to `use` up to the parent module for functions (so you write `player::start()` and it's clear where `start` comes from) but to bring structs and enums all the way in (so you write `Album` rather than `types::Album`). This isn't a rule, just a community convention that helps with readability.

When you need multiple items from the same module, you can use a nested path:

```rust
use std::io::{self, Write};
```

This brings both `io` itself and the `Write` trait into scope in a single line. You've seen this exact line in the [greeting program](@/2026-04-15-rust-building-something-a-greeting-program.md) earlier in this series.

### Re-exporting with pub use

Sometimes you want to expose an item from a submodule as if it were part of the parent module. The `pub use` syntax re-exports an item, making it available through a shorter path:

```rust
// In src/audio/mod.rs
mod player;

pub use player::start;
```

Now code outside the `audio` module can call `audio::start()` instead of `audio::player::start()`. This is useful for creating a clean public API that hides your internal module structure from users of your code.

### Where to Go from Here

The crate, package, and module system can feel like a lot of concepts at once, but in daily use it boils down to a few habits: put related code in modules, mark public items with `pub`, use `mod` declarations to tell the compiler where to find things, and bring frequently used items into scope with `use`. As your projects grow, these tools keep your code navigable and your public interfaces intentional.

### Resources

[The Rust Programming Language, Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

[Rust By Example: Modules](https://doc.rust-lang.org/rust-by-example/mod.html)
