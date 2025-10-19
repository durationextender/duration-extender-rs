use std::time::Duration;

/// An extension trait that adds fluent time unit methods to integer primitives,
/// allowing for highly readable time duration creation.
///
/// # Panics
///
/// - Signed integers (`i32`, `i64`) **panic** if the value is negative.
/// - All integer types **panic** on overflow when creating a `Duration` (e.g., very large minutes, hours, days, or weeks).
///
/// # Examples
///
/// ```rust
/// use duration_extender::DurationExt;
/// use std::time::Duration;
///
/// let timeout = 10.seconds();
/// let delay = 5.minutes();
/// let long_wait = 2.days();
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
    /// Creates a `Duration` representing this many days (24 hours).
    ///
    /// # Note
    /// This is a fixed duration of 86,400 seconds. It does **not** account for calendar days or DST.
    fn days(self) -> Duration;
    /// Creates a `Duration` representing this many weeks (7 days).
    ///
    /// # Note
    /// This is a fixed duration of 604,800 seconds. It does **not** account for calendar weeks or DST.
    fn weeks(self) -> Duration;
    /// Creates a `Duration` representing this many milliseconds.
    fn milliseconds(self) -> Duration;
    /// Creates a `Duration` representing this many microseconds.
    fn microseconds(self) -> Duration;
    /// Creates a `Duration` representing this many nanoseconds.
    fn nanoseconds(self) -> Duration;
}


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

    fn days(self) -> Duration {
        let secs = self.checked_mul(86400)
            .expect(&format!("duration value {} days overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn weeks(self) -> Duration {
        let secs = self.checked_mul(604800)
            .expect(&format!("duration value {} weeks overflows u64 seconds capacity", self));
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

    fn minutes(self) -> Duration {
        let secs = (self as u64).checked_mul(60)
            .expect(&format!("duration value {} minutes overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn hours(self) -> Duration {
        let secs = (self as u64).checked_mul(3600)
            .expect(&format!("duration value {} hours overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn days(self) -> Duration {
        let secs = (self as u64).checked_mul(86400)
            .expect(&format!("duration value {} days overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn weeks(self) -> Duration {
        let secs = (self as u64).checked_mul(604800)
            .expect(&format!("duration value {} weeks overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
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

impl DurationExt for i64 {
    fn seconds(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} seconds", self);
        Duration::from_secs(self as u64)
    }

    fn minutes(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} minutes", self);
        let secs = (self as u64).checked_mul(60)
            .expect(&format!("duration value {} minutes overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn hours(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} hours", self);
        let secs = (self as u64).checked_mul(3600)
            .expect(&format!("duration value {} hours overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn days(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} days", self);
        let secs = (self as u64).checked_mul(86400)
            .expect(&format!("duration value {} days overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn weeks(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} weeks", self);
        let secs = (self as u64).checked_mul(604800)
            .expect(&format!("duration value {} weeks overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
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
        let secs = (self as u64).checked_mul(60)
            .expect(&format!("duration value {} minutes overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn hours(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} hours", self);
        let secs = (self as u64).checked_mul(3600)
            .expect(&format!("duration value {} hours overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn days(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} days", self);
        let secs = (self as u64).checked_mul(86400)
            .expect(&format!("duration value {} days overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
    }

    fn weeks(self) -> Duration {
        assert!(self >= 0, "duration cannot be negative: got {} weeks", self);
        let secs = (self as u64).checked_mul(604800)
            .expect(&format!("duration value {} weeks overflows u64 seconds capacity", self));
        Duration::from_secs(secs)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // --- U64 Tests ---
    // The largest number that can be multiplied by 60 without overflowing u64
    const MAX_FOR_MINUTES: u64 = u64::MAX / 60;
    // A number one larger than MAX_FOR_MINUTES
    const OVERFLOW_MINUTES: u64 = MAX_FOR_MINUTES.saturating_add(1);

    #[test]
    fn test_u64_large_units() {
        let five: u64 = 5;
        assert_eq!(five.minutes(), Duration::from_secs(5 * 60));
    }
    
    // New test to ensure overflow now panics
    #[test]
    #[should_panic(expected = "overflows u64 seconds capacity")]
    fn test_u64_minutes_panics_on_overflow() {
        let _ = OVERFLOW_MINUTES.minutes();
    }


    // --- I64 Tests ---
    #[test]
    fn test_i64_positive() {
        let pos_ten: i64 = 10;
        assert_eq!(pos_ten.minutes(), Duration::from_secs(600));
    }

    #[test]
    #[should_panic(expected = "duration cannot be negative")]
    fn test_i64_negative_panics() {
        let neg_ten: i64 = -10;
        let _ = neg_ten.minutes(); 
    }
    
    // New test to ensure signed overflow now panics
    #[test]
    #[should_panic(expected = "overflows u64 seconds capacity")]
    fn test_i64_minutes_panics_on_overflow() {
        let _ = (OVERFLOW_MINUTES as i64).minutes();
    }

   
    #[test]
    fn test_max_u64_small_units() {
        let max_u64 = u64::MAX;
        // This relies on Duration::from_millis to handle its own limits
        let small_duration = max_u64.milliseconds();
        assert!(small_duration.as_millis() > 0);
    }
}
