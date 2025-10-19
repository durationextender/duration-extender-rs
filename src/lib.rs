use std::time::Duration;

/// An extension trait that adds fluent time unit methods to integer primitives,
/// allowing for highly readable time duration creation.
///
/// This crate is optimized for **system timing** (timeouts, sleeps, fixed cache TTLs).
/// It explicitly excludes methods for units longer than hours (days, weeks) 
/// to prevent calendar errors related to Daylight Saving Time (DST) and time zones.
///
/// # Panics
///
/// - Signed integers (`i32`, `i64`) **panic** if the value is negative.
/// - Overflow panics for `.minutes()` and `.hours()` when the resulting seconds exceed `u64::MAX`.
///
/// # Examples
///
/// ```rust
/// use duration_extender::DurationExt;
/// use std::time::Duration;
///
/// let timeout = 10.seconds();
/// let delay = 5.minutes();
/// 
/// // For a fixed 2-day duration, use the hour equivalent:
/// let fixed_long_wait = (2 * 24).hours();
///
/// let total_time = 2.hours() + 30.minutes() + 15.seconds();
///
/// // Signed integers must be non-negative
/// let elapsed = 100.seconds(); // ✅ Works
/// // let bad = (-100).seconds(); // ❌ Panics!
/// ```
pub trait DurationExt {
    /// Creates a `Duration` representing this many seconds.
    fn seconds(self) -> Duration;
    /// Creates a `Duration` representing this many minutes.
    fn minutes(self) -> Duration;
    /// Creates a `Duration` representing this many hours.
    fn hours(self) -> Duration;
    /// Creates a `Duration` representing this many milliseconds.
    fn milliseconds(self) -> Duration;
    /// Creates a `Duration` representing this many microseconds.
    fn microseconds(self) -> Duration;
    /// Creates a `Duration` representing this many nanoseconds.
    fn nanoseconds(self) -> Duration;
}

// ===== Implementation for unsigned integers =====
impl DurationExt for u64 {
    fn seconds(self) -> Duration {
        Duration::from_secs(self)
    }

    fn minutes(self) -> Duration {
        let secs = self.checked_mul(60)
            .expect(&format!("duration value {} minutes overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn hours(self) -> Duration {
        let secs = self.checked_mul(3600)
            .expect(&format!("duration value {} hours overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn milliseconds(self) -> Duration {
        Duration::from_millis(self)
    }

    fn microseconds(self) -> Duration {
        Duration::from_micros(self)
    }

    fn nanoseconds(self) -> Duration {
        Duration::from_nanos(self)
    }
}

impl DurationExt for u32 {
    fn seconds(self) -> Duration {
        Duration::from_secs(self as u64)
    }

    // Delegates to u64's implementation which includes the correct overflow check
    fn minutes(self) -> Duration {
        (self as u64).minutes()
    }

    // Delegates to u64's implementation which includes the correct overflow check
    fn hours(self) -> Duration {
        (self as u64).hours()
    }

    fn milliseconds(self) -> Duration {
        Duration::from_millis(self as u64)
    }

    fn microseconds(self) -> Duration {
        Duration::from_micros(self as u64)
    }

    fn nanoseconds(self) -> Duration {
        Duration::from_nanos(self as u64)
    }
}

// ===== Implementation for signed integers =====
impl DurationExt for i64 {
    fn seconds(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} seconds", self);
        Duration::from_secs(self as u64)
    }

    fn minutes(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} minutes", self);
        // Delegates to u64 for the multiplication and overflow check
        (self as u64).minutes()
    }

    fn hours(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} hours", self);
        // Delegates to u64 for the multiplication and overflow check
        (self as u64).hours()
    }

    fn milliseconds(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} milliseconds", self);
        Duration::from_millis(self as u64)
    }

    fn microseconds(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} microseconds", self);
        Duration::from_micros(self as u64)
    }

    fn nanoseconds(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} nanoseconds", self);
        Duration::from_nanos(self as u64)
    }
}

