use std::time::Duration;

/// An extension trait that adds fluent time unit methods to integer primitives,
/// allowing for highly readable time duration creation.
/// 
/// The functions return a standard Rust `std::time::Duration`.
pub trait DurationExt {
    /// Creates a Duration representing this many seconds.
    fn seconds(self) -> Duration;
    /// Creates a Duration representing this many minutes.
    fn minutes(self) -> Duration;
    /// Creates a Duration representing this many hours.
    fn hours(self) -> Duration;
    /// Creates a Duration representing this many days (24 hours).
    fn days(self) -> Duration;
    /// Creates a Duration representing this many weeks (7 days).
    fn weeks(self) -> Duration;
    /// Creates a Duration representing this many milliseconds.
    fn milliseconds(self) -> Duration;
    /// Creates a Duration representing this many microseconds.
    fn microseconds(self) -> Duration;
    /// Creates a Duration representing this many nanoseconds.
    fn nanoseconds(self) -> Duration;
}

impl DurationExt for u64 {
    fn seconds(self) -> Duration {
        Duration::from_secs(self)
    }

    fn minutes(self) -> Duration {
        Duration::from_secs(self.saturating_mul(60))
    }

    fn hours(self) -> Duration {
        Duration::from_secs(self.saturating_mul(3600))
    }

    fn days(self) -> Duration {
        Duration::from_secs(self.saturating_mul(86400))
    }

    fn weeks(self) -> Duration {
        Duration::from_secs(self.saturating_mul(604800))
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
        Duration::from_secs((self as u64).saturating_mul(60))
    }

    fn hours(self) -> Duration {
        Duration::from_secs((self as u64).saturating_mul(3600))
    }

    fn days(self) -> Duration {
        Duration::from_secs((self as u64).saturating_mul(86400))
    }

    fn weeks(self) -> Duration {
        Duration::from_secs((self as u64).saturating_mul(604800))
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
        Duration::from_secs(self.abs() as u64)
    }

    fn minutes(self) -> Duration {
        Duration::from_secs((self.abs() as u64).saturating_mul(60))
    }

    fn hours(self) -> Duration {
        Duration::from_secs((self.abs() as u64).saturating_mul(3600))
    }

    fn days(self) -> Duration {
        Duration::from_secs((self.abs() as u64).saturating_mul(86400))
    }

    fn weeks(self) -> Duration {
        Duration::from_secs((self.abs() as u64).saturating_mul(604800))
    }
    
    fn milliseconds(self) -> Duration {
        Duration::from_millis(self.abs() as u64)
    }

    fn microseconds(self) -> Duration {
        Duration::from_micros(self.abs() as u64)
    }

    fn nanoseconds(self) -> Duration {
        Duration::from_nanos(self.abs() as u64)
    }
}

impl DurationExt for i32 {
    fn seconds(self) -> Duration {
        Duration::from_secs(self.abs() as u64)
    }

    fn minutes(self) -> Duration {
        Duration::from_secs((self.abs() as u64).saturating_mul(60))
    }

    fn hours(self) -> Duration {
        Duration::from_secs((self.abs() as u64).saturating_mul(3600))
    }

    fn days(self) -> Duration {
        Duration::from_secs((self.abs() as u64).saturating_mul(86400))
    }

    fn weeks(self) -> Duration {
        Duration::from_secs((self.abs() as u64).saturating_mul(604800))
    }

    fn milliseconds(self) -> Duration {
        Duration::from_millis(self.abs() as u64)
    }

    fn microseconds(self) -> Duration {
        Duration::from_micros(self.abs() as u64)
    }

    fn nanoseconds(self) -> Duration {
        Duration::from_nanos(self.abs() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // --- U64 Tests ---
    #[test]
    fn test_u64_large_units() {
        let five: u64 = 5;
        assert_eq!(five.seconds(), Duration::from_secs(5));
        assert_eq!(five.minutes(), Duration::from_secs(5 * 60));
        assert_eq!(five.hours(), Duration::from_secs(5 * 3600));
        assert_eq!(five.days(), Duration::from_secs(5 * 86400));
        assert_eq!(five.weeks(), Duration::from_secs(5 * 604800));
    }

    #[test]
    fn test_u64_small_units() {
        let one: u64 = 1;
        assert_eq!(one.milliseconds(), Duration::from_millis(1));
        assert_eq!(one.microseconds(), Duration::from_micros(1));
        assert_eq!(one.nanoseconds(), Duration::from_nanos(1));

        let large_num: u64 = 1_234_567;
        assert_eq!(large_num.milliseconds(), Duration::from_millis(1_234_567));
    }

    // --- U32 Tests ---
    #[test]
    fn test_u32_large_units() {
        let three: u32 = 3;
        assert_eq!(three.seconds(), Duration::from_secs(3));
        assert_eq!(three.minutes(), Duration::from_secs(3 * 60));
        assert_eq!(three.hours(), Duration::from_secs(3 * 3600));
    }

    #[test]
    fn test_u32_small_units() {
        let num: u32 = 999;
        assert_eq!(num.milliseconds(), Duration::from_millis(999));
        assert_eq!(num.microseconds(), Duration::from_micros(999));
        assert_eq!(num.nanoseconds(), Duration::from_nanos(999));
    }

    // --- I64 Tests ---
    #[test]
    fn test_i64_positive_and_negative() {
        let pos_ten: i64 = 10;
        let neg_ten: i64 = -10;

        // Large units
        assert_eq!(pos_ten.minutes(), Duration::from_secs(600));
        assert_eq!(neg_ten.minutes(), Duration::from_secs(600));

        // Small units
        assert_eq!(pos_ten.milliseconds(), Duration::from_millis(10));
        assert_eq!(neg_ten.milliseconds(), Duration::from_millis(10));
    }

    // --- I32 Tests ---
    #[test]
    fn test_i32_positive_and_negative() {
        let pos_two: i32 = 2;
        let neg_two: i32 = -2;

        // Large units
        assert_eq!(pos_two.days(), Duration::from_secs(2 * 86400));
        assert_eq!(neg_two.days(), Duration::from_secs(2 * 86400));

        // Small units
        assert_eq!(pos_two.microseconds(), Duration::from_micros(2));
        assert_eq!(neg_two.microseconds(), Duration::from_micros(2));
    }
    
    // --- Saturation Test ---
    #[test]
    fn test_saturating_mul_no_panic() {
        let max_u64 = u64::MAX;
        // Test large multiplication for saturation
        let duration = max_u64.minutes();
        assert!(duration.as_secs() > 0); 
        
        // Test small multiplication where the value is simply passed to std::time::Duration 
        // (i.e., we rely on Duration::from_* to handle the conversion/saturation internally, 
        // but ensure our methods don't panic)
        let small_duration = max_u64.milliseconds();
        assert!(small_duration.as_millis() > 0);
    }
}
