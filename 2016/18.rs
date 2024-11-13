use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 18)
}

fn solve(input: &str, rows: usize) -> i64 {
    let len = input.chars().count();
    let mask = ((1 << len) - 1) << 1; // 0111...1110

    let mut row = 0u128;
    let mut safe_cnt = 0;

    for (i, c) in input.chars().enumerate() {
        row |= ((c == '^') as u128) << (i + 1);
    }

    for _ in 0..rows {
        safe_cnt += len - row.count_ones() as usize;
        row = ((row << 1) ^ (row >> 1)) & mask;
    }

    safe_cnt as i64
}

fn part1(input: &str) -> i64 {
    solve(input.trim(), 40)
}

fn part2(input: &str) -> i64 {
    solve(input.trim(), 400000)
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
