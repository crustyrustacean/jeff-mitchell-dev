+++
title = "Vectors"
date = 2026-08-28
description = "How to define, instantiate and use the vector collection type in Rust."
categories = ["fundamentals"]
tags = ["collection-types", "vector"]
draft = false
aliases = ["/2026-08-28-rust-fundamentals-collection-types-vectors"]
+++

Today I resume my journey through the Rust Book, and this time the spotlight is on the vector type.

The Rust Book introduces vectors as a "collection". A vector is similar to an array type, but with one critical difference: a vector can grow and shrink in size. A vector stores a list of items in a single data structure, and all the individual values sit next to each other in memory. There is one catch — a vector can only store data of the same type.

Vectors are an extremely versatile and useful way of storing data. I think you'll find yourself reaching for them quite a bit in your own Rust adventures.

## Creation

### The Vec::new() Function

A fresh, empty vector can be created like so:

```rust
let origin_coordinates: Vec<i32> = Vec::new();
```

Since we haven't initialized this coordinates vector with any values, we have to tell the compiler what we want — otherwise it has no way of knowing. The vector type provided by the Rust standard library is implemented using generics and can hold any type. Here we've said that our coordinates vector is going to contain i32 elements.

### The vec! Macro

More often than not, we want to initialize a vector with some values already in place. Rust gives us a macro for exactly that: the vec! macro creates a vector with whatever values we choose:

```rust
let origin_coordinates = vec![0, 0, 0];
```

Because we've provided initial values, the compiler can infer what we want, so we don't need a type annotation like in the first example.

### Modifying a Vector

Hopefully you recall that all variables in Rust are immutable when declared — they can't be changed. If we know we need to change the values in our vector, we need the mut keyword:

```rust
let mut coordinates = vec![1, 3, 5];
```

Then, we can add values to this vector using the push method:

```rust
coordinates.push(10);
coordinates.push(15);
coordinates.push(20);
```

Let's make a complete program to see what we get:

```rust
fn main() {
    let mut coordinates = vec![1, 3, 5];
    coordinates.push(10);
    coordinates.push(15);
    coordinates.push(20);

    for coordinate in coordinates {
        println!("{}", coordinate);
    }
}
```

Here we initialize our vector with some default i32 values, then push three more values onto the end. Finally, we use a for loop to print the values out to the console. Note that we don't need any type annotations — the Rust compiler can infer everything from the information we've provided.

```text
   Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/playground`
1
3
5
10
15
20
```

### Reading the Elements of a Vector

So, we can add elements to our vector — how do we read them back out? Rust gives us a couple of ways, depending on what we want our program to do.

_Panic Attack_

The first way of reading an element is simple indexing:

```rust
let coordinates = vec![25, 24, 23];

let z: &i32 = &coordinates[2];
```

In one of the greatest quirks of computer science — one which, to this day, trips up just about everyone at one time or another — vectors are indexed starting at zero. In the previous example, the indices of our coordinates vector are 0, 1, and 2. To get the third element, let's call it the 'z' coordinate, we use `&` and `[]` along with the index 2, giving us a reference to the element 23.

This is all fine and happy as long as the element at the index we request actually exists. What if it doesn't? The program panics and immediately terminates. We might want that behaviour, so it remains a legitimate option.

_A More Elegant Way_

There is another, more elegant way to handle the possibility of a vector element not existing:

```rust
let coordinates = vec![25, 24, 23];

let z: Option<&i32> = coordinates.get(2);
match z {
    Some(z) => println!("The z coordinate is {z}"),
    None => println!("Oops, no third element exists in this vector!"),
}
```

The get method, when passed an index outside the vector's range, leverages the Option type and returns None without any panic. You can then use the match statement to gracefully handle the possibilities. This approach results in more user-friendliness than a panic and crash, because you can craft error messages that explain exactly what happened.

### Enums to Store Multiple Types

Remember earlier when I said that vectors can only hold data of the same type? Well, I lied a teeny tiny bit. We can leverage Rust's enum type to get around this limitation. There are times when we may want to have a list of items that have different types:

```rust
enum SportsTeam {
    Name(String),
    Conference(String),
    Standing(i32),
}

let teams = vec![
    SportsTeam::Name(String::from("Seattle Seahawks")),
    SportsTeam::Conference(String::from("NFC West")),
    SportsTeam::Standing(2),
];
```

This is not the greatest example, because the variations represented by the SportsTeam enum are not that dramatically different from one another. However, it illustrates that we can create a vector to hold some information about our sports team, and because the underlying type of each vector element is the enum, every element is still of the same type. Everyone's happy.

A vector is stored on the heap, and the Rust compiler must know exactly how much memory to set aside for each element at compile time. This is where the enum earns its keep — it hands the compiler the complete set of possibilities up front, and the compiler uses our match expressions to make sure every variation gets handled. If the data in your program is such that you can't know the exhaustive set of types at compile time, then this enum technique won't work. The solution in that case is a trait object, which I'll cover in a future article.

## Conclusion

This article has been my take on vectors. I've gone over the basics, but be sure to check out the Rust Standard Library documentation for the `std::vec` module — there is plenty more you can do with this powerful and flexible data type.

Thanks for reading!

## References

- [Rust Standard Library, Module std::vec](https://doc.rust-lang.org/std/vec/index.html)
- [The Rust Programming Language, Chapter 8.1 — Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)