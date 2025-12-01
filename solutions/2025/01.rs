use advent::prelude::*;
use std::iter::repeat_n;

fn default_input() -> &'static str {
    include_input!(2025 / 01)
}

fn parse_line(line: &str) -> i64 {
    let n = line.nums().nu();
    n * if line.starts_with('L') { -1 } else { 1 }
}

fn solve(turns: impl Iterator<Item = i64>) -> i64 {
    turns
        .scan(50, |dial, turn| {
            *dial += turn;
            Some(*dial)
        })
        .filter(|dial| dial.rem_euclid(100) == 0)
        .count() as i64
}

fn part1(input: &str) -> i64 {
    let turns = input.lines().map(parse_line);
    solve(turns)
}

fn part2(input: &str) -> i64 {
    let turns = input
        .lines()
        .map(parse_line)
        .flat_map(|i| repeat_n(i.signum(), i.unsigned_abs() as usize));
    solve(turns)
}

fn main() {
    advent::new(2026, 1, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    assert_eq!(part1(input), 3);
    assert_eq!(part2(input), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1152);
    assert_eq!(part2(input), 6671);
}
