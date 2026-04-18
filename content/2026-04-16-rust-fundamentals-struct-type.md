+++
title = "The Struct Type"
date = 2026-04-16
description = "How to define, instantiate, and use structs in Rust, including update syntax, field init shorthand, and tuple structs."
categories = ["Fundamentals"]
tags = ["struct-type"]
draft = false
+++

Up to this point, we've worked with scalar types like integers and booleans, and compound types like tuples and arrays. These are useful, but they don't give you a great way to represent something with multiple related pieces of data, like a music album that has an artist, a name, a genre, and a release date. That's where structs come in.

### What is a Struct?

A struct is a custom data type that lets you group related values together under named fields. If you've used tuples, you already understand the idea of grouping values, but structs improve on tuples in an important way: each field has a name, so you don't have to remember that index 0 is the artist and index 2 is the genre. The field names make your intent clear.

### Defining a Struct

You define a struct with the `struct` keyword followed by a name in PascalCase, then list the fields inside curly braces:

```rust
struct Album {
    id: u32,
    genre: String,
    artist: String,
    name: String,
    release_date: String,
}
```

This is a definition, not an instance. Think of it as a blueprint that describes what data an `Album` contains. No actual data exists yet.

Each field has a name and a type, separated by a colon. The fields here are mostly `String` types rather than string slices (`&str`). This is deliberate because it means each instance of `Album` owns its data. Ownership is a topic for a future article, but for now just know that using `String` is the simpler choice when you're starting out with structs.

### Creating an Instance

To actually use the struct, you create an instance by providing values for each field:

```rust
fn main() {
    let album1 = Album {
        id: 1,
        genre: String::from("Heavy Metal"),
        artist: String::from("Iron Maiden"),
        name: String::from("Senjutsu"),
        release_date: String::from("September 3, 2021"),
    };
}
```

The variable `album1` is now an instance of our `Album` struct, with concrete values in every field. The fields can be listed in any order when creating an instance since they're identified by name, not position.

### Accessing Fields

You access individual fields using dot notation:

```rust
println!("The name of the album is: {}", album1.name);
println!("Released on: {}", album1.release_date);
```

This is one of the main advantages over tuples. Reading `album1.name` is immediately clear, whereas `album1.3` would force you to go back and count fields to figure out what you're looking at.

### Mutability

As declared in the example above, none of the fields can be changed after creation. If you need to modify fields, the entire instance must be declared as mutable with `mut`. Rust doesn't allow marking individual fields as mutable, it's all or nothing.

```rust
fn main() {
    let mut album1 = Album {
        id: 1,
        genre: String::from("Heavy Metal"),
        artist: String::from("Iron Maiden"),
        name: String::from("Senjutsu"),
        release_date: String::from("September 3, 2021"),
    };

    album1.name = String::from("The Book of Souls");
    album1.release_date = String::from("September 4, 2015");
}
```

After declaring `album1` with `mut`, we can reassign any of its fields using the same dot notation we use to read them.

### Field Init Shorthand

When you're building a struct instance inside a function, you'll often find that the function parameters have the same names as the struct fields. Rust provides a shorthand for this situation so you don't have to write `id: id` or `genre: genre`:

```rust
fn build_album(id: u32, genre: String, artist: String) -> Album {
    Album {
        id,
        genre,
        artist,
        name: String::from("Unknown"),
        release_date: String::from("Unknown"),
    }
}
```

The `id`, `genre`, and `artist` fields are populated directly from the function parameters because the names match. The remaining fields are set manually. This is a small convenience, but it makes builder-style functions much cleaner to read, especially when a struct has many fields.

### Struct Update Syntax

Rust provides a convenient shorthand for creating a new instance from an existing one, where most of the values stay the same. The `..` syntax copies the remaining fields from another instance:

```rust
let album2 = Album {
    id: 2,
    name: String::from("The Number of the Beast"),
    release_date: String::from("March 22, 1982"),
    ..album1
};
```

This creates a new `Album` that gets its own `id`, `name`, and `release_date`, but takes `genre` and `artist` from `album1`. This is especially useful when you have structs with many fields and only need to change a few. One thing to be aware of is that this moves `String` values out of `album1`, so some fields of `album1` may no longer be usable afterward.

### Tuple Structs

Not every struct needs named fields. Rust supports tuple structs, which have a name but use positional fields like a tuple. These are useful when you want to give a meaningful type name to a small group of values without the overhead of naming each field.

A good example is representing a position in three-dimensional space:

```rust
struct Coordinates(i32, i32, i32);

fn main() {
    let position = Coordinates(0, 1, 5);

    let x = position.0;
    let y = position.1;
    let z = position.2;
}
```

Fields are accessed with dot notation and an index, just like a regular tuple. The difference is that `Coordinates` is now its own distinct type. If you also had a `Color(i32, i32, i32)` tuple struct, Rust would treat them as completely different types even though they contain the same kinds of values. You couldn't accidentally pass a `Color` where a `Coordinates` is expected.

Tuple structs work best when you have two or three fields whose meaning is obvious from context. Beyond that, named fields make the code much easier to follow.

### Unit Structs

Rust also allows structs with no fields at all, called unit structs:

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

This might look pointless at first, but unit structs become useful when you need a type to implement a trait without actually storing any data. Traits are a way of defining shared behavior in Rust, and I'll cover them in a future article. For now, just know that unit structs exist and that they serve a purpose you'll appreciate once traits are on the table.

### Where to Go from Here

Structs are one of the most commonly used types in Rust and you'll reach for them constantly. This article has covered defining and instantiating structs, the field init shorthand, update syntax, tuple structs, and unit structs. There's still more to explore, including methods and associated functions with `impl` blocks and deriving common traits like `Debug`, which will be the topic of a future article.

### Resources

[The Rust Programming Language, Chapter 5.1](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)