+++
title = "Rama 0.3.0 is Live"
date = 2026-07-07
description = "Today marks the 0.3.0 release of Rama"
categories = ["Releases"]
tags = ["rust", "rama", "proxy", "networking", "web-framework", "http", "release", "async"]
draft = false
+++

Today is an important day for the Rust community.

The crew behind [Rama](https://ramaproxy.org) announced the release of [Rama 0.3.0](https://plabayo.tech/blog/rama-0-3?tag=Rama), a significant update to the async Rust framework for building proxies, load balancers, and API gateways. This release brings enhanced performance, improved developer experience, and several new features that make building network services even more straightforward.

## What's New in 0.3.0

### Enhanced Middleware System

The middleware system has been redesigned for better composability and performance. You can now chain middleware more intuitively, and the new `Layer` trait provides cleaner separation of concerns. This makes it easier to add authentication, logging, and rate limiting to your services.

### Improved TLS Support

TLS handling has been streamlined with better certificate management and SNI support. The new `TlsConfig` builder pattern simplifies configuration, whether you're terminating TLS at the proxy or passing it through upstream.

### Better Load Balancing

New load balancing algorithms have been added, including least connections and weighted round-robin. The health checking system is now more robust, with configurable retry logic and circuit breaker patterns built-in.

### Performance Improvements

Benchmarks show 15-20% improvement in request throughput compared to 0.2.x, primarily from optimizations in the connection pooling and request routing code paths. Memory usage has also been reduced through better buffer management.

## Migration Guide

Most applications will require minimal changes to upgrade. The main breaking change is the middleware API, which now uses the new `Layer` trait instead of the previous `Transform` pattern. Check the [migration guide](https://docs.rs/rama/0.3.0/rama/migration.html) for specific steps.

## What's Next

The Rama team is already working on 0.4.0, which will focus on observability improvements with built-in metrics, tracing, and logging integrations. We're also exploring WebAssembly support for plugin architectures.

## Get Started

Update your `Cargo.toml`:

```toml
[dependencies]
rama = "0.3.0"
```

Check out the [documentation](https://docs.rs/rama) and [examples](https://github.com/plabayo/rama/tree/main/examples) to see what's possible with Rama 0.3.0.

Thanks to all the contributors who made this release possible. If you're building network services in Rust, we'd love to hear about your experience with Rama.