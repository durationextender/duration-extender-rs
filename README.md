# duration-extender

[![Crates.io](https://img.shields.io/crates/v/duration-extender.svg)](https://crates.io/crates/duration-extender)
[![Documentation](https://docs.rs/duration-extender/badge.svg)](https://docs.rs/duration-extender)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A lightweight, zero-dependency Rust crate that extends primitive integer types with intuitive methods for creating `std::time::Duration` objects.

Write expressive, human-readable code for timeouts, delays, and schedules without the verbosity of `Duration::from_secs()`.

## Why duration-extender?

## ⚠️ Breaking Changes in v0.2.0

**Negative values now panic instead of using absolute value.**

Previously in v0.1.0:
```rust
(-5).minutes() == 5.minutes() // ❌ Surprising - used abs()
```

Now in v0.2.0:
```rust
5.minutes()    // ✅ Works fine
(-5).minutes() // ❌ Panics: "duration cannot be negative: got -5 minutes"
```

This change prevents silent bugs. See [CHANGELOG.md](CHANGELOG.md) for details.

---

**Before:**
```rust
let timeout = Duration::from_secs(30);
let delay = Duration::from_secs(5 * 60);
let cache_ttl = Duration::from_secs(24 * 60 * 60);
```

**After:**
```rust
let timeout = 30.seconds();
let delay = 5.minutes();
let cache_ttl = 1.days();
```

## Features

- **Fluent API** — Natural, readable syntax for duration creation
- **Type-safe** — Works with `u64`, `u32`, `i64`, and `i32`
- **Explicit errors** — Uses saturating arithmetic for overflow; panics on negative values with clear messages
- **Zero dependencies** — Just the standard library
- **Minimal overhead** — Compiles down to the same code as manual duration creation

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
duration-extender = "0.2"  # ← Change from 0.1
```

## Usage

Import the `DurationExt` trait to unlock duration methods on integers:

```rust
use duration_extender::DurationExt;
use std::time::Duration;

fn main() {
    // Create durations with clear, readable syntax
    let timeout = 10.seconds();
    let delay = 5.minutes();
    let long_wait = 2.days();
    
    // Combine durations naturally
    let total_time = 2.hours() + 30.minutes() + 15.seconds();
    
    // Works with variables
    let retry_count = 3;
    let backoff = retry_count.seconds();
    
    // Signed integers must be non-negative (panics on negative)
    let elapsed = 100.seconds(); // ✅ Works
    // let bad = (-100).seconds(); // ❌ Panics!
}
```

### Real-World Examples

**HTTP client timeout:**
```rust
let client = reqwest::Client::builder()
    .timeout(30.seconds())
    .build()?;
```

**Tokio sleep:**
```rust
tokio::time::sleep(2.seconds()).await;
```

**Cache expiration:**
```rust
cache.insert_with_ttl("key", value, 24.hours());
```

**Rate limiting:**
```rust
let rate_limit = RateLimit::new(100, 1.minutes());
```

## Available Methods

The `DurationExt` trait provides these methods:

| Method | Equivalent |
|--------|------------|
| `.seconds()` | `Duration::from_secs(n)` |
| `.minutes()` | `Duration::from_secs(n * 60)` |
| `.hours()` | `Duration::from_secs(n * 3600)` |
| `.days()` | `Duration::from_secs(n * 86400)` |
| `.weeks()` | `Duration::from_secs(n * 604800)` |

## Supported Types

The `DurationExt` trait is implemented for:

- `u64` and `u32` — Direct conversion
- `i64` and `i32` — Panics on negative values to prevent bugs

All operations use **saturating arithmetic** to prevent overflow panics.

## Safety Guarantees

- **Overflow safe** — Saturating arithmetic prevents overflow panics
- **Negative handling** — Signed integers panic on negative values with clear error messages
- **Type safety** — Leverages Rust's type system for compile-time correctness

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is dual-licensed under:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

You may choose either license for your purposes.

## Acknowledgments

Inspired by duration extension patterns from other languages and the Rust community's focus on ergonomic APIs.