use std::iter::once;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 18)
}

fn solve(input: &str, rows: usize) -> i64 {
    let mut row = input.trim().chars().map(|c| c == '^').collect_vec();
    let mut safe_cnt = row.iter().filter(|t| !**t).count();

    for _ in 1..rows {
        row = chain!(once(false), row, once(false))
            .tuple_windows()
            .map(|(a, _, b)| a != b)
            .collect();

        safe_cnt += row.iter().filter(|t| !**t).count();
    }

    safe_cnt as i64
}

fn part1(input: &str) -> i64 {
    solve(input, 40)
}

fn part2(input: &str) -> i64 {
    solve(input, 400000)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = ".^^.^.^^^^";
    assert_eq!(solve(input, 10), 38);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1978);
    assert_eq!(part2(input), 20003246);
}
