+++
title = "Pro Tip: Templates and Cargo Generate"
date = 2026-12-28
description = "A tip I discovered when using cargo generate to scaffold something new."
categories = ["Tips"]
tags = ["rust", "cargo-generate"]
draft = true
aliases = ["/2026-12-28-cargo-generate-pro-tip"]
+++

A short piece with a tip about how to use `cargo-generate` to start a new project.

<!-- more -->

Half of the internet is down today, so like a barbarian I write on the version of my blog site that lives on GitHub. No database, no Shuttle...guess there's something to be said for simplicity.

Speaking of the [new blog](https://crustyrustacean-dev-blog-5d59.shuttle.app/) site, I like what Claude and I generated with it but want to take it one step further to see if I can make it a more *generic* platform. I dream of Wordpress, backed by the reliability and scalability of Rust, but acknowledge I'll likely never ever get there.

A guy can dream though.

In the recent past I discovered `cargo-generate` which let's you scaffold a new project based on any repo you want. However, in this case, because the base project uses the `tera` crate and had templates with curly braces, `cargo-generate` would tend to get confused and error out when building the new thing.

After some head banging against the docs, what I found is you have to add a `cargo-generate.toml`, that looks like this:

```toml
[templates]
exclude = [
    "templates/*",
]
```

Put this in the root *of the project you're using as the template* and when you do:

```bash
cargo generate --init --git git@github.com:<your-account-name>/<project-to-clone>,GitHub
```

The content inside the templates folder will be ignored but still copied, allowing you to create a nice, fresh starting point.

That's it! Just wanted to get this up here to save folks some headbanging.

Good luck!
