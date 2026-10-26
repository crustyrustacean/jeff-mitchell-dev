+++
title = "De-mystifying Slices (with a Table of Test Cases)"
date = 2026-10-25
description = "When to reach for a slice, what the two ampersands mean, and a table-of-test-data pattern that finally made them click."
categories = ["Field Notes"]
tags = ["syntax", "slices", "testing"]
draft = false
+++

Slices have always been fog for me. I could read `&[(&str, &str)]` in someone else's code, roughly, but I could never *write* it without guessing: why two ampersands? Is that an array? A vector? Do I need a lifetime here? Every time, I'd burn a cycle re-deriving it from first principles — or give up and reach for a `Vec` because it felt safer.

The other day I was writing tests for a small lookup function in a Tauri app — name in, URL out — and wanted to do it properly, tests first. The natural shape of the test data was a table: pairs of inputs and expected outputs, looped over, one assertion per row. At first I fumbled for an array, but that wasn't right. I thought vaguely about slices. The vagueness cleared after some headbashing and this time it stuck. Here's the background I was missing, then the pattern.

## What a slice actually is

Background first, because "when do I reach for one" makes no sense until you know what it is. If you've read the post on [compound data types](@/blog/2026-04-11-rust-fundamentals-compound-data-types.md), you know `[T; N]` is an *array*: fixed length, and the length is part of the type. `[(&str, &str); 3]` is a different type from `[(&str, &str); 2]`. That makes arrays awkward as function parameters — hardcode the length and your function only accepts one size of data.

A slice, `&[T]`, is just a **view over a contiguous run of items**: a pointer to the first element plus a length. Nothing more. It owns nothing and borrows from something else — that's the borrowing half of the [ownership model](@/blog/2026-04-20-rust-fundamentals-memory-ownership-borrowing.md).

That "view over anything contiguous" property is the whole answer to *when you reach for one*:

- a slice over an array `[a, b, c]`
- a slice over a `Vec` (`&my_vec` coerces to `&[T]`)
- even a slice over part of either, like `&xs[1..3]`

So the rule for function parameters is the same one you already know from strings — prefer `&str` over `&String` — generalized: **take the widest borrow, `&[T]`, so callers can pass arrays, vectors, or sub-ranges without copying.** Once I noticed that `&str` *is* a slice (of bytes, roughly), the two ideas collapsed into one and the fog thinned. Slices weren't a third mystery; they were the general form of the string-slice trick I'd been using all along.

## The pattern: a table of test cases

Back to the tests. The lookup I was writing takes a `&str` and returns the URL for a known destination, or an error for anything else. The test data I wanted was two tables: valid pairs, and rejected inputs.

The syntax that stopped me for years is this line:

```rust
let pairs: &[(&str, &str)] = &[("red", "#ff0000"), ("green", "#00ff00")];
```

Two ampersands, and both are doing a different job. The one in the **type** says "this variable is a borrowed view" — it's the slice type annotation. The one on the **literal** says "create the array `[...]`, then borrow it" — because the literal alone is an array, with the length baked into its type, and the variable declared the wider slice type. Type and value meet in the middle. That's it. That's the whole trick.

The loop is the second half, and it has its own ampersand with a different job again — this one lives in the *pattern* and strips the reference layer as it destructures:

```rust
for &(input, expected) in pairs {
    assert_eq!(lookup(input), Ok(expected));
}
```

Without the `&` in the pattern, `input` and `expected` come out as `&&str`, which works but produces uglier errors when something's wrong. With it, they arrive as plain `&str`, because `&str` is `Copy` — peeling the reference just copies the pointer. Pattern position: strip. Type position: borrow. Saying that to myself has replaced an hour of guessing.

For the rejects table — a plain slice of strings — the one habit that matters is naming the input in the failure message, because when a looped assertion fails, a bare message doesn't tell you *which* row tripped it:

```rust
let bad: &[&str] = &["", "RED", " red "];
for &input in bad {
    assert!(lookup(input).is_err(), "expected {input:?} to be rejected");
}
```

The `{input:?}` inline capture uses Debug formatting, whose quotes-and-escapes output makes whitespace padding like `" red "` actually visible in the failure.

And a bonus from the same session: comparing the whole `Result` — `assert_eq!(lookup(input), Ok(expected))` rather than pulling the value out with `unwrap` — means a failure prints both sides of the equation, expected and actual, and skips any panic-in-a-test awkwardness entirely.

## The rule I'm keeping

Reach for a slice whenever a function wants to *look at* a run of items it didn't create: `&[T]` for collections, `&str` for text, never `&Vec<T>` or `&String`. And for test data specifically: the table is a slice literal of pairs, the loop destructures with a leading `&`, and every assertion message names its row. Write it enough times and the two ampersands stop being fog and start being grammar.
