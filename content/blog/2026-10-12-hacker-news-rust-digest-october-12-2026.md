+++
title = "Hacker News Rust Digest — October 12, 2026"
date = 2026-10-12
description = "An AI-port week: Quake lands in safe Rust in the browser, the TypeScript compiler gets the same treatment, Eurydice compiles Rust back to readable C, Rust-first programmers start writing C tutorials, and a smart-home library reverse-engineers TP-Link's lock-in."
categories = ["Digest"]
tags = ["rust", "hacker-news", "llm", "webassembly", "typescript", "compilers", "iot"]
draft = true
aliases = ["/2026-10-12-hacker-news-rust-digest-october-12-2026"]
+++

A week where LLM-assisted Rust ports kept making headlines — a playable Quake in safe Rust, a token-burning TypeScript compiler port — while Rust-first programmers started writing the reverse tutorial, and an IoT library fought back against firmware lock-in.

## Show HN: Quake ported to safe Rust, playable in browser

A self-described "slop rust port" — SRP really does stand for that — brings Quake to the browser in memory-safe Rust via WASM, running smoothly even on phones. The thread (239 points, 180 comments) turned into a meditation on the milestone: the top comment blessed the slop and framed it as standing on decades of human work in browser rendering, WASM, and Rust that let an LLM "come in at the (relatively speaking) last second and claim a win," while another noted with mock outrage that the famous `0x5f3759df` fast-inverse-sqrt constant is nowhere in the source.

[HN Discussion](https://news.ycombinator.com/item?id=50016312) | [Article](https://quake-srp.pages.dev/)

## Penguin Mail – open-source Rust email client for Linux with AI

A GTK 4/libadwaita email and calendar client written in Rust, aggregating Gmail, Outlook, Fastmail, iCloud, and plain IMAP/POP3, with OpenPGP and S/MIME signing whose keys never leave your machine. It led the week's Rust-adjacent threads at 242 points and 187 comments, where the author fielded questions but still caught flak for the embedded "with AI" chat window and for calling a month-old codebase version 1.0 — "a project with the complexity of an email client cannot be considered stable in a month." Sparrow nostalgics came out in force.

[HN Discussion](https://news.ycombinator.com/item?id=49984716) | [Article](https://penguin-mail.com/)

## C for Rust programmers

A Rust-first systems programmer writes the reverse of the usual tutorial, coming to C after growing up on Rust and being genuinely startled by what isn't there (172 points, 204 comments). The most-loved comment flipped the genre on its head: the fact that a Rust programmer's reaction to C is shock rather than "look at Rust's fancy safety stuff" shows the field's baseline has lifted — one reader went as far as predicting a world without memory corruption within their lifetime. A C standards participant countered that C safety lives in tooling and warnings rather than the spec, while others noted the hardest habit to unlearn in the other direction is losing the freedom to write quick-and-dirty exploratory code.

[HN Discussion](https://news.ycombinator.com/item?id=50005743) | [Article](https://bd103.dev/blog/2026-10-07-c-for-rust-programmers/)

## Port of the TypeScript compiler, checker and lsp to Rust, by LLM

pingdotgg's ts-rust is an LLM-ported TypeScript compiler, checker, and LSP to Rust, reportedly produced with over $400,000 of API-priced tokens — commenters suspected subscriptions and eval credits in practice. The README earned real praise for its "Slop Line" convention marking everything below it as LLM-written, and a TypeScript team member confirmed in the thread (117 points, 222 comments) that three such ports had appeared within a single week and the team is paying attention. Skeptics asked how much `unsafe` the output contains and whether a port validated only against existing tests would silently miss the cases where the original authors relied on common sense.

[HN Discussion](https://news.ycombinator.com/item?id=50000676) | [Article](https://github.com/pingdotgg/ts-rust)

## Compiling Rust to readable C with Eurydice

LWN covers Eurydice, a tool that compiles Rust to readable C so Rust code can target platforms without a Rust compiler (141 points, 39 comments). Commenters confirmed that Rust's runtime safety checks survive the trip — bounds checks live in the IR and carry through to the C output — and one suggested readable C output could even enable a full-source bootstrap of rustc. Others joked about the name ("you're rid o' C") and pointed at variables like `uu____0` as evidence the "readable" claim still has room to run.

[HN Discussion](https://news.ycombinator.com/item?id=50027853) | [Article](https://lwn.net/Articles/1055211/)

## Tapo (Rust/Python library) now speaks TP-Link's TPAP protocol

The unofficial Rust client library for TP-Link Tapo smart plugs, lights, and hubs reverse-engineered the vendor's new undocumented TPAP protocol — SPAKE2+ login and all — after firmware updates began locking out third-party clients unless users flip a "Third-Party Compatibility" switch (133 points, 68 comments). The author walked the thread through the protocol genealogy, from Kasa's unauthenticated XOR-obfuscated JSON on port 9999 to 2023's KLAP to today's TPAP. The discussion doubled as a state-of-the-RE report: current LLMs paired with Ghidra or IDA have made reverse-engineering closed protocols a weekend project instead of a months-long slog, though some commenters mostly wanted to talk about divesting from vendors hostile to interoperability.

[HN Discussion](https://news.ycombinator.com/item?id=49978563) | [Article](https://mihai.dinculescu.dev/posts/tapo-speaks-tpap/)

---

If one theme ran through the week, it's that the line between "AI slop" and legitimate engineering is being actively redrawed in Rust's territory: a Quake port cheerfully calls itself slop, a TypeScript compiler port draws a literal Slop Line through its own README, and a $400k token bill sparks a debate about whether the result is a serious project or the Doom-on-a-microwave of our era. Meanwhile, quieter signals suggest Rust is becoming the default lens — "C for Rust programmers" now exists as a genre because Rust-first programmers are a real population, and tooling like Eurydice treats Rust as the source of truth with C as the compilation target. The slop makes the headlines, but Rust keeps winning the boring, structural battles.
