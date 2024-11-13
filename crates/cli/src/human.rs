//! Format numbers for humans.

use std::cmp::max;
use std::fmt;

/// A number that is scaled in an easy to read way.
#[derive(Debug, Clone, Copy)]
pub struct Number(f64, Scale);

/// The scale to represent the number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scale {
    Nano,
    Micro,
    Milli,
    Unit,
    Kilo,
    Mega,
    Giga,
}

/// Represents the time taken.
#[derive(Debug, Clone, Copy)]
pub struct Time(Number);

impl Number {
    pub fn new(v: f64) -> Self {
        let scales = [
            (Scale::Giga, 1e-9),
            (Scale::Mega, 1e-6),
            (Scale::Kilo, 1e-3),
            (Scale::Unit, 1e0),
            (Scale::Milli, 1e3),
            (Scale::Micro, 1e6),
            (Scale::Nano, 1e9),
        ];
        let scale = scales
            .into_iter()
            .find_map(|(s, f)| if v * f >= 1.0 { Some(s) } else { None })
            .unwrap_or(Scale::Nano);
        Self::with_scale(v, scale)
    }

    pub fn with_scale(v: f64, scale: Scale) -> Self {
        let v = match scale {
            Scale::Nano => v * 1e9,
            Scale::Micro => v * 1e6,
            Scale::Milli => v * 1e3,
            Scale::Unit => v * 1e0,
            Scale::Kilo => v * 1e-3,
            Scale::Mega => v * 1e-6,
            Scale::Giga => v * 1e-9,
        };
        Self(v, scale)
    }
}

impl Time {
    pub fn new(secs: f64) -> Self {
        Self(Number::new(secs))
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Some examples of what we need to handle nicely
        //
        // - 500 s
        // - 180.4 s
        // - 1.234 s
        // - 42.53 ms
        // - 978.1 µs
        //
        // Rules:
        // - If very large >= 300 then use no precision
        // - Right align number in a width of 5 characters and fill the space
        //   if possible

        let &Self(Number(v, s)) = self;
        let p = 4 - digits(v);
        let (precision, v, suffix) = match s {
            Scale::Nano => (p, v, " ns"),
            Scale::Micro => (p, v, " µs"),
            Scale::Milli => (p, v, " ms"),
            Scale::Unit => (if v >= 300.0 { 0 } else { p }, v, " s"),
            Scale::Kilo => (0, v * 1e3, " s"),
            Scale::Mega => (0, v * 1e6, " s"),
            Scale::Giga => (0, v * 1e9, " s"),
        };
        fmt::Display::fmt(&format!("{v:.precision$}{suffix}"), f)
    }
}

fn digits(mut v: f64) -> usize {
    let mut n = 0;
    while v >= 1. {
        v *= 0.1;
        n += 1;
    }
    max(n, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64_digits() {
        assert_eq!(digits(1.0), 1);
        assert_eq!(digits(0.123), 1);
        assert_eq!(digits(10.0), 2);
        assert_eq!(digits(15.0), 2);
        assert_eq!(digits(199.99), 3);
        assert_eq!(digits(123.123), 3);
    }

    #[test]
    fn time_display() {
        let test_cases = [
            (500.0, "500 s"),
            (180.4, "180.4 s"),
            (1.234, "1.234 s"),
            (0.04253, "42.53 ms"),
            (0.0009781, "978.1 µs"),
        ];
        for (t, expected) in test_cases {
            assert_eq!(Time::new(t).to_string(), expected);
        }
    }
}
