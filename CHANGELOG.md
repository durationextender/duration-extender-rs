# Changelog

## [0.2.0] - 2024-10-20

### Breaking Changes

- **BREAKING**: Negative values for `i32` and `i64` now panic with clear error messages instead of silently using absolute value
  - Previously: `(-5).minutes()` would equal `5.minutes()` (surprising!)
  - Now: `(-5).minutes()` panics with "duration cannot be negative: got -5 minutes"
  - This prevents silent bugs where negative values were unintentionally converted
  - Thanks to @nik-rev, @juicedatom, and @burntsushi for the feedback!

### Why this change?

The previous behavior of silently taking the absolute value was confusing and could hide bugs, especially in domains like robotics where signed time arithmetic is common. Panicking makes the behavior explicit and catches errors early.

If you need to handle potentially negative values, check the sign before calling these methods:
```rust
let value: i64 = get_some_value();
let duration = if value >= 0 {
    value.minutes()
} else {
    panic!("Unexpected negative duration");
};
```

## [0.1.0] - 2024-10-19

- Initial release
- Fluent API for creating `std::time::Duration`
- Support for u64, u32, i64, i32
- Zero dependencies
- Saturating arithmetic for overflow safety