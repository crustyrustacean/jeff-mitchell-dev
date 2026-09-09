+++
title = "Comments"
date = 2026-04-07
description = "How to use regular comments and documentation comments in Rust, and why commenting your code matters."
categories = ["Fundamentals"]
tags = ["getting-started"]
draft = false
aliases = ["/2026-04-07-rust-fundamentals-comments"]
+++

I'm going to start this article by asking you to take my advice, because I'm not using it. I say that because I'm terrible at commenting my code. Comments are important, don't be like me.

Before I continue, I've covered the following basics so far:

Now, let's talk about leaving yourself some bread crumbs.

### Why Comment?

You could struggle for days on a particular piece of your program. If you're diligent with commenting, you'll have something to come back to later when faced with a similar problem. Comments are a great way to document the problem-solving process. Try to make them reflect the thinking behind the code, not so much what the code does. The code itself tells you what it does. The comments should tell you why.

This is a lesson I keep learning the hard way. I'll write something, understand it perfectly in the moment, then come back two weeks later and stare at it like it was written by a stranger. A few well-placed comments would have saved me every time.

### Regular Comments

Comments in Rust are prefaced with two forward slashes:

```rust
// This is a comment in Rust.
```

The compiler ignores everything after the `//` on that line. You can place comments on their own line or at the end of a line of code:

```rust
let life = 42; // the answer to everything
```

For comments that span multiple lines, each line gets its own `//`:

```rust
// This is the first line...
// ...of a multi-line comment.
```

Comments tend to be most useful right next to the code they describe:

```rust
fn main() {
    // Bind an integer to the variable named life,
    // then print the meaning of life to the console.
    let life = 42;
    println!("What is the meaning of life? {}", life);
}
```

### Documentation Comments

Now we get to something I find genuinely amazing about Rust. There is a special kind of comment that powers Rust's built-in documentation system. Documentation comments use three slashes instead of two:

```rust
/// Calculates the sum of two integers.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

These `///` comments are picked up by Cargo's documentation tool. When you run:

```bash
cargo doc --open
```

Cargo generates a full HTML documentation site for your project, complete with all the documentation comments you've written. It looks professional and is searchable. The fact that this is built right into the toolchain, for free, is one of those things that makes Rust a joy to work with.

Documentation comments support Markdown, so you can use headings, code blocks, lists, and links within them. The `# Examples` heading you see above is a convention in the Rust community. The code examples inside documentation comments are even run as tests when you execute `cargo test`, which helps ensure your documentation stays accurate as your code evolves.

### Module-Level Documentation

There is one more comment style worth knowing. The `//!` syntax is used to document the item that contains the comment, rather than the item that follows it. You'll most often see this at the top of a file to describe what the module or crate does:

```rust
//! # My Awesome Crate
//!
//! This crate provides utilities for doing amazing things
//! with numbers and strings.

/// Adds two numbers together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

The difference is subtle but important. `///` documents what comes after it. `//!` documents what surrounds it. When Cargo generates your documentation, the `//!` comments become the front page description of your crate or module.

### Conclusion

Comments are one of those things that feel like a chore in the moment but pay dividends later. Regular comments help you and your teammates understand the reasoning behind code decisions. Documentation comments take it further by integrating with Cargo to produce professional documentation automatically. Get in the habit early. Your future self will thank you, trust me on this one.

### Resources

[The Rust Programming Language, Chapter 3.4](https://doc.rust-lang.org/book/ch03-04-comments.html)

[The Rust Programming Language, Chapter 14.2](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)