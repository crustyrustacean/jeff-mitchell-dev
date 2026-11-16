+++
title = "Hacker News Rust Digest — November 16, 2026"
date = 2026-11-16
description = "Rust 1.100 lands the never type and custom allocators, while standard-library safety research, an experimental JIT, and native applications test the ecosystem's promises."
categories = ["Digest"]
tags = ["rust", "hacker-news", "rust-1-100", "memory-safety", "jit", "gpui", "xmpp", "ratatui"]
draft = true
aliases = ["/2026-11-16-hacker-news-rust-digest-november-16-2026"]
+++

A week of high-signal Rust discussion on Hacker News, led by a milestone stable release and a closer look at the unsafe foundations beneath safe APIs, with faster development loops and native applications rounding out the mix.

## Rust 1.100: The Never Type, Custom Allocators, and Cargo Improvements

Rust 1.100 drew 901 points and 323 comments as it stabilized the never type, the core allocator API for `Vec` and `Box`, and `DropGuard`, alongside Cargo warnings for unused dependencies and opt-in dependency cooldowns. The HN discussion celebrated long-awaited features finally landing, especially custom allocators and the change making `cargo install` use a packaged lockfile by default. Commenters also debated the security tradeoffs of delaying dependency updates and the limited compatibility break from changing never-type fallback in older editions — a reminder that even a celebratory release has migration details worth reading.

[HN Discussion](https://news.ycombinator.com/item?id=50472038) | [Article](https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/)

## Memory Safety Bugs in Rust's Standard Library

Microsoft's FORGE Lab described memory-safety issues in `std` and `core`, including cases where entirely safe caller code can trigger undefined behavior through unsound library internals, drawing 193 points and 199 comments. The research groups the failures around misplaced trust: assuming callbacks will not panic, trusting safe trait implementations to behave consistently, and relying on the underlying system. HN's discussion focused on whether panic unwinding is worth its complexity and why safe APIs must remain sound even when callers supply incorrect implementations; several commenters stressed that these findings are bugs in Rust's safety boundary, not permission for safe code to cause undefined behavior.

[HN Discussion](https://news.ycombinator.com/item?id=50478701) | [Article](https://forge-blog.github.io/posts/rust-std-memory-safety/)

## Relax: An Experimental JIT for Faster Rust Iteration

Relax, from the Makepad project, proposes running Rust source through a live compiler and JIT that compiles functions on demand and hot-patches edits without restarting the process, earning 180 points and 114 comments. Its authors explicitly describe it as an AI-generated reimplementation for development iteration, not a replacement for release builds or a tool that automatically inherits Rust's safety guarantees. HN welcomed the ambition and drew comparisons with Lisp-style live environments, but early testers reported compilation failures, apparent miscompilation, and cases slower than ordinary incremental builds. The interesting question is not just how quickly it reaches a first frame, but how broadly its correctness and latency claims survive outside Makepad's own workloads.

[HN Discussion](https://news.ycombinator.com/item?id=50458849) | [Article](https://makepad.github.io/relax/)

## Lonewolf 0.3: A Rust XMPP Server Gains Federation

Lonewolf 0.3 adds server-to-server federation, direct TLS, and extensions used by desktop clients to its standalone Rust XMPP server, attracting 161 points and 57 comments. The release remains pre-1.0 and single-node, with federation requiring trusted certificates and SASL EXTERNAL rather than supporting Server Dialback. HN welcomed another implementation of an open protocol, while users of established servers such as Prosody questioned the need to switch. A recurring argument was that XMPP's bigger adoption problem is polished, consistent clients rather than a shortage of capable servers, with a separate debate over trust in AI-assisted protocol implementations.

[HN Discussion](https://news.ycombinator.com/item?id=50484007) | [Article](https://github.com/lonewolf-im/lonewolf/releases/tag/v0.3.0)

## DBDelve: A Native Database Client Built with Rust and GPUI

DBDelve is a cross-platform database client built with Rust and GPUI, pitched as a responsive alternative to heavier desktop tools; its Show HN thread reached 125 points and 50 comments. The author's benchmarks report 133 MB of idle memory versus DBeaver's 606 MB and faster rendering of large result sets, while explicitly noting that loading a million-row table is a stress test rather than a normal workflow. HN's questions were practical: database coverage, schema-aware completion, connection imports, and whether a free project can sustain development. The thread also exposed some GPUI ecosystem friction, with the author explaining the choice among community distributions when the official crates.io release lags behind.

[HN Discussion](https://news.ycombinator.com/item?id=50427934) | [Article](https://github.com/ShayanAbbas1/dbdelve)

## Terminactory: A Ratatui Factory Game Beside Your Coding Agents

Terminactory puts an idle factory game in the terminal using Rust and Ratatui, with production chains, research, and in-game messages delivered through coding-agent hooks; it collected 169 points and 40 comments. The most entertaining HN connection was to NetHack's decades-old mail-delivery mechanic: agents sending messages into a game turn out to have a surprisingly familiar ancestor. Feedback mixed enthusiasm for the ASCII presentation with complaints about visual density, prompting the author to add a calmer interface preset. Other commenters questioned the AI-assisted README and lore, leading the author to commit to rewriting the narrative material personally.

[HN Discussion](https://news.ycombinator.com/item?id=50497132) | [Article](https://gitlab.com/snipedbywifi/terminactory)

---

The week's strongest contrast is between Rust's mature guarantees and the work needed to extend or preserve them: 1.100 delivers long-awaited foundations, while the FORGE findings show how carefully unsafe internals must handle panics and untrusted behavior from safe code. Relax takes a different bet, trading a conventional ahead-of-time workflow for faster feedback, but still has to earn confidence outside its home project. Meanwhile, DBDelve, Lonewolf, and Terminactory show the language reaching users through very different interfaces — where responsiveness, interoperability, and polish matter at least as much as the implementation language.
