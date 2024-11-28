use itertools::Itertools;

pub trait NumExtensions
where
    Self: Sized,
{
    fn divisors(self) -> impl Iterator<Item = Self>;
}

impl NumExtensions for i64 {
    fn divisors(self) -> impl Iterator<Item = i64> {
        (1..=self)
            .take_while(move |n| n * n <= self)
            .filter(move |n| self % n == 0)
            .flat_map(move |n| [n, self / n])
            .dedup()
    }
}

/// Returns the greatest common divisor of two numbers.
pub fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x != 0 {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}

/// Returns the least common multiple of two numbers.
pub fn lcm(x: i64, y: i64) -> i64 {
    x * y / gcd(x, y)
}

pub fn sum_to(n: i64) -> i64 {
    n * (n + 1) / 2
}
