use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 01)
}

fn part1(input: &str) -> i64 {
    input
        .sections()
        .map(|section| section.nums().sum())
        .max()
        .unwrap_or_default()
}

fn part2(input: &str) -> i64 {
    input
        .sections()
        .map(|section| section.nums().sum::<i64>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    assert_eq!(part1(input), 24000);
    assert_eq!(part2(input), 45000);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 67016);
    assert_eq!(part2(input), 200116);
}
