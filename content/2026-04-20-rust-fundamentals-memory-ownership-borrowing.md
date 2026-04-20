+++
title = "Ownership and Borrowing"
date = 2026-04-20
description = "How Rust manages memory through ownership, moving, copying, cloning, borrowing, and references, with the rules that make your programs safe without a garbage collector."
categories = ["Fundamentals"]
tags = ["ownership"]
draft = false
+++

> "Fortune, fame, mirror vain, gone insane...but the memory remains!" - Metallica, The Memory Remains

I'll be honest, I procrastinated on this article for a long time. Ownership and borrowing are the concepts that make Rust different from every other language I've used, and for a while I wasn't confident I understood them well enough to write about them. I'm going for it anyway, because if you never start, you never finish.

These concepts are probably the most challenging part of learning Rust. They're also the most rewarding once they click, because they're the reason Rust programs are memory-safe without paying the cost of a garbage collector. Everything you've learned so far in this series has been building toward this.

### How Languages Handle Memory

Programming languages take different approaches to managing memory, and it helps to understand the landscape before diving into what Rust does.

Some languages, like C, leave memory management entirely up to you. You allocate memory when you need it and free it when you're done. Forget to free it and you have a memory leak. Free it too early and you have a dangling pointer. Free it twice and you have undefined behavior. The flexibility is total, and so is the risk.

Other languages, like JavaScript or Python, use a garbage collector that periodically scans for memory that's no longer in use and frees it automatically. This is convenient, but the garbage collector runs at runtime, which adds overhead and can cause unpredictable pauses. For performance-sensitive or resource-constrained systems, this cost may not be acceptable.

Rust takes a third approach. The compiler enforces a set of ownership rules at compile time. If your code violates any of these rules, it won't compile. Because the checks happen at compile time rather than at runtime, there's no performance cost. You get the safety of garbage collection with the performance of manual memory management.

### The Stack and the Heap

To understand ownership, you need a basic picture of where data lives in memory. There are two regions that matter: the stack and the heap.

The stack is fast and structured. Data goes on top, comes off the top, and everything stored on it must have a known, fixed size. All of the scalar types we covered earlier, integers, floats, booleans, characters, as well as fixed-size compound types like arrays and tuples, live on the stack. Allocation and deallocation are essentially instant because there's no searching involved.

The heap is more flexible. When you need to store data whose size can change or isn't known at compile time, like a `String` that can grow, it goes on the heap. The memory allocator finds a block of space large enough, marks it as in use, and gives you back a pointer to that location. Accessing heap data is slower than stack data because the program has to follow the pointer to find it. Managing heap memory is where things get complicated in most languages, and it's exactly where Rust's ownership system earns its keep.

### The Rules of Ownership

Rust's ownership system is built on three rules:

1. Every value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

That third rule is doing the heavy lifting. Let's see it in action.

### Scope and Drop

A scope in Rust is defined by curly braces. When a variable goes out of scope, Rust automatically calls an internal `drop` function to clean up the memory it was using:

```rust
fn main() {
    let fuel = String::from("Give me fuel, give me fire");
    // fuel is valid here, we can use it freely

} // fuel goes out of scope here and Rust drops it
```

Once execution passes the closing brace, `fuel` no longer exists and the memory it occupied on the heap is freed. You don't have to remember to free it, and you can't accidentally use it after it's been freed. The compiler enforces both.

This seems straightforward with a single function, but things get interesting when you start passing values around.

### Move Semantics

Consider what happens when you pass a `String` to a function:

```rust
fn main() {
    let fuel = String::from("Give me fuel, give me fire");

    let (fuel2, len) = calculate_length(fuel);

    println!("The length of '{}' is {}.", fuel2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

When we pass `fuel` to `calculate_length`, ownership of the string moves into the function. The variable `fuel` is no longer valid in `main` after that point. If we tried to use `fuel` after the function call, the compiler would stop us.

To keep using the string in `main`, the function has to return it back. That's why `calculate_length` returns a tuple containing both the string and its length. We then bind those return values to new variables.

This works, but it's awkward. We're doing a lot of passing back and forth just to calculate the length of a string. Fortunately, Rust gives us a better way.

### Copy vs Move

Before we get to that better way, there's an important nuance to understand. Not all types behave like `String` when you assign them to another variable or pass them to a function. Try this with an integer:

```rust
let x = 5;
let y = x;

println!("x is {} and y is {}", x, y);
```

This compiles and works fine. Both `x` and `y` are valid. But if you try the same thing with a `String`:

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}", s1); // This won't compile
```

The compiler stops you because `s1` was moved into `s2` and is no longer valid.

The difference comes down to the `Copy` trait. Types that are simple and live entirely on the stack, like integers, floats, booleans, and characters, implement the `Copy` trait. When you assign one of these to another variable, Rust makes a cheap bitwise copy of the data. It's so inexpensive that there's no reason to prevent you from using the original.

Types that involve heap allocation, like `String`, don't implement `Copy`. Assigning or passing these values moves ownership instead, because having two variables pointing to the same heap data and both thinking they own it would be a problem. When the first one went out of scope and Rust tried to drop the data, the second one would be left pointing at freed memory. The ownership rules prevent this from ever happening.

This distinction between Copy and Move is something you'll bump into constantly in Rust, and understanding it saves a lot of confusion when the compiler tells you a value has been moved.

### Clone

Sometimes you genuinely need a full, independent copy of heap data. That's what the `.clone()` method is for:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 is {} and s2 is {}", s1, s2);
```

Both `s1` and `s2` are valid because `.clone()` creates a complete, deep copy of the data on the heap. The original isn't moved because a brand new allocation was made for the clone.

It's worth being deliberate about when you use `.clone()`. It's perfectly fine and sometimes necessary, but it's also more expensive than a borrow because it duplicates data in memory. If you find yourself reaching for `.clone()` to make the compiler happy, take a moment to consider whether a reference would work instead. That said, don't be afraid of `.clone()` when it's the right tool. Correct and clear code is more important than saving a few allocations, especially while you're learning.

### References and Borrowing

Instead of transferring ownership, we can let a function borrow a value by passing a reference to it. A reference allows access to the data without taking ownership:

```rust
fn main() {
    let fuel = String::from("Give me fuel, give me fire");

    let len = calculate_length(&fuel);

    println!("The length of '{}' is {}.", fuel, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

The `&` in `&fuel` creates a reference to the value without moving ownership. The function signature `s: &String` says it accepts a reference to a `String`. Inside the function, we can read the data through the reference, but `fuel` still belongs to `main`.

Because `calculate_length` never owned the string, it doesn't need to return it. After the function call, `fuel` is still perfectly valid and we can keep using it. This is called borrowing, and it's how you'll work with data most of the time in Rust.

### Mutable References

References are immutable by default. If a function borrows a value, it can read it but can't change it. This makes sense, because you generally don't want someone else modifying something they don't own without your explicit permission.

When you do need to modify borrowed data, you use a mutable reference:

```rust
fn main() {
    let mut fuel = String::from("Give me fuel, give me fire");

    complete_lyrics(&mut fuel);

    println!("{}", fuel);
}

fn complete_lyrics(s: &mut String) {
    s.push_str(", give me that which I desire");
}
```

Three things need to line up for this to work. The variable must be declared with `mut`. The reference must be passed as `&mut fuel`. And the function parameter must accept `&mut String`. All three are required, making it very explicit at every level that mutation is happening.

Rust enforces an important restriction on mutable references: you can have only one mutable reference to a particular piece of data at a time. This prevents data races, which are a category of bug where two parts of a program try to modify the same data simultaneously with unpredictable results.

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // This won't compile

println!("{}, {}", r1, r2);
```

The compiler will refuse this code. Only one mutable reference can exist at a time.

There's a related restriction: you can't have a mutable reference while immutable references to the same data are still in use. This is because code holding an immutable reference doesn't expect the value to change out from under it.

```rust
let mut s = String::from("hello");

let r1 = &s;     // immutable borrow
let r2 = &s;     // another immutable borrow, this is fine
let r3 = &mut s;  // mutable borrow while immutable borrows exist, won't compile

println!("{}, {}, {}", r1, r2, r3);
```

Multiple immutable references are fine because no one is changing the data. But the moment a mutable reference enters the picture, no other references of any kind can coexist. These rules might feel restrictive at first, but they eliminate entire categories of bugs at compile time that would be very difficult to track down at runtime.

### Dangling References

One more safety guarantee worth understanding: Rust prevents you from creating dangling references. A dangling reference is a pointer to memory that has already been freed, and in languages like C they're a notorious source of bugs.

Consider a function that tries to return a reference to a value it created locally:

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```

This won't compile. The variable `s` is created inside `dangle` and will be dropped when the function ends. If Rust let you return a reference to `s`, the caller would receive a reference pointing to freed memory. The compiler catches this and tells you exactly what's wrong.

The fix is to return the `String` itself, transferring ownership to the caller:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

Now the string moves out of the function rather than being dropped, and the caller takes ownership. This is another example of the ownership system preventing bugs that would be very difficult to diagnose at runtime.

### A Note on Slices

Throughout this article series, you've been using `&str` in function parameters and struct definitions without a full explanation of what it is. A `&str` is a string slice, which is a reference to a portion of string data. Slices are a kind of reference that don't take ownership, and they work with other collection types as well, not just strings.

Slices deserve their own article because they're a concept you'll use constantly in Rust, and there are important details around how they relate to ownership. For now, knowing that `&str` is a borrowed view into string data, rather than an owned copy, is enough to make sense of the code you've seen so far.

### Where to Go from Here

Ownership and borrowing are concepts you'll wrestle with for a while. In the early days there's a lot of so-called "fighting the borrow checker," and it can be frustrating. But over time these rules become intuitive, and you start to appreciate that the compiler is catching real bugs, not just being pedantic. The ownership system is what allows Rust to be both safe and fast, and everything else in the language is built on top of it.

There's more to the ownership story, including slices in depth and lifetimes, which I'll cover in future articles. For now, the core ideas of ownership, moving, copying, cloning, borrowing, and the rules around mutable references give you the foundation to start understanding why your code does or doesn't compile. When in doubt, read the compiler's error messages carefully. The Rust compiler is genuinely trying to help you, and its ownership-related messages are some of the most informative you'll find in any language.

### Resources

[The Rust Programming Language, Chapter 4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

[The Rust Programming Language, Chapter 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

[The Rust Programming Language, Chapter 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)

