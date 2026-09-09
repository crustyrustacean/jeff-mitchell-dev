+++
title = "Flux Web: The Too Ambitious Networking Book"
date = 2026-11-30
description = "A post about the GCRA based rate limiter I made in Rust"
categories = ["Projects"]
tags = ["rust", "networking"]
draft = true
aliases = ["/2026-11-30-the-too-ambitious-networking-book"]
+++

I'm going to build my own web framework, from scratch, in Rust.

Am I insane?

Probably.

<!-- more -->

I want to do this because the web framework solutions that are out there now are *too complicated*. They assume you're a computer scientist with years of experience.

I want the benefits of Rust in backend web services.

I want something like `Express` in NodeJS land that essentially lets me compose a web service from easy pieces. Am I sacrificing capability? Probably. If I want that though, I can just go pick up Axum or Actix Web.

Here's what I'm doing.

I'm starting this book: [Flux HTTP Docs](https://crusty-rustacean.com/flux-http-docs) through which I'll pick up enough networking knowledge to build something from the ground up. I'm not expecting anyone else to be interested in using this thing, it's primarily for me.

It will have:

- straighforward way of composing routes
- complexity tucked in the "engine" behind the scenes
- built in Tera templating
- built in static asset serving
- built in link to cloud storage via Apache OpenDAL
- hostable via Docker or via [Shuttle](https://shuttle.dev)

It may take years, I may never finish, but this is my goal.