impl DurationExt for i32 {
    fn seconds(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} seconds", self);
        Duration::from_secs(self as u64)
    }

    fn minutes(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} minutes", self);
        // Delegates to u64 for the multiplication and overflow check
        (self as u64).minutes()
    }

    fn hours(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} hours", self);
        // Delegates to u64 for the multiplication and overflow check
        (self as u64).hours()
    }

    fn milliseconds(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} milliseconds", self);
        Duration::from_millis(self as u64)
    }

    fn microseconds(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} microseconds", self);
        Duration::from_micros(self as u64)
    }

    fn nanoseconds(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} nanoseconds", self);
        Duration::from_nanos(self as u64)
    }
}

// ===== Tests =====
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // Constants for calculating the exact overflow boundary of u64 in minutes/hours.
    const MAX_FOR_MINUTES: u64 = u64::MAX / 60;
    const MAX_FOR_HOURS: u64 = u64::MAX / 3600;

    const OVERFLOW_MINUTES: u64 = MAX_FOR_MINUTES + 1;
    const OVERFLOW_HOURS: u64 = MAX_FOR_HOURS + 1;

    // --- u64 Tests ---
    #[test]
    fn test_u64_large_units() {
        let five: u64 = 5;
        assert_eq!(five.minutes(), Duration::from_secs(5 * 60));
        assert_eq!(five.hours(), Duration::from_secs(5 * 3600));
    }

    #[test]
    #[should_panic(expected = "overflows u64 seconds capacity")]
    fn test_u64_minutes_panics_on_overflow() {
        let _ = OVERFLOW_MINUTES.minutes();
    }

    #[test]
    #[should_panic(expected = "overflows u64 seconds capacity")]
    fn test_u64_hours_panics_on_overflow() {
        let _ = OVERFLOW_HOURS.hours();
    }

    #[test]
    fn test_max_u64_small_units() {
        let max_u64 = u64::MAX;
        assert_eq!(max_u64.milliseconds(), Duration::from_millis(u64::MAX));
        assert_eq!(max_u64.microseconds(), Duration::from_micros(u64::MAX));
        assert_eq!(max_u64.nanoseconds(), Duration::from_nanos(u64::MAX));
    }

    // --- i64 Tests ---
    #[test]
    fn test_i64_positive() {
        let pos: i64 = 10;
        assert_eq!(pos.seconds(), Duration::from_secs(10));
        assert_eq!(pos.minutes(), Duration::from_secs(600));
        assert_eq!(pos.hours(), Duration::from_secs(36000));
    }

    #[test]
    #[should_panic(expected = "duration cannot be negative")]
    fn test_i64_negative_panics() {
        let neg: i64 = -10;
        let _ = neg.minutes();
    }

    #[test]
    #[should_panic(expected = "overflows u64 seconds capacity")]
    fn test_i64_minutes_panics_on_overflow() {
        let _ = (OVERFLOW_MINUTES as i64).minutes();
    }

    #[test]
    #[should_panic(expected = "overflows u64 seconds capacity")]
    fn test_i64_hours_panics_on_overflow() {
        let _ = (OVERFLOW_HOURS as i64).hours();
    }
    
    // --- i32/u32 Tests ---

    #[test]
    fn test_i32_positive_full() {
        let val: i32 = 5;
        assert_eq!(val.seconds(), Duration::from_secs(5));
        assert_eq!(val.minutes(), Duration::from_secs(300));
        assert_eq!(val.hours(), Duration::from_secs(18000));
    }

    #[test]
    #[should_panic(expected = "duration cannot be negative")]
    fn test_i32_negative_panics() { let _ = (-5).seconds(); }
    
    // u32 test to ensure delegation works for non-overflow cases
    #[test]
    fn test_u32_positive_full() {
        let val: u32 = 5;
        assert_eq!(val.seconds(), Duration::from_secs(5));
        assert_eq!(val.minutes(), Duration::from_secs(300));
        assert_eq!(val.hours(), Duration::from_secs(18000));
    }
}
