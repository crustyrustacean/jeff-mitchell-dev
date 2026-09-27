+++
title = "De-mystifying `if-let`"
date = 2026-09-27
description = "How to think about the `if-let` syntax"
categories = ["Field-Notes"]
tags = ["syntax", "if-let"]
draft = true 
aliases = ["/2026-09-27-rust-field-notes-de-mystifying-if-let"]
+++

The `if-let` syntax has always given me difficulty.

Yesterday, I was working in the `taxus` code base, and I had an epiphany.  

Take this code block:

```rust
Commands::Routes { dir } => {
            if let Err(e) = run_routes(&dir) {
                render_error(&e);
                std::process::exit(1);
            }
        }
```

This snip is part of a match arm that lives in the `taxus` binary. Originally this block was written like this:

```rust
Commands::Routes { dir } => match run_routes(&dir) {
            Ok(()) => {}
            Err(e) => {
                render_error(&e);
                std::process::exit(1);
            }
        },
```

There's nothing wrong with this, the match calls the `run_routes()` function, and evaluates the returned result against two arms:

- the happy path: basically do noting, execution continues
- the unhappy path: render an error and halt execution

This is in effect the reverse case of what we're shown in the Rust Book. The `if-let` syntax helps make code more concise where we have two paths, but don't care about one of them. In this case, we only care about the unhappy path. So, the code in the first snip is the cleaner way to express it. 

We discard the happy path altogether and explicitly handle the unhappy path.
