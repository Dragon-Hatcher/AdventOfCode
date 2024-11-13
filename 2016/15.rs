use advent::prelude::*;
use std::iter::once;

fn default_input() -> &'static str {
    include_input!(2016 / 15)
}

fn disks(input: &str) -> impl Iterator<Item = (i64, i64)> + '_ {
    input.lines().map(|l| {
        let (_, p, _, s) = l.nums().tup();
        (p, s)
    })
}

fn solve(disks: impl Iterator<Item = (i64, i64)>) -> i64 {
    let mut step_size = 1;
    let mut trying = 1;

    for (i, (positions, start)) in disks.enumerate() {
        while (start + trying + i as i64 + 1) % positions != 0 {
            trying += step_size;
        }
        step_size = lcm(step_size, positions);
    }

    trying
}

fn part1(input: &str) -> i64 {
    solve(disks(input))
}

fn part2(input: &str) -> i64 {
    solve(disks(input).chain(once((11, 0))))
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "Disc #1 has 5 positions; at time=0, it is at position 4.
    Disc #2 has 2 positions; at time=0, it is at position 1.";
    assert_eq!(part1(input), 5);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 203660);
    assert_eq!(part2(input), 2408135);
}
