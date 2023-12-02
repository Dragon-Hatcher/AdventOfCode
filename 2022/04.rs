use advent::prelude::*;
use std::{ops::RangeInclusive, str::FromStr};

fn default_input() -> &'static str {
    include_input!(2022 / 04)
}

#[derive(Debug, Clone)]
struct Ranges {
    first: RangeInclusive<i64>,
    second: RangeInclusive<i64>,
}

impl FromStr for Ranges {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_start, first_end, second_start, second_end) = s.nums_pos().tup();

        Ok(Ranges {
            first: first_start..=first_end,
            second: second_start..=second_end,
        })
    }
}

fn subsumes(outer: &RangeInclusive<i64>, inner: &RangeInclusive<i64>) -> bool {
    inner.start() >= outer.start() && inner.end() <= outer.end()
}

impl Ranges {
    fn subsumed(&self) -> bool {
        subsumes(&self.first, &self.second) || subsumes(&self.second, &self.first)
    }

    fn overlap(self) -> bool {
        self.subsumed()
            || self.second.contains(self.first.start())
            || self.second.contains(self.first.end())
    }
}
fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.parse::<Ranges>().unwrap().subsumed() as i64)
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.parse::<Ranges>().unwrap().overlap() as i64)
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part1(input), 2);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 651);
    assert_eq!(part2(input), 956);
}
