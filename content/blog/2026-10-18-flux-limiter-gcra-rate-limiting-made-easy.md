+++
title = "Flux Limiter: GCRA Rate Limiting Made Easy"
date = 2026-10-18
description = "The announcement post I wrote for flux-limiter a year ago and never published, shown essentially as-written — overconfident claims, race-condition code samples, and all. Part 0 of the re-opening the hood series."
categories = ["Projects"]
tags = ["rust", "flux-limiter", "rate-limiting", "gcra"]
draft = false
aliases = ["/2026-10-18-flux-limiter-gcra-rate-limiting-made-easy"]
+++

> **A note before you read.** I wrote this post about a year ago, shortly after first publishing flux-limiter, and never posted it. I'm publishing it now essentially as-written: the claims and the code samples are untouched. The AI-assisted prose got a light trim, removing the worst of the filler, but nothing of substance changed (and I've added three bracketed notes where I couldn't resist). It reads very differently in hindsight. The code snippets below show the pre-0.8.0 implementation, including at least one race condition I hadn't found yet. The performance claims were never measured — there were no benchmarks at all. This is Part 0 of a short series, *Re-opening the Hood on flux-limiter*. Part 1, coming later this week, audits everything wrong here.

Rate limiting is like being a bouncer at a popular nightclub – you need to control how many people can enter at a time, and when, without causing a riot outside. In the world of web APIs and services, rate limiting serves a similar purpose: protecting your servers from being overwhelmed while providing fair access to resources.

<!-- more -->

This post is about **flux-limiter**, a rate limiting library I wrote in Rust using the Generic Cell Rate Algorithm (GCRA). I'll cover why I built it, how GCRA works, and how the code fits together.

## Why did I build this thing?

The simple fact is, I was looking at other rate limiters, namely `governor` and its associated `tower-governor` incarnation for Axum, and had a great deal of trouble understanding them. I found myself longing for something simple that made sense, based on clear criteria. A challenge I've found in using Rust libraries is the *level of abstraction*. It's really challenging, at first glance, for a beginner to understand *how* to use them because they assume you think quickly in terms of abstraction.

I need to be hit in the face.

I figured *maybe* others might need that as well.

## What is rate limiting, and why do we need it?

Imagine you're running a popular API that provides weather data. Without any controls, a single client could make thousands of requests per second, potentially:

- **Overwhelming your servers** and causing downtime for everyone
- **Driving up your infrastructure costs** unnecessarily  
- **Creating an unfair experience** where one heavy user monopolizes resources

Rate limiting solves these problems by setting boundaries: "You can make X requests per time period, but no more." It's like having a speed limit on the highway, it keeps traffic flowing smoothly.

## The rate limiting algorithm landscape

There are five common approaches:

### 1. Token bucket
Think of this like a bucket that gets filled with tokens at a steady rate. Each request "costs" a token. If the bucket is empty, requests are denied. The bucket has a maximum capacity, allowing for bursts of activity.

**Pros**: Intuitive, allows bursts, mathematically well-defined  
**Cons**: Requires background token refill process, potential precision issues

### 2. Leaky bucket
Imagine a bucket with a hole in the bottom that leaks at a constant rate. Each request adds water; whatever doesn't fit spills over and is dropped. The output is a steady, constant rate.

**Pros**: Perfect for smoothing bursty traffic  
**Cons**: Can be overly restrictive, doesn't handle natural burst patterns well

### 3. Fixed window counter
Count requests in fixed time windows (e.g., per minute). Simple but can have edge cases where clients get twice the limit at window boundaries.

**Pros**: Dead simple to implement  
**Cons**: Vulnerable to boundary attacks, uneven request distribution

### 4. Sliding window
A more sophisticated version that tracks requests over a rolling time window, providing smoother rate limiting without boundary issues.

**Pros**: More accurate than fixed windows  
**Cons**: Complex to implement efficiently, requires more memory

### 5. Generic Cell Rate Algorithm (GCRA)
This is where things get interesting – and where our flux-limiter comes in.

## Enter GCRA

The GCRA might sound intimidating, but it's actually quite elegant. Instead of tracking tokens like the token bucket algorithm, GCRA tracks **when the next request should be allowed**. I found this *seriously* simple to understand.

*[Note from present-day me: simple to understand, in principle at least. I still have trouble running it on paper, but I digress...]*

Here's the key insight: GCRA is mathematically equivalent to the token bucket algorithm, but uses a different approach:

- **Token Bucket**: "How many tokens do I have right now?"
- **GCRA**: "When is the theoretical arrival time (TAT) for the next request?"

