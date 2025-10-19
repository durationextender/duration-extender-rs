duration-extender

A small, idiomatic utility crate that extends primitive integer types (u64, i32, etc.) with fluent methods for creating standard library std::time::Duration objects.

This allows you to write highly readable and expressive code when defining timeouts, delays, or schedules.

🚀 Getting Started

Add duration-extender to your Cargo.toml:

[dependencies]
duration-extender = "0.1"


💡 Usage

To use the methods, you must bring the DurationExt trait into scope.

use duration_extender::DurationExt;
use std::time::Duration;

fn main() {
    // Basic usage is extremely clear
    let timeout = 10.seconds();
    assert_eq!(timeout, Duration::from_secs(10));

    // Works great for larger units
    let long_delay = 5.minutes();
    assert_eq!(long_delay.as_secs(), 300);

    // Works with signed integers (i32, i64) by taking the absolute value
    let past_due_time = (-2).hours();
    assert_eq!(past_due_time.as_secs(), 7200);

    // Combine types for easy calculation
    let total_wait = 2.days() + 30.minutes();
}


⚙️ Supported Types

The DurationExt trait is implemented for:

u64

u32

i64

i32

All methods safely use saturating arithmetic and handle negative inputs by taking the absolute value, ensuring a panic-free experience.

📝 License

This project is licensed under either of:

Apache License, Version 2.0

MIT license