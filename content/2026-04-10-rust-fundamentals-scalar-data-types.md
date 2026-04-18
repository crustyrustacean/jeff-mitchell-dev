+++
title = "Scalar Data Types"
date = 2026-04-10
description = "Rust's scalar types: integers, floating-point numbers, booleans, and characters."
categories = ["Fundamentals"]
tags = ["data-types"]
draft = false
+++

Data types are at the core of getting things done in Rust. They tell the compiler what kind of value a variable holds, which in turn determines how much memory to allocate and what operations are valid. Rust has two broad categories of built-in data types: scalar and compound. This article covers scalar types, which represent a single value. Compound types, which group multiple values together, are up next.

Rust has four scalar types: integers, floating-point numbers, booleans, and characters. If you've worked with another compiled language before, these will feel familiar. I originally learned these concepts in Turbo Pascal many years ago, and the ideas carry over almost directly.

### Integers

An integer is a whole number with no fractional component. Rust provides both signed and unsigned integer types in several sizes:

| Signed | Unsigned | Size |
|--------|----------|------|
| i8     | u8       | 8-bit |
| i16    | u16      | 16-bit |
| i32    | u32      | 32-bit |
| i64    | u64      | 64-bit |
| i128   | u128     | 128-bit |
| isize  | usize    | Platform dependent |

Signed integers can hold negative values. Unsigned integers can only hold zero or positive values, but their maximum positive value is higher for the same number of bits. The `isize` and `usize` types match the pointer size of the computer running the program, which is 64 bits on most modern systems.

Rust defaults to `i32` when you don't specify a type, which is a good general-purpose choice:

```rust
let life = 42;
```

This creates a variable named `life` and binds the integer value 42 to it. The compiler infers the type as `i32`.

Choosing the right integer size is something you'll develop a feel for over time. For most situations, the default `i32` works well. Use `u8` when you're working with bytes, `usize` for indexing into collections, and the larger types when you know you'll be dealing with very large numbers.

### Floating-Point Numbers

A floating-point number has a decimal component. Rust provides two sizes: `f32` (32-bit) and `f64` (64-bit). The default is `f64`, which gives you more precision at roughly the same speed on modern hardware. All floating-point types are signed, there is no unsigned option like with integers.

```rust
let pi = 3.14;
```

This creates a variable named `pi` and binds the floating-point value 3.14 to it. The compiler infers the type as `f64`.

### Numeric Operations

Rust supports all the standard mathematical operations you'd expect: addition, subtraction, multiplication, division, and remainder (sometimes called modulo). Here's a small example that brings together variables and a numeric operation:

```rust
fn main() {
    let x = 5;
    let y = 6;
    let sum = x + y;
    println!("The sum of {} and {} is: {}", x, y, sum);
}
```

We create two integer variables, add them together, bind the result to a third variable, and print everything out. The other operators work the same way: `-` for subtraction, `*` for multiplication, `/` for division, and `%` for remainder.

One thing to be aware of with division: dividing two integers performs integer division, meaning the result is truncated rather than rounded. So `7 / 2` gives you `3`, not `3.5`. If you need the decimal result, at least one of the values needs to be a floating-point number.

### Booleans

A boolean is either `true` or `false`. It occupies a single byte of memory and is used extensively in conditional logic, which I'll cover in a later article.

```rust
let is_active = true;
let is_complete: bool = false;
```

The type annotation on the second line is optional since Rust can infer the type, but I've included it to show that the type is written as `bool`.

### Characters

The character type represents a single Unicode character and is declared with single quotes:

```rust
let letter = 'Z';
let emoji = '🦀';
```

The `char` type in Rust is four bytes wide because it represents a Unicode scalar value, which means it can hold far more than just ASCII letters. Emoji, Chinese characters, accented letters, and many other symbols are all valid `char` values.

It's important to distinguish characters from strings. A `char` is always a single character wrapped in single quotes. Strings, which use double quotes and can contain multiple characters, are a separate and more complex type that I'll cover in a future article. The Rust Book devotes significant space to strings for good reason.

### Where to Go from Here

These four scalar types are the simplest building blocks in Rust, but they show up in everything you write. The next article covers compound data types, tuples and arrays, which let you start grouping these simple values together into more useful structures.

### Resources

[The Rust Programming Language, Chapter 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html)