+++
title = "Pattern Matching with `match`"
date = 2026-04-21
description = "How to use Rust's match expression for exhaustive pattern matching, including enums with data, the wildcard pattern, if let, and Option."
categories = ["Fundamentals"]
tags = ["pattern-matching"]
draft = false
aliases = ["/2026-04-21-rust-fundamentals-match-syntax"]
+++

In previous articles I covered [conditional logic](@/blog/2026-04-13-rust-fundamentals-conditional-logic.md) with `if` and `else`, which works well when you have one or two conditions to check. But Rust has a much more powerful tool for situations where a value could be one of many possibilities: the `match` expression. Once you get comfortable with `match`, it becomes genuinely addictive, and you'll miss it whenever you work in a language that doesn't have it.

### The Basics of match

A `match` expression takes a value and compares it against a series of patterns called arms. Each arm has a pattern and the code to run if that pattern matches. Rust evaluates the arms from top to bottom and runs the code for the first matching arm.

Let's explore this with a Star Wars themed example:

```rust
enum Classification {
    Starfighter,
    Freighter,
    Corvette,
    Frigate,
    StarDestroyer,
}

fn get_ship_type(ship: Classification) -> &'static str {
    match ship {
        Classification::Starfighter => "A small fighter-type craft",
        Classification::Freighter => "A craft which carries cargo",
        Classification::Corvette => "The ship that carried Princess Leia in A New Hope",
        Classification::Frigate => "Like the ship Luke was on at the end of Empire Strikes Back",
        Classification::StarDestroyer => "The classic wedge-shaped subjugator of worlds",
    }
}

fn main() {
    let xwing = Classification::Starfighter;
    let millennium_falcon = Classification::Freighter;
    let tantive_iv = Classification::Corvette;

    println!("An X-wing is: {}", get_ship_type(xwing));
    println!("The Millennium Falcon is: {}", get_ship_type(millennium_falcon));
    println!("The Tantive IV is: {}", get_ship_type(tantive_iv));
}
```

The `get_ship_type` function receives a `Classification` enum and uses `match` to return a different string for each variant. Each line inside the `match` is an arm, consisting of a pattern (like `Classification::Starfighter`), the `=>` symbol, and the code to execute. Because the last expression in each arm is the return value, there are no semicolons after the strings.

A quick note on the `&'static str` return type: the `'static` part is a lifetime annotation. Lifetimes are a topic for a future article. For now, just know that it means these string slices live for the entire duration of the program, which is true of any string literal you write directly in your code.

### Exhaustiveness

One of the most powerful things about `match` is that it must be exhaustive. Every possible value must be covered by an arm. If you added a new variant to the `Classification` enum but forgot to add a corresponding arm, the compiler would refuse to build your program. This is a safety net you'll come to appreciate as your enums grow, because it means you can't accidentally forget to handle a case.

### The Wildcard Pattern

When you only care about specific values and want everything else handled the same way, the `_` wildcard catches anything that wasn't matched by the arms above it:

```rust
fn describe_roll(roll: u32) {
    match roll {
        1 => println!("Critical failure!"),
        20 => println!("Critical hit!"),
        _ => println!("You rolled a {}.", roll),
    }
}
```

The `_` arm here handles every number that isn't 1 or 20. Without it, the compiler would complain that we haven't covered every possible `u32` value, which would be impractical to list out individually.

### Multiple Patterns and Ranges

You can match against multiple values in a single arm using the `|` operator:

```rust
fn describe_roll(roll: u32) {
    match roll {
        1 => println!("Critical failure!"),
        2 | 3 => println!("That's rough."),
        18 | 19 => println!("Nice roll!"),
        20 => println!("Critical hit!"),
        _ => println!("You rolled a {}.", roll),
    }
}
```

For contiguous ranges, you can use the `..=` syntax:

