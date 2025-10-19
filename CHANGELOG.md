# Changelog

## [0.5.0] - 2024-10-20

### New Features

- **Added `f64` and `f32` support** for fractional duration creation
  - Example: `0.5.seconds()` instead of `500.milliseconds()`
  - All float methods leverage `Duration::from_secs_f64()` and `Duration::from_secs_f32()`
  - Automatically panics on NaN, infinity, and negative values (handled by `std::time::Duration`)
  - Thanks to @oconnor663 (blake3 maintainer) for the suggestion!

### Examples
```rust
use duration_extender::DurationExt;

let half_sec = 0.5.seconds();           // 500 milliseconds
let two_and_half_min = 2.5.minutes();   // 150 seconds
let quarter_hour = 0.25.hours();        // 900 seconds
```

---

## [0.4.0] - 2024-10-19

### Breaking Changes

- **BREAKING**: Removed support for `.days()` and `.weeks()` methods.
  - These methods were removed to prevent potential errors related to calendar and time zone irregularities (like Daylight Saving Time) when calculating fixed system-level durations.
  - The maximum supported time unit for fluent creation is now **hours**.
  - To represent fixed long durations, use the equivalent number of hours, e.g., `(7 * 24).hours()` instead of `7.days()`.

---

## [0.3.0] - 2025-10-19

### Breaking Changes

- **BREAKING**: Integer overflow during duration creation now **panics** for all types instead of silently saturating.
  - Previously: Large values (e.g., `u64::MAX.minutes()`) would saturate to `u64::MAX` seconds.
  - Now: Panics with descriptive error messages like:
    `"duration value 3074457345618258602 minutes overflows u64 seconds capacity"`.
  - Signed integers (`i32`, `i64`) continue to **panic on negative values**.
  - Aligns with Rust's philosophy of explicit failure on invalid input.

### Why this change?

Previously, overflow was silently handled, which could hide bugs. Panicking makes failures explicit and safer for production code.

---

## [0.2.0] - 2025-10-19

### Breaking Changes

- **BREAKING**: Negative values for `i32` and `i64` now panic with clear error messages instead of silently using absolute value.
  - Previously: `(-5).minutes()` would equal `5.minutes()` (surprising!)
  - Now: `(-5).minutes()` panics with "duration cannot be negative: got -5 minutes".

### Why this change?

Panicking on negative values prevents silent bugs and ensures explicit, safe behavior.

---

## [0.1.0] - 2025-10-19

- Initial release
- Fluent API for creating `std::time::Duration`
- Support for u64, u32, i64, i32
- Zero dependencies
- Saturating arithmetic for overflow safety