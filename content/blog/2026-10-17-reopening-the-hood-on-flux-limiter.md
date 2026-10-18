+++
title = "Re-opening the Hood on flux-limiter: First Principles, Readable Rust, and a Benchmark Reality Check"
date = 2026-10-17
description = "A year after publishing my GCRA rate limiter, a stranger's benchmark crate revealed it was the slowest column in their comparison table. This is the audit, the first criterion numbers of the crate's life, and what running before walking actually cost."
categories = ["Projects"]
tags = ["rust", "flux-limiter", "gcra", "benchmarking", "criterion", "ai-assisted"]
draft = true
aliases = ["/2026-10-17-reopening-the-hood-on-flux-limiter"]
+++

About a year ago, I decided to build and publish a rate limiter crate to crates.io called flux-limiter, based on the Generic Cell Rate Algorithm (GCRA).  
The Rust ecosystem already had established rate-limiting libraries. I didn't start this project because the world was desperate for another rate limiter. I built it because I wanted to understand how one worked.

*(This is Part 1 of the series. [Part 0 is here](/blog/flux-limiter-gcra-rate-limiting-made-easy/): the announcement post I wrote at the time and never published, with every unverified claim and a race condition still in it. Read it first — the rest of this post makes considerably more sense once you've seen what I believed back then.)*

## Why build it in the first place?

When I set out to write flux-limiter, I had three specific goals.

The first was understanding from first principles. Rate limiting is simple until you push past basic token buckets and encounter GCRA, an algorithm rooted in old telecom networks that replaces background timers with a single timestamp calculation called Theoretical Arrival Time (TAT). I wanted to work through the math and state transitions myself rather than treating rate limiting as a black box.

The second was code that isn't buried in generic soup. If you've ever tried to read the source code of mature, hyper-optimized Rust crates to learn how they work, you know the pain. Between densely nested trait bounds, phantom types, custom middleware layers, and zero-cost abstraction gymnastics, the core algorithm is almost impossible to follow. I wanted code where a human could open src/flux\_limiter.rs and read the algorithm top to bottom in five minutes.

The third was approachable, thorough documentation: a crate that didn't dump raw API signatures onto docs.rs, but explained burst tolerance, error handling strategies (fail-open vs. fail-closed), and how to wire it into a real web service.

## The catch: running before walking

My habit in Rust is to bite off slightly more than I can chew, so I leaned heavily on AI assistance to get the project over the finish line. It helped me wire up the concurrency model, polish the ergonomics, and write out the documentation. The tests passed, I published it to crates.io, and I even started dogfooding it in my own over-ambitious hobby project (casually trying to re-invent Instagram in Rust). As far as I knew, I was my own only production user, and the crate did its job cleanly.  
Then, this Spring, the download counter on crates.io suddenly spiked.  
For a brief moment, I did a small happy dance, thinking people had stumbled across my well-documented little rate limiter and started adopting it. Then I paused and said, *"Wait."*  
After a bit of digging, I found the truth in another crate's `Cargo.toml`. A new crate called `quota`, self-described as the "Fastest Lane-parallel Rate-limiter for Rust", lists `flux-limiter` in its **dev-dependencies**. Its benchmark suite imports five rate-limiter crates (mine alongside `governor`, `tokio-rate-limit`, `ratelimit`, and `leaky-bucket`) and races them head-to-head. My download spike was cargo fetching my crate so their benchmarks could beat it. My crate's biggest fan is their CI pipeline.  
And in their published comparison table, mine is the slowest keyed column: 201.96 ns/op against their 46.94. Not a "what not to do" example, exactly. Merely the slowest column. For perspective: `governor`, the de-facto standard for Rust rate limiting, also "loses" to them in their own table. I'm in good company. And still last.

## Auditing my own code (and the README lie)

Nothing motivates a code audit quite like finding out your library is slow in public. And the sting had a history: months earlier, in the 0.8.2 release, I had already scrubbed performance claims from that same README ("high-performance," "zero-allocation hot path") after my changelog admitted that *no in-repo benchmarks existed to back them up*. But the scrub was incomplete. The first line of the README still read:  
*"A simple rate limiter based on the Generic Cell Rate Algorithm (GCRA) with nanosecond precision and lock-free concurrent access."*  
That word, **lock-free**, had survived my own audit. Under the hood, flux-limiter stores client state in a DashMap\<T, u64\>. DashMap is fantastic and spares you from wrapping a standard HashMap in a single giant Mutex, but it is **not lock-free**; it uses sharded RwLocks. The one claim my earlier self-cleanup missed was exactly the one a stranger's benchmark would eventually expose. The classic trap of letting an overly enthusiastic LLM help draft your README isn't that it lies constantly. It's that it lies plausibly enough to survive a partial audit.

This time I did two things differently: I audited *with* AI assistance instead of taking its word, and I cross-checked that audit against a second assistant and against measurements of my own. Between that work, the reasons behind the poor benchmark performance jumped right off the screen. The biggest one hadn't come from the AI audit at all:

> 0. The clock was half my hot path. Every call to `check_request()` starts with `SystemTime::now()`, a syscall, before touching any client state. Measured in isolation on my machine: ~28 ns, or **57% of the total cost** of a check that benchmarks at ~49 ns. The AI audit found four real issues in my locking and math, and never once mentioned the clock. Two AIs, two partial views. Verification isn't only for catching fabrications. It's for catching omissions.

The four issues the audit did find:

> 1. Holding the shard lock while doing floating-point math. To prevent Time-Of-Check to Time-Of-Use (TOCTOU) race conditions, I used DashMap::entry(). That's correct for thread safety, since it acquires an exclusive write lock on the shard. However, I constructed the entire FluxLimiterDecision return struct, including helper calls doing floating-point math, *inside* the match arms. Every thread hitting that shard was stuck waiting while the lock holder calculated HTTP header metadata. (Worth being precise about magnitude: single-digit nanoseconds, and it only matters under contention. But under contention is exactly where the benchmark lived.)  
> 2. Redundant f64 division on the hot path. To calculate remaining burst capacity on every allowed request, my helper method converted nanoseconds to seconds by dividing by 1\_000\_000\_000.0, and then immediately multiplied by the rate, which itself was calculated on the fly by dividing 1\_000\_000\_000.0 by rate\_nanos. I was dividing by a billion and multiplying by a billion on every single request.  
> 3. The key allocation tax. Because DashMap::entry(client\_id) takes ownership of the key in case it needs to insert a new entry, calling check\_request with a String forces a heap allocation (.clone()) on every single check, even when the client is already in the map 99.9% of the time. Measured: the ~34 ns gap between my u64-key and String-key benchmarks is almost entirely this clone.  
> 4. Reads that take write locks. Even for existing clients, DashMap::entry() takes the exclusive write lock on that shard. For a rate limiter, where the overwhelmingly common case is an *existing* client, the state could live in an AtomicU64 updated with a compare-and-swap under a cheap read lock, reserving the write lock for first insertion. This is, almost detail for detail, the architecture of the crate that beat me. My audit wasn't only finding bugs; it had reverse-engineered the winner's design.

## Enter criterion: establishing a baseline

If the original goal of flux-limiter was understanding things from first principles without hiding behind complexity, the next step was obvious: stop guessing, learn how to benchmark properly in Rust, and measure the bottlenecks myself.  
So that's what I did this morning: added criterion (yes, the same morning I learned what it is), wired up my first benchmark suite to test three scenarios (a single thread using a cheap u64 key, a single thread cloning a String key, and 4 threads hammering the same client ID concurrently), pressed `cargo bench` for the first time in my life, and got real numbers:

> * Single thread, u64 key: \~49.1 ns per check  
> * Single thread, String clone: \~83.4 ns per check (a \~34 ns gap, the clone tax, visible on day one)  
> * 4 threads contended, 1,000 checks: \~272 µs total (\~272 ns per check, over 5x slower per request than single-threaded, courtesy of shard lock contention)

A note on honesty in benchmarks: the crate that beat me reports its headline numbers as wall-clock time divided by total operations across 256 Tokio tasks on 16 worker threads. That's throughput, perfectly legitimate, but not the same quantity as single-core latency. When I eventually re-run their harness for the rematch, I'll report their methodology on their terms and mine on mine, and label which is which. I scrubbed unverifiable claims from my README once already; I'm not about to start making new ones.

## What's next

The beauty of keeping the codebase simple and readable is that fixing it doesn't require untangling twenty layers of trait abstractions. In the next part of this series, I'm going to walk through the fixes step by step in ROI order: a borrowed-key API to kill the clone tax, a cheap monotonic clock to replace the syscall, precomputed constants instead of per-request division, and hoisting the decision math out of the shard lock. Each step gets before/after criterion numbers. Then the AtomicU64 design, which is less a patch than a first step toward the architecture the winner uses. And finally the rematch, with honest numbers, including whatever gap remains. Some of it is architecture, and architecture doesn't close for free.

One more confession before the fixes: until this morning, "Criterion, anyone?" was a joke I didn't fully get. I didn't know what criterion was, had never run `cargo bench`, and didn't know my own crate had never been measured. That's the real running-before-you-walking: not writing bad code, but shipping for a year without ever once asking "how fast is it, and where does the time go?" The tooling was one `cargo add criterion` away the entire time.