### Why GCRA is awesome

1. **No Background Processes**: Unlike token bucket, there's no need to continuously refill tokens
2. **Precise Timing**: Works with exact timestamps, avoiding floating-point precision issues
3. **Efficient State**: Only needs to store one timestamp per client
4. **Mathematically Sound**: Equivalent to token bucket but often simpler to reason about

The algorithm boils down to:
- If current time ≥ (stored TAT - tolerance), allow the request
- Update TAT = max(current time, previous TAT) + increment
- If denied, calculate retry time

## Meet flux-limiter: GCRA in Rust

On to the code. I built flux-limiter to be production-ready with these goals:

- **High Performance**: Lock-free concurrent access
- **Precise Timing**: Nanosecond precision for accurate rate limiting  
- **Generic**: Work with any client ID type (strings, IPs, user IDs)
- **Rich Metadata**: Provide detailed information for HTTP responses
- **Memory Efficient**: Automatic cleanup of stale client data

### Architecture overview

The codebase is organized into several clean modules:

```bash
src/
├── lib.rs              // Public API exports
├── flux_limiter.rs     // Core rate limiter logic  
├── config.rs           // Configuration types
├── clock.rs            // Time abstraction
└── errors.rs           // Error handling
```

### The `Clock` abstraction

One of my favorite design decisions was abstracting time through a `Clock` trait:

```rust
pub trait Clock: Send + Sync {
    fn now(&self) -> u64;
}
```

This simple abstraction provides huge benefits:
- **Testability**: Inject controllable time for deterministic tests
- **Flexibility**: Swap in different time sources if needed
- **Thread Safety**: Built-in Send + Sync requirements

The `SystemClock` implementation is straightforward:

```rust
impl Clock for SystemClock {
    fn now(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock went backwards")
            .as_nanos() as u64
    }
}
```

### Configuration

Rate limiter configuration uses a builder pattern that's both flexible and type-safe:

```rust
let config = FluxLimiterConfig::new(10.0, 5.0)  // 10 req/sec, burst of 5
    .rate(20.0)    // Change to 20 req/sec
    .burst(10.0);  // Allow bursts of 10
```

The config validates itself, catching invalid parameters early:

```rust
pub fn validate(&self) -> Result<(), FluxLimiterError> {
    if self.rate_per_second <= 0.0 {
        return Err(FluxLimiterError::InvalidRate);
    }
    if self.burst_capacity < 0.0 {
        return Err(FluxLimiterError::InvalidBurst);
    }
    Ok(())
}
```

### The core algorithm

Here's where the GCRA magic happens. The heart of the rate limiter lives in the `check_request` method:

```rust
pub fn check_request(&self, client_id: T) -> Result<FluxLimiterDecision, FluxLimiterError> {
    let current_time_nanos = self.clock.now();
    
    // Get the stored TAT for this client (or use current time if new)
    let previous_tat_nanos = self.client_state
        .get(&client_id)
        .map(|entry| *entry.value())
        .unwrap_or(current_time_nanos);

    // GCRA core logic: check if current time is within tolerance of TAT
    let is_conforming = current_time_nanos >= 
        previous_tat_nanos.saturating_sub(self.tolerance_nanos);

    if is_conforming {
        // Allow request and update TAT
        let new_tat_nanos = current_time_nanos.max(previous_tat_nanos) + self.rate_nanos;
        self.client_state.insert(client_id, new_tat_nanos);
        // ... return allowed decision with metadata
    } else {
        // Deny request and calculate retry time
        // ... return denied decision with retry info
    }
}
```

