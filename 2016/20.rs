use std::ops::RangeInclusive;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 20)
}

fn parse(input: &str) -> Vec<RangeInclusive<i64>> {
    input.nums_pos().tuples().map(|(a, b)| a..=b).collect_vec()
}

fn next_allowed(after: i64, ranges: &[RangeInclusive<i64>]) -> i64 {
    let mut search_val = after;

    'outer: loop {
        for r in ranges {
            if r.contains(&search_val) {
                search_val = r.end() + 1;
                continue 'outer;
            }
        }

        return search_val;
    }
}

fn next_disallowed(after: i64, ranges: &[RangeInclusive<i64>]) -> Option<i64> {
    ranges
        .iter()
        .map(|r| r.start())
        .filter(|v| **v > after)
        .copied()
        .min()
}

fn part1(input: &str) -> i64 {
    let ranges = parse(input);
    next_allowed(0, &ranges)
}

fn part2(input: &str) -> i64 {
    let ranges = parse(input);

    let mut search_val = 0;
    let mut total = 0;

    loop {
        let allowed = next_allowed(search_val, &ranges);
        let disallowed = next_disallowed(allowed, &ranges);

        match disallowed {
            Some(disallowed) => {
                total += disallowed - allowed;
                search_val = disallowed;
            }
            None => break total,
        }
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "5-8
    0-2
    4-7";
    assert_eq!(part1(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 14975795);
    assert_eq!(part2(input), 101);
}
