+++
title = "Quick Take: Generics"
date = 2026-11-02
description = "A quick take on generic types in Rust."
categories = ["Advanced Guides"]
tags = ["rust", "generics"]
draft = true
+++

Generics are something I struggle with. I sort of get the concept of them, you specify a container that could accomodate many types of data, then for a given instance, tell the Rust compiler what you want that type to be. I don't have enough experience to know when to make something generic, it doesn't come to me as an obvious choice to make.

<!-- more -->

As of this past week, things *may* be starting to become more clear.

Let's look at an example:

```Rust
struct Message<T> {
    id: i32,
    content: T,
}
```

We've made a struct called `Message`. The `<T>` type parameter is necessary to make the struct generic. This struct has two fields, an `id` field, which is a simple integer type, and a content field, which, by using the generic type parameter `<T>` tells the Rust compiler it's generic.

Now, to use this struct, we do:

```Rust
struct Message<T> {
    id: i32,
    content: T,
}

fn main() {
    let msg = Message {
        id: 1,
        content: "This is some content".to_string()
    };

    println!("Message id: {}", msg.id);
    println!("Message content: {}", msg.content);
}
```

In this code, we instantiate an example of our message struct, and simply print out it's fields to the console. The Rust compiler knows we want the content field to be a `String` type, as we've used the `.to_string()` method on the text data.

This example is a little silly, as we can very simply set the `content` field to be a `String` type right off the bat.

So why would we want to leverage generics? For future adaptation. If our needs change, we don't have to re-write the entire struct every time before using it. By setting the type to be generic right from the get-go, we set ourselves up to make changes in the future.

To conclude this short thought piece, generic structs are useful when:

- you want something reusable across types
- you want extensibility, design once, use for any content later

Generic structs give you the safety of the Rust type system, with the flexibility to future-proof as needed.

## References:

[Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
