+++
title = "Hacker News Rust — Digest May 11, 2026"
date = 2026-05-11
description = "This week's top Rust stories on Hacker News span a Bun rewrite milestone, a Lisp-on-Rust experiment, Nvidia's CUDA tooling, and new AI tooling built in Rust."
categories = ["Digest"]
tags = ["rust", "hacker-news", "systems-programming", "tooling", "llm", "cuda", "bun"]
draft = false
+++

A lively week on Hacker News for Rust: tooling continued its march into new domains, a major JavaScript runtime quietly swapped its foundation, and a playful language experiment stole the weekend.

### Show HN: Rust but Lisp

A weekend hack that hit 199 points and sparked 70 comments, "Rust but Lisp" is a project that adds a Lisp-style S-expression syntax layer on top of Rust semantics. The discussion ranged from genuine enthusiasm about homoiconicity and macros to skepticism about whether the ergonomics hold up at scale. It's the kind of off-the-wall experiment that HN loves to argue about productively.

[HN Discussion](https://news.ycombinator.com/item?id=48078575) | [Article](https://github.com/ThatXliner/rust-but-lisp)

---

### Bun Ported to Rust in Six Days

Jarred Sumner announced that Bun's experimental rewrite from Zig to Rust reached 99.8% test suite compatibility on Linux x64 glibc — and that it took only six days. The HN thread drew comparisons to other high-profile rewrites and debate over whether the Zig-to-Rust switch signals a broader shift in the systems space. The speed of the port surprised even veteran Rust developers in the comments.

[HN Discussion](https://news.ycombinator.com/item?id=48073680) | [Article](https://xcancel.com/jarredsumner/status/2053047748191232310)

---

### Nvidia Releases CUDA-Oxide 0.1: Experimental Rust-to-CUDA Compiler

Nvidia quietly shipped CUDA-Oxide 0.1, an experimental compiler backend that lets Rust code target CUDA. Coverage on Phoronix surfaced it on HN. While still pre-production, it opens a path for GPU kernel development in safe, idiomatic Rust a long-requested capability in the ML and HPC communities where Rust adoption has otherwise stalled at the CPU boundary.

[HN Discussion]() | [Article](https://www.phoronix.com/news/NVIDIA-CUDA-Oxide-0.1)

---

### Atlas Pure Rust Inference Engine

Avarok Cybersecurity published Atlas, a pure-Rust inference engine aimed at local AI workloads. It joins a growing set of Rust-native ML runtimes competing with Python-wrapped C++ stacks. The HN submission is fresh today and discussion is just beginning, but the project points to a maturing Rust ecosystem for inference at the edge.

[HN Discussion](https://news.ycombinator.com/item?id=48094162) | [Article](https://github.com/Avarok-Cybersecurity/atlas)

---

### tsz: TypeScript Checker and LSP Written in Rust

A second TypeScript tooling project written in Rust surfaced this week (alongside an earlier tsz.dev submission), explicitly positioning itself as a competitor to Microsoft's native tsgo rewrite. The ambition is notable: two independent teams are now racing to out-perform TypeScript's own Go-based checker using Rust, suggesting the tooling space sees real performance headroom to be claimed.

[HN Discussion](https://news.ycombinator.com/item?id=48078113) | [Article](https://github.com/mohsen1/tsz)

---

The standout story of the week is undeniably the Bun rewrite: porting a full JavaScript runtime from Zig to Rust in six days with 99.8% test parity is an extraordinary data point on how far Rust tooling and developer familiarity have come, and it will likely shape how teams think about language migration timelines for years.
