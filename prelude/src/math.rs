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
