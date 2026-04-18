+++
title = "Conditional Logic"
date = 2026-04-13
description = "Using if, else, and else if expressions in Rust to control program flow, including conditional variable assignment."
categories = ["Fundamentals"]
tags = ["control-flow"]
draft = false
+++

Software would be pretty limited if there were no way to make decisions and follow different logic paths. The ability to branch and provide different outcomes, based on either internal results or external user input, is essential to any useful program. Rust provides this through `if`, `else`, and `else if` expressions.

An important word in that last sentence is "expressions." In many languages, `if` is a statement that controls flow but doesn't produce a value. In Rust, `if` is an expression, which means it evaluates to a value and can be used in places that might surprise you if you're coming from another language. We'll see this in action later in the article.

### The if Expression

An `if` expression starts with the `if` keyword followed by a condition. If the condition evaluates to true, the block of code inside the curly braces runs. If not, it's skipped entirely.

```rust
fn main() {
    let year = 1984;
    if year > 2000 {
        println!("The year is: {}", year);
    }
}
```

Here, `year` holds the value 1984, which is not greater than 2000, so the `println!` inside the block never runs. The program produces no output.

Conditions can be combined using the logical operators `&&` (and) and `||` (or) to build more complex checks:

```rust
fn main() {
    let year = 2022;
    if year > 2000 && year < 2025 {
        println!("The year is: {}", year);
    }
}
```

This condition requires both parts to be true. Since 2022 is greater than 2000 and less than 2025, the message prints. If either condition were false, the block would be skipped.

### Adding else

When you want one thing to happen if a condition is true and something different if it's false, you add an `else` block:

```rust
fn main() {
    let number = 26;
    if number > 30 {
        println!("{} is greater than 30", number);
    } else {
        println!("{} is not greater than 30", number);
    }
}
```

Since 26 is not greater than 30, execution goes to the `else` block and the message "26 is not greater than 30" is printed. Only one of the two blocks will ever run for a given value.

### Chaining with else if

When there are more than two possible paths, `else if` lets you check additional conditions in sequence:

```rust
fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

This example, adapted from The Rust Programming Language, illustrates an important nuance. The number 6 is divisible by both 3 and 2, but only the first matching branch runs. Rust evaluates the conditions from top to bottom and executes the block for the first one that's true, then skips everything else. The output here is "number is divisible by 3" and the `% 2` check never gets a chance, even though it would also be true.

Long chains of `else if` can become hard to read and reason about. Rust has a more powerful tool for this called `match`, which I'll cover in a future article. If you find yourself writing more than two or three `else if` branches, it's worth knowing that a better option exists.

### Using if in a let Statement

This is where Rust's treatment of `if` as an expression really shines. Because `if` produces a value, you can use it on the right side of a `let` binding to conditionally assign a variable:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
```

The variable `number` gets the value 5 because `condition` is true. If `condition` were false, `number` would be 6 instead. This is more concise than writing out a full `if` block that assigns to a mutable variable, and it lets `number` remain immutable since it's only assigned once.

There is one constraint to keep in mind. Both branches of the `if` expression must return the same type. Rust needs to know at compile time what type `number` will be, and it can't be an integer in one branch and a string in the other. This is the kind of safety check that catches subtle bugs before your program ever runs.

### Where to Go from Here

With conditional logic in place, the next step is learning how to repeat things. The next article covers Rust's three loop types, which combined with what you've learned here, will give you full control over how your programs flow through their instructions.

### Resources

[The Rust Programming Language, Chapter 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html)