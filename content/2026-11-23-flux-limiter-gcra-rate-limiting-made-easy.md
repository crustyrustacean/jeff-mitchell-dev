+++
title = "Flux Limiter: GCRA Rate Limiting Made Easy"
date = 2026-11-23
description = "A post about the GCRA based rate limiter I made in Rust"
categories = ["Projects"]
tags = ["rust", "rate-limiting", "gcra"]
draft = true
+++

Rate limiting is like being a bouncer at a popular nightclub – you need to control how many people can enter and when, without causing a riot outside. In the world of web APIs and services, rate limiting serves a similar purpose: protecting your servers from being overwhelmed while providing fair access to resources.

<!-- more -->

# Building a High-Performance Rate Limiter in Rust: A Deep Dive into GCRA

Today, I want to share my journey building **flux-limiter**, a high-performance rate limiting library in Rust that uses the Generic Cell Rate Algorithm (GCRA). Whether you're a seasoned developer or just getting started with rate limiting, this post will walk you through the concepts and implementation details.

# Why Did I Build this Thing?

The simple fact is, I was looking at other rate limiters, namely `governor` and it's associated `tower-governor` incarnation for Axum, and had a great deal of trouble understanding them. I found myself longing for something simple that made sense, based on clear criteria. A challenge I've found in using Rust libraries is the *level of abstraction*. It's really challenging, at first glance, for a beginner to understand *how* to use them because they assume you think quickly in terms of abstraction.

I need to be hit in the face.

I figured *maybe* others might need that as well.

## What is Rate Limiting and Why Do We Need It?

Imagine you're running a popular API that provides weather data. Without any controls, a single client could make thousands of requests per second, potentially:

- **Overwhelming your servers** and causing downtime for everyone
- **Driving up your infrastructure costs** unnecessarily  
- **Creating an unfair experience** where one heavy user monopolizes resources

Rate limiting solves these problems by setting boundaries: "You can make X requests per time period, but no more." It's like having a speed limit on the highway – it keeps traffic flowing smoothly for everyone.

## The Rate Limiting Algorithm Landscape

Before diving into implementation, let's explore the main approaches to rate limiting:

### 1. Token Bucket
Think of this like a bucket that gets filled with tokens at a steady rate. Each request "costs" a token. If the bucket is empty, requests are denied. The bucket has a maximum capacity, allowing for bursts of activity.

**Pros**: Intuitive, allows bursts, mathematically well-defined  
**Cons**: Requires background token refill process, potential precision issues

### 2. Leaky Bucket
Imagine a bucket with a hole in the bottom that leaks at a constant rate. Requests are added to the bucket, and if it overflows, they're rejected. This smooths out traffic to a constant rate.

**Pros**: Perfect for smoothing bursty traffic  
**Cons**: Can be overly restrictive, doesn't handle natural burst patterns well

### 3. Fixed Window Counter
Count requests in fixed time windows (e.g., per minute). Simple but can have edge cases where clients get twice the limit at window boundaries.

**Pros**: Dead simple to implement  
**Cons**: Vulnerable to boundary attacks, uneven request distribution

### 4. Sliding Window
A more sophisticated version that tracks requests over a rolling time window, providing smoother rate limiting without boundary issues.

**Pros**: More accurate than fixed windows  
**Cons**: Complex to implement efficiently, requires more memory

### 5. Generic Cell Rate Algorithm (GCRA)
This is where things get interesting – and where our flux-limiter comes in.

## Enter GCRA: The Elegant Solution

The Generic Cell Rate Algorithm might sound intimidating, but it's actually quite elegant. Instead of tracking tokens like the token bucket algorithm, GCRA tracks **when the next request should be allowed**. I found this *seriously* simple to understand.

Here's the key insight: GCRA is mathematically equivalent to the token bucket algorithm, but uses a different approach:

- **Token Bucket**: "How many tokens do I have right now?"
- **GCRA**: "When is the theoretical arrival time (TAT) for the next request?"

### Why GCRA is Awesome

1. **No Background Processes**: Unlike token bucket, there's no need to continuously refill tokens
2. **Precise Timing**: Works with exact timestamps, avoiding floating-point precision issues
3. **Efficient State**: Only needs to store one timestamp per client
4. **Mathematically Sound**: Equivalent to token bucket but often simpler to reason about

