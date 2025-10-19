# duration-extender

[![Crates.io](https://img.shields.io/crates/v/duration-extender.svg)](https://crates.io/crates/duration-extender)
[![Documentation](https://docs.rs/duration-extender/badge.svg)](https://docs.rs/duration-extender)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A lightweight, zero-dependency Rust crate that extends primitive integer types with intuitive methods for creating `std::time::Duration` objects.

Write expressive, human-readable code for timeouts, delays, and schedules without the verbosity of `Duration::from_secs()`.

## ⚠️ Breaking Changes in v0.4.0

- `.days()` and `.weeks()` methods are **removed**.
  ```rust
  // ❌ Not allowed anymore
  // let week = 1.weeks();

  // ✅ Use hours instead
  let week = (7 * 24).hours();
  ```

## ⚠️ Breaking Changes in v0.3.0

**Overflow values now panic for all integer types**, in addition to negative values panicking for signed integers.

Previously in v0.2.0:
```rust
(-5).minutes() // ❌ Panics
u64::MAX.minutes() // ❌ Panics
```

Now in v0.3.0:
```rust
5.minutes()    // ✅ Works fine
(-5).minutes() // ❌ Panics: "duration cannot be negative: got -5 minutes"
u64::MAX.minutes() // ❌ Panics: "duration value ... overflows u64 seconds capacity"
```

This makes the behavior fully explicit and safe. See [CHANGELOG.md](CHANGELOG.md) for full details.

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
```

## Features

- **Fluent API** — Natural, readable syntax for duration creation.
- **Type-safe** — Works with `u64`, `u32`, `i64`, and `i32`.
- **Explicit errors** — Panics on overflow and negative values with clear messages.
- **Zero dependencies** — Only the standard library.
- **Minimal overhead** — Compiles down to the same code as manual duration creation.

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
duration-extender = "0.4"
```

## Usage

Import the `DurationExt` trait to unlock duration methods on integers:

```rust
use duration_extender::DurationExt;
use std::time::Duration;

fn main() {
    let timeout = 10.seconds();
    let delay = 5.minutes();

    let total_time = 2.hours() + 30.minutes() + 15.seconds();

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

| Method | Equivalent |
|--------|------------|
| `.seconds()` | `Duration::from_secs(n)` |
| `.minutes()` | `Duration::from_secs(n * 60)` |
| `.hours()` | `Duration::from_secs(n * 3600)` |
| `.milliseconds()` | `Duration::from_millis(n)` |
| `.microseconds()` | `Duration::from_micros(n)` |
| `.nanoseconds()` | `Duration::from_nanos(n)` |

## Supported Types

The `DurationExt` trait is implemented for:

- `u64` and `u32` — Direct conversion.
- `i64` and `i32` — Panics on negative values to prevent bugs.

All operations now use **checked arithmetic** to prevent silent overflow.

## Safety Guarantees

- **Overflow checked** — Panics on overflow with a clear message.
- **Negative handling** — Signed integers panic on negative values with clear error messages.
- **Type safety** — Uses Rust's strong type system for compile-time correctness.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Dual-licensed under:
- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

Choose whichever license suits your needs.

## Acknowledgments

Inspired by ergonomic duration APIs in other ecosystems and refined by community feedback.
