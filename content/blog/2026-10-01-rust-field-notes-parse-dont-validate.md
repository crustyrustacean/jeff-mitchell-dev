+++
title = "Parse, don't validate"
date = 2026-10-01
description = "Draft outline — a recurring shape in Rust: raw untrusted input at the boundary, converted into a typed domain value."
categories = ["Field Notes"]
tags = ["field-notes", "rust", "tryfrom"]
draft = true
+++

# Parse, don't validate — DRAFT OUTLINE

<!-- RAW MATERIAL — Jeff's outline — write from this, own words. Flip draft = false when ready to publish. -->

## Title candidates

- "Parse, don't validate" (the famous name — see link below)
- "The trick was never hidden"
- "The shape is the same every time"

## The arc (how this post actually happened — Oct 1, BuildFEST project)

- Opened the day sure I couldn't build the thing at all: "how would I even do a form and any sort of auth?"
- Ran the wall card on it. **What do I see?** → hardcoded data with a structure (the blog's rotation widget, fed by a JSON file). **What do I need?** → a domain data model with the same structure.
- Designed the boundary: form → raw struct → `TryFrom` → domain type.
- Then the extraction: *"This shape will be the same no matter what you're working with — whenever you take in data from the outside world, that you don't trust."*
- And the relief: *"the details of how are smaller questions, likely searchable."*

## The shape (the one code example the post needs)

```rust
struct RawEntry {
    what: String,
    artist: String,
}

impl TryFrom<RawEntry> for RotationEntry {
    type Error = String;

    fn try_from(raw: RawEntry) -> Result<Self, Self::Error> {
        // trim, validate, build the domain type
    }
}
```

- After the boundary, the compiler guarantees the rest of the program — the type can't hold an invalid entry. Make illegal states unrepresentable.
- The std library is already this shape: `"192.168.1.1".parse::<IpAddr>()`.

## Where I've seen it before (receipts)

- Zero To Production: the signup form converted into validated user types. Done it once, didn't file it.
- Alexis King, ["Parse, don't validate"](https://lexi-lambda.github.io/blog/2019/11/05/parse-dont-validate/) — the famous essay. I re-derived the shape before knowing it had a name.
- Every config loader, CLI arg parser, and JSON API on earth is this body in different clothes.

## Alive sentences (mine, from the night — verbatim, use anywhere)

- "This shape will be the same no matter what you're working with — whenever you take in data from the outside world, that you don't trust."
- "I've done this before, seen it talked about before, and finally I think it's forming into something I reach for."
- Ending candidate: "This shape will be the same every time, it's just the details of how, which are smaller questions, likely searchable."

## Notes to self (voice rules)

- Locate it: Oct 1, BuildFEST, stuck on auth/forms for the MusicFeed form. Name the wall and the hour.
- Don't state the significance — let the discovery speak. (The aluminum rule.)
- Gig, not album: post it rough. De-mystifying if-let proved the Field Notes format works.
- Possible close: my recipe box of recurring shapes grew by one today — and this one came from my own project, not a book.

## Open question (maybe the next post)

- Where else does this shape already live in my code? Go find one instance in an older project (r2-photo-api, halation, taxus) and link it — proof the shape was always there, unrecognized.
