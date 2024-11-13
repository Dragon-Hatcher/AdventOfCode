use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 01)
}

fn delta(c: char) -> i64 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn part1(input: &str) -> i64 {
    input.chars().map(delta).sum()
}

fn part2(input: &str) -> i64 {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        floor += delta(c);
        if floor == -1 {
            return i as i64 + 1;
        }
    }

    -1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "()())";
    assert_eq!(part1(input), -1);
    assert_eq!(part2(input), 5);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 280);
    assert_eq!(part2(input), 1797);
}