The algorithm boils down to:
- If current time ≥ (stored TAT - tolerance), allow the request
- Update TAT = max(current time, previous TAT) + increment
- If denied, calculate retry time

## Meet flux-limiter: GCRA in Rust

Now let's dive into the implementation! I built flux-limiter to be production-ready with these goals:

- **High Performance**: Lock-free concurrent access
- **Precise Timing**: Nanosecond precision for accurate rate limiting  
- **Generic**: Work with any client ID type (strings, IPs, user IDs)
- **Rich Metadata**: Provide detailed information for HTTP responses
- **Memory Efficient**: Automatic cleanup of stale client data

### Architecture Overview

The codebase is organized into several clean modules:

```rust
src/
├── lib.rs              // Public API exports
├── flux_limiter.rs     // Core rate limiter logic  
├── config.rs           // Configuration types
├── clock.rs            // Time abstraction
└── errors.rs           // Error handling
```

### The Clock Abstraction

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

### Configuration Made Simple

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

### The Core Algorithm Implementation

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

Let's break down the key concepts:

- **`rate_nanos`**: Time between requests (1/rate converted to nanoseconds)
- **`tolerance_nanos`**: How much burst we allow (burst_capacity × rate_nanos)
- **`previous_tat_nanos`**: When this client's next request should theoretically arrive
- **`is_conforming`**: The GCRA test - are we within the tolerance window?

### Concurrent Access with DashMap

For high-performance concurrent access, I chose `DashMap` – a blazingly fast concurrent HashMap:

```rust
pub client_state: Arc<DashMap<T, u64>>,
```

DashMap provides:
- **Lock-free reads** in most cases
- **Fine-grained locking** for writes
- **Excellent performance** under high concurrency

### Rich Decision Metadata

Instead of just returning "allow/deny", flux-limiter provides rich metadata perfect for HTTP responses:

```rust
pub struct FluxLimiterDecision {
    pub allowed: bool,                    // Allow or deny
    pub retry_after_seconds: Option<f64>, // When to retry (if denied)
    pub remaining_capacity: Option<f64>,  // How much burst is left
    pub reset_time_nanos: u64,           // When the window resets
}
```

This makes it trivial to add proper HTTP headers:

```rust
if let Some(retry_after) = decision.retry_after_seconds {
    headers.insert("Retry-After", retry_after.to_string());
}
if let Some(remaining) = decision.remaining_capacity {
    headers.insert("X-RateLimit-Remaining", remaining.to_string());
}
```

### Memory Management

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

## Testing: The Secret Sauce

One of my favorite aspects of this implementation is how testable it is. The `Clock` abstraction makes deterministic testing trivial:

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

## Real-World Usage

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

## Performance Characteristics

flux-limiter is designed for high-throughput scenarios:

- **Memory**: O(active clients) - only stores one timestamp per client
- **Time Complexity**: O(1) for rate limiting decisions
- **Concurrency**: Lock-free reads, minimal contention on writes
- **Precision**: Nanosecond timing for exact rate limiting
- **Throughput**: Capable of millions of decisions per second

## Key Takeaways

Building flux-limiter taught me several important lessons:

1. **GCRA is underrated** – It's simpler than token bucket in many ways while being mathematically equivalent
2. **Abstraction matters** – The Clock trait made testing infinitely easier
3. **Rich metadata is valuable** – Don't just return boolean decisions; provide context
4. **Rust's type system is your friend** – Generic client IDs and compile-time safety prevented many bugs
5. **Performance and correctness can coexist** – Lock-free concurrency with nanosecond precision

## What's Next?

flux-limiter demonstrates that you can build high-performance, mathematically sound rate limiting without complexity. The GCRA algorithm provides an elegant foundation, and Rust's zero-cost abstractions let us build something both fast and safe.

If you're interested in the full implementation, check out the [repository](https://github.com/crustyrustacean/flux-limiter). The code is MIT licensed and production-ready.

Rate limiting doesn't have to be complicated – sometimes the most elegant solution is also the most effective.

That's not to say there isn't work still left to do. The cleanup system is perhaps overly simplistic, it could stand for some more nuanced refinement. Also, there is no quota strategy for replenishment, as the `governor` crate has. You have to start somewhere though, so there you have it!

---

*Have you implemented rate limiting in your projects? What algorithms have you used? I'd love to hear about your experiences in the comments below!*