*[Note from present-day me: this snippet shows the original `get()`-then-`insert()` pattern. It has a Time of Check to Time of Use (TOCTOU) race condition — two threads can both read the same TAT and both be allowed. Fixed in 0.8.1 by switching to DashMap's `entry()` API for an atomic read-modify-write. I had no idea when I wrote this.]*

Let's break down the key concepts:

- **`rate_nanos`**: Time between requests (1/rate converted to nanoseconds)
- **`tolerance_nanos`**: How much burst we allow (burst_capacity × rate_nanos)
- **`previous_tat_nanos`**: When this client's next request should theoretically arrive
- **`is_conforming`**: The GCRA test - are we within the tolerance window?

### Concurrent access with DashMap

For high-performance concurrent access, I chose `DashMap` – a fast concurrent HashMap:

```rust
pub client_state: Arc<DashMap<T, u64>>,
```

DashMap provides:
- **Lock-free reads** in most cases
- **Fine-grained locking** for writes
- **Excellent performance** under high concurrency

### Rich decision metadata

Instead of just returning "allow/deny", flux-limiter provides rich metadata perfect for HTTP responses:

```rust
pub struct FluxLimiterDecision {
    pub allowed: bool,                    // Allow or deny
    pub retry_after_seconds: Option<f64>, // When to retry (if denied)
    pub remaining_capacity: Option<f64>,  // How much burst is left
    pub reset_time_nanos: u64,           // When the window resets
}
```

This makes it straightforward to add proper HTTP headers:

```rust
if let Some(retry_after) = decision.retry_after_seconds {
    headers.insert("Retry-After", retry_after.to_string());
}
if let Some(remaining) = decision.remaining_capacity {
    headers.insert("X-RateLimit-Remaining", remaining.to_string());
}
```

### Memory management

One challenge with per-client rate limiting is memory growth. Clients come and go, but their state lingers. flux-limiter includes a cleanup mechanism:

```rust
pub fn cleanup_stale_clients(&self, max_stale_nanos: u64) {
    let current_time_nanos = self.clock.now();
    self.client_state.retain(|_, &mut tat| {
        tat + self.tolerance_nanos > 
            current_time_nanos.saturating_sub(max_stale_nanos)
    });
}
```

Call this periodically to remove clients that haven't been seen recently.

## Testing

One of my favorite aspects of this implementation is how testable it is. The `Clock` abstraction makes deterministic testing straightforward:

```rust
#[test]
fn rate_limiting_blocks_rapid_requests() {
    let clock = TestClock::new(0.0);
    let config = FluxLimiterConfig::new(1.0, 0.0); // 1 req/sec, no burst
    let limiter = FluxLimiter::with_config(config, clock.clone()).unwrap();
    
    // First request at t=0 should be allowed
    assert!(limiter.check_request("client1").unwrap().allowed);
    
    // Second request immediately should be blocked  
    assert!(!limiter.check_request("client1").unwrap().allowed);
    
    // Request at exactly 1 second later should be allowed
    clock.set_time(1.0);
    assert!(limiter.check_request("client1").unwrap().allowed);
}
```

The `TestClock` lets us control time completely, making tests fast and deterministic.

## Real-world usage

Using flux-limiter in your application is straightforward:

```rust
use flux_limiter::{FluxLimiter, FluxLimiterConfig, SystemClock};

// Create a rate limiter: 100 req/sec with burst of 50
let config = FluxLimiterConfig::new(100.0, 50.0);
let limiter = FluxLimiter::with_config(config, SystemClock)?;

// In your request handler:
match limiter.check_request(client_id) {
    Ok(decision) if decision.allowed => {
        // Process the request
        handle_request().await
    }
    Ok(decision) => {
        // Rate limited - return 429 with retry info
        let retry_after = decision.retry_after_seconds.unwrap_or(1.0);
        Err(TooManyRequests { retry_after })
    }
    Err(_) => Err(InternalError)
}
```

## Performance characteristics

*[Note from present-day me: keep reading, but know that none of the numbers implied below had ever been measured when I wrote this. There were no benchmarks — I wouldn't add `criterion` to the crate for another year, and the "lock-free reads" claim about DashMap is simply wrong. Everything in this section gets audited, with real numbers, in the next post.]*

flux-limiter is designed for high-throughput scenarios:

- **Memory**: O(active clients) - only stores one timestamp per client
- **Time Complexity**: O(1) for rate limiting decisions
- **Concurrency**: Lock-free reads, minimal contention on writes
- **Precision**: Nanosecond timing for exact rate limiting
- **Throughput**: Capable of millions of decisions per second

## Key takeaways

Building flux-limiter taught me several important lessons:

1. **GCRA is underrated** – It's simpler than token bucket in many ways while being mathematically equivalent
2. **Abstraction matters** – The Clock trait made testing infinitely easier
3. **Rich metadata is valuable** – Don't just return boolean decisions; provide context
4. **Rust's type system is your friend** – Generic client IDs and compile-time safety prevented many bugs
5. **Performance and correctness can coexist** – Lock-free concurrency with nanosecond precision

## What's next?

The full implementation is in the [repository](https://github.com/crustyrustacean/flux-limiter). The code is MIT licensed and production-ready.

That's not to say there isn't work still left to do. The cleanup system is perhaps overly simplistic, it could stand for some more nuanced refinement. Also, there is no quota strategy for replenishment, as the `governor` crate has. You have to start somewhere though, so there you have it!

---

*Have you implemented rate limiting in your projects? What algorithms have you used? I'd love to hear about your experiences in the comments below!*
