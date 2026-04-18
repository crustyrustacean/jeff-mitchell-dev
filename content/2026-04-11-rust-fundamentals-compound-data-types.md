+++
title = "Compound Data Types"
date = 2026-04-11
description = "How to create and access tuples and arrays in Rust, with indexing, destructuring, and when to use each."
categories = ["Fundamentals"]
tags = ["data-types"]
draft = false
+++

In the previous article on [Scalar Data Types](@/2026-04-10-rust-fundamentals-scalar-data-types.md), we covered values that represent a single thing: an integer, a float, a boolean, or a character. Compound data types let you group multiple values together into one type. Rust has two built-in compound types: tuples and arrays.

### Tuples

A tuple groups together values that can each be a different type. Once created, a tuple has a fixed length and cannot grow or shrink.

```rust
let survey_result: (&str, u8, bool) = ("John", 36, true);
```

This creates a tuple containing a string slice, an unsigned 8-bit integer, and a boolean, bound to the variable `survey_result`. You could imagine this representing a response from a survey form. The type annotations after the variable name are optional here since Rust can infer the types from the values, but they make the intent clearer when reading the code.

There are two ways to access the values inside a tuple.

**Dot Notation**

Each element in a tuple can be accessed by its position using a dot followed by the index, starting from 0:

```rust
fn main() {
    let survey_result: (&str, u8, bool) = ("John", 36, true);
    let name = survey_result.0;
    let age = survey_result.1;
    let answer = survey_result.2;
}
```

This works fine for small tuples, but tracking indices gets tedious as the number of elements grows.

**Destructuring**

A more readable approach is to unpack the tuple into named variables all at once:

```rust
fn main() {
    let survey_result: (&str, u8, bool) = ("John", 36, true);
    let (name, age, answer) = survey_result;
    println!("{}, aged {}, answered {} on the survey form.", name, age, answer);
}
```

Instead of three separate `let` statements with dot notation, we bind all three values in a single line. The variable names make it clear what each value represents, which is a big improvement over remembering that index 1 means age.

Tuples are handy for small, lightweight groupings where defining a full type would be overkill. They might seem like a minor feature at first, but in practice they show up constantly in Rust. Functions returning multiple values, `.enumerate()` giving you `(index, value)` pairs when iterating, destructuring in pattern matching, key-value pairs from a HashMap. Once you start writing real code, you'll find tuples are one of those quiet workhorses that are everywhere.

When your data structure has more than a few fields, or when you need to give meaningful names to each field, a struct is the better choice. I'll cover structs in a future article.

### Arrays

An array holds multiple values of the same type. Like tuples, arrays have a fixed length that is set when they're created.

A simple array of the days of the week looks like this:

```rust
let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
```

You can add an explicit type annotation that specifies both the element type and the length:

```rust
let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
```

The `[i32; 6]` syntax tells the compiler this is an array of six 32-bit integers. The compiler already knows this from the values, but being explicit can help with readability.

There's also a shorthand for creating an array where every element has the same value:

```rust
let zeroes = [0; 5];
```

This creates an array of five elements, all set to 0. The value comes first, then a semicolon, then the count.

**Accessing Elements**

Array elements are accessed using square bracket notation with a zero-based index:

```rust
fn main() {
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let midweek = weekdays[2];
    let end_of_week = weekdays[4];
    println!("Midweek is {} and the week ends on {}", midweek, end_of_week);
}
```

Index 0 is Monday, index 1 is Tuesday, and so on. I'll be honest, zero-based indexing still trips me up from time to time. It's one of those things you just have to get used to.

**Bounds Checking**

One of Rust's safety features shows up here. If you try to access an index that's beyond the end of an array, Rust will catch it. Unlike some languages that will happily let you read garbage data from some unrelated piece of memory, Rust checks that the index is within bounds and will panic (crash) at runtime if it's not. This might sound harsh, but it prevents an entire category of bugs that can be very difficult to track down in other languages. You'll see this idea of preventing unsafe memory access come up again and again as you go deeper into Rust.

### When to Use Which

Both tuples and arrays are stored on the stack, which makes them cheap to create and fast to access since the compiler knows their exact size at compile time.

The main distinction is straightforward. Use a tuple when you need to group a few values of different types together, like a function returning both a status code and a message. Use an array when you have a fixed collection of values that are all the same type, like the days of the week or a set of configuration flags.

For situations where you need a collection that can grow or shrink at runtime, Rust provides the `Vec` (vector) type, which I'll cover in a future article. Vectors are what you'll reach for most often in practice, but understanding arrays first gives you the foundation to appreciate what vectors add on top.

### Where to Go from Here

With scalar types, tuples, and arrays covered, we now have the building blocks for holding data in our programs. The next article moves on to functions, which give us the ability to organize our code into reusable, named pieces.

### Resources

[The Rust Programming Language, Chapter 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html)