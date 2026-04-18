+++
title = "The Enum Type"
date = 2026-04-17
description = "How to define and use enums in Rust, including variants with data and an introduction to pattern matching with match."
categories = ["Fundamentals"]
tags = ["enum-type"]
draft = false
+++

If you're coming from a language like C or Java, you probably think of enums as a list of named constants. Rust's enums go much further than that. Each variant of a Rust enum can carry its own data, and different variants can carry different types and amounts of data. This makes enums one of the most expressive features in the language.

### What is an Enum?

An enum defines a type by listing its possible variants. Where a struct says "this thing has all of these fields," an enum says "this thing is one of these possibilities." That distinction is important. A struct models something that has multiple attributes at once. An enum models something that can be exactly one of several different things.

### Defining an Enum

At its simplest, an enum is a list of named variants:

```rust
enum Pepper {
    Bell,
    Banana,
    Habanero,
    Ghost,
}
```

This defines a custom type called `Pepper` with four possible variants. Like structs, enum names use PascalCase, and so do their variants.

### Variants with Data

What makes Rust enums powerful is that each variant can hold data, and different variants can hold different types:

```rust
enum Pepper {
    Bell,
    Banana(String),
    Habanero(i32),
    Ghost { name: String, shu: i32 },
}
```

Here, `Bell` carries no data. `Banana` carries a `String` (maybe a description). `Habanero` carries an `i32` for its Scoville Heat Unit rating. And `Ghost` carries a named struct-like set of fields. This flexibility is something most languages don't offer in their enum types, and it's one of the reasons Rust enums are so useful for modeling real-world data.

Creating instances of each variant looks like this:

```rust
let mild = Pepper::Bell;
let tangy = Pepper::Banana(String::from("sweet and mild"));
let spicy = Pepper::Habanero(100_000);
let extreme = Pepper::Ghost {
    name: String::from("Bhut Jolokia"),
    shu: 1_000_000,
};
```

Each variant is accessed through the enum name with `::` syntax. Notice that even though these are all `Pepper` values, they each carry different kinds of data.

### Accessing Enum Data with match

To work with the data inside an enum, you use the `match` expression. A `match` looks at which variant you have and lets you write different code for each possibility:

```rust
fn describe_pepper(pepper: Pepper) {
    match pepper {
        Pepper::Bell => {
            println!("A bell pepper, nice and sweet with 0 SHU.");
        }
        Pepper::Banana(description) => {
            println!("A banana pepper: {}.", description);
        }
        Pepper::Habanero(shu) => {
            println!("A habanero pepper with {} SHU. Watch out!", shu);
        }
        Pepper::Ghost { name, shu } => {
            println!("A {} with {} SHU. You've been warned.", name, shu);
        }
    }
}
```

Each branch of the `match` is called an arm. The arm specifies a pattern to match against and the code to run when that pattern matches. For variants that carry data, you bind the data to variables right in the pattern, like `shu` in the `Habanero` arm or `name` and `shu` in the `Ghost` arm.

An essential rule with `match` is that it must be exhaustive. Every possible variant must be accounted for. If you leave one out, the compiler will catch it and refuse to compile your program. This is a powerful safety feature because it means you can add a new variant to an enum and the compiler will immediately tell you every place in your code that needs to handle it.

When you have variants you don't care about, you can use the `_` wildcard as a catch-all:

```rust
match pepper {
    Pepper::Ghost { name, shu } => {
        println!("Danger! {} at {} SHU!", name, shu);
    }
    _ => {
        println!("This pepper is manageable.");
    }
}
```

The `_` arm matches anything that wasn't matched by the arms above it. It's useful when you only care about specific variants and want to handle everything else the same way.

### A Glimpse of What's Ahead

Enums are at the heart of two of Rust's most important types: `Option` and `Result`. The `Option` enum represents a value that might or might not exist, replacing the need for null values that cause so many bugs in other languages. The `Result` enum represents an operation that might succeed or fail, and it's the foundation of Rust's error handling. Both are topics for future articles, but knowing that they're built on the same enum concept you've just learned should give you a sense of how central enums are to the language.

### Where to Go from Here

Enums and structs together form the backbone of Rust's type system. Where structs let you model things that have multiple attributes, enums let you model things that can be one of several possibilities. Combined with `match` for exhaustive pattern matching, enums give you a way to write code that the compiler can verify handles every case. Practice defining a few enums of your own, especially ones where different variants carry different data, as that's where the real power lies.

### Resources

[The Rust Programming Language, Chapter 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

[The Rust Programming Language, Chapter 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)