```rust
fn describe_roll(roll: u32) {
    match roll {
        1 => println!("Critical failure!"),
        2..=5 => println!("Not great."),
        6..=14 => println!("A decent roll."),
        15..=19 => println!("Nice roll!"),
        20 => println!("Critical hit!"),
        _ => println!("Invalid roll: {}", roll),
    }
}
```

The `..=` means an inclusive range, so `2..=5` matches 2, 3, 4, and 5. This is much cleaner than writing out `2 | 3 | 4 | 5`.

### Matching Enums with Data

In the [enums article](@/blog/2026-04-17-rust-fundamentals-enum-type.md), we saw that enum variants can carry data. The `match` expression is how you extract that data. Each arm can bind the inner values to variables:

```rust
enum Command {
    Quit,
    Echo(String),
    MoveCursor(i32, i32),
}

fn execute(cmd: Command) {
    match cmd {
        Command::Quit => {
            println!("Shutting down.");
        }
        Command::Echo(message) => {
            println!("{}", message);
        }
        Command::MoveCursor(x, y) => {
            println!("Moving cursor to ({}, {}).", x, y);
        }
    }
}
```

In the `Echo` arm, the `String` carried by the variant is bound to the variable `message`. In the `MoveCursor` arm, both integers are bound to `x` and `y`. This is the real power of combining enums with `match`: you define a set of possibilities, each with its own shape of data, and then handle each one with full access to whatever it contains.

Notice that the `Quit` arm uses curly braces around its body. When an arm contains multiple statements, or when you just want visual clarity, wrapping the body in braces works well. For simple single-expression arms, you can leave them off.

### Matching on Option

One of the most common uses of `match` is working with Rust's `Option` type. `Option` is an enum built into the standard library that represents a value which might or might not exist:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

You don't need to define this yourself because it's always available. Anywhere you might use `null` in another language, Rust uses `Option` instead. The advantage is that the compiler forces you to handle the `None` case, so you can never accidentally try to use a value that doesn't exist.

Here's a practical example:

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

fn main() {
    let numbers = [1, 3, 5, 8, 11];

    match find_first_even(&numbers) {
        Some(n) => println!("The first even number is {}.", n),
        None => println!("No even numbers found."),
    }
}
```

The function returns `Some(num)` when it finds an even number and `None` if it reaches the end without finding one. The `match` in `main` handles both possibilities. If you tried to skip the `None` arm, the compiler would stop you, which is exactly the kind of safety that prevents null pointer errors.

### Simplifying with `if let`

Sometimes you only care about one specific pattern and want to ignore everything else. Writing a full `match` with a `_ => ()` arm for this case feels heavy. Rust provides `if let` as a shorthand:

```rust
let some_value: Option<i32> = Some(42);

if let Some(n) = some_value {
    println!("Got a value: {}", n);
}
```

This is equivalent to:

```rust
match some_value {
    Some(n) => println!("Got a value: {}", n),
    _ => (),
}
```

The `if let` version is more concise when you only need to handle one variant. You can also add an `else` block to handle everything that doesn't match:

```rust
if let Some(n) = some_value {
    println!("Got a value: {}", n);
} else {
    println!("Got nothing.");
}
```

Use `if let` when you care about one pattern. Use `match` when you need to handle multiple patterns or want the compiler to verify you've covered everything. Both have their place, and you'll develop a feel for which to reach for over time.

### Where to Go from Here

The `match` expression is one of Rust's most distinctive features, and it shows up everywhere once you start writing real code. Error handling with `Result`, extracting values from `Option`, branching on enum variants, and even destructuring structs and tuples all use the same pattern matching system. There are more advanced features like match guards and nested patterns that I'll cover in a future article, but the fundamentals covered here will carry you a long way.

### Resources

[The Rust Programming Language, Chapter 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)

[The Rust Programming Language, Chapter 6.3](https://doc.rust-lang.org/book/ch06-03-if-let.html)

[The Rust Programming Language, Chapter 18](https://doc.rust-lang.org/book/ch18-00-patterns.html)