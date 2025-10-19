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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_u64_methods() {
        let zero: u64 = 0;
        assert_eq!(zero.seconds(), Duration::from_secs(0));
        assert_eq!(zero.minutes(), Duration::from_secs(0));
        assert_eq!(zero.hours(), Duration::from_secs(0));
        assert_eq!(zero.days(), Duration::from_secs(0));
        assert_eq!(zero.weeks(), Duration::from_secs(0));

        let five: u64 = 5;
        assert_eq!(five.seconds(), Duration::from_secs(5));
        assert_eq!(five.minutes(), Duration::from_secs(5 * 60));
        assert_eq!(five.hours(), Duration::from_secs(5 * 3600));
        assert_eq!(five.days(), Duration::from_secs(5 * 86400));
        assert_eq!(five.weeks(), Duration::from_secs(5 * 604800));
    }

    #[test]
    fn test_u32_methods() {
        let zero: u32 = 0;
        assert_eq!(zero.seconds(), Duration::from_secs(0));
        assert_eq!(zero.minutes(), Duration::from_secs(0));
        assert_eq!(zero.hours(), Duration::from_secs(0));
        assert_eq!(zero.days(), Duration::from_secs(0));
        assert_eq!(zero.weeks(), Duration::from_secs(0));

        let five: u32 = 5;
        assert_eq!(five.seconds(), Duration::from_secs(5));
        assert_eq!(five.minutes(), Duration::from_secs(5 * 60));
        assert_eq!(five.hours(), Duration::from_secs(5 * 3600));
        assert_eq!(five.days(), Duration::from_secs(5 * 86400));
        assert_eq!(five.weeks(), Duration::from_secs(5 * 604800));
    }

    #[test]
    fn test_i64_methods() {
        let zero: i64 = 0;
        assert_eq!(zero.seconds(), Duration::from_secs(0));
        assert_eq!(zero.minutes(), Duration::from_secs(0));
        assert_eq!(zero.hours(), Duration::from_secs(0));
        assert_eq!(zero.days(), Duration::from_secs(0));
        assert_eq!(zero.weeks(), Duration::from_secs(0));

        let five: i64 = 5;
        let neg_five: i64 = -5;
        let expected = Duration::from_secs(5 * 60);
        assert_eq!(five.seconds(), Duration::from_secs(5));
        assert_eq!(five.minutes(), expected);
        assert_eq!(five.hours(), Duration::from_secs(5 * 3600));
        assert_eq!(five.days(), Duration::from_secs(5 * 86400));
        assert_eq!(five.weeks(), Duration::from_secs(5 * 604800));

        // Negative values produce duration equal to their absolute value
        assert_eq!(neg_five.seconds(), Duration::from_secs(5));
        assert_eq!(neg_five.minutes(), expected);
        assert_eq!(neg_five.hours(), Duration::from_secs(5 * 3600));
        assert_eq!(neg_five.days(), Duration::from_secs(5 * 86400));
        assert_eq!(neg_five.weeks(), Duration::from_secs(5 * 604800));
    }

    #[test]
    fn test_i32_methods() {
        let zero: i32 = 0;
        assert_eq!(zero.seconds(), Duration::from_secs(0));
        assert_eq!(zero.minutes(), Duration::from_secs(0));
        assert_eq!(zero.hours(), Duration::from_secs(0));
        assert_eq!(zero.days(), Duration::from_secs(0));
        assert_eq!(zero.weeks(), Duration::from_secs(0));

        let five: i32 = 5;
        let neg_five: i32 = -5;
        let expected = Duration::from_secs(5 * 60);
        assert_eq!(five.seconds(), Duration::from_secs(5));
        assert_eq!(five.minutes(), expected);
        assert_eq!(five.hours(), Duration::from_secs(5 * 3600));
        assert_eq!(five.days(), Duration::from_secs(5 * 86400));
        assert_eq!(five.weeks(), Duration::from_secs(5 * 604800));

        // Negative values produce duration equal to their absolute value
        assert_eq!(neg_five.seconds(), Duration::from_secs(5));
        assert_eq!(neg_five.minutes(), expected);
        assert_eq!(neg_five.hours(), Duration::from_secs(5 * 3600));
        assert_eq!(neg_five.days(), Duration::from_secs(5 * 86400));
        assert_eq!(neg_five.weeks(), Duration::from_secs(5 * 604800));
    }

    #[test]
    fn test_saturating_mul_no_panic() {
        let max_u64 = u64::MAX;
        let duration = max_u64.minutes();
        assert!(duration.as_secs() >= max_u64);
    }
}
