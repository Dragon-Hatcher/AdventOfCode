use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!({ year } / { day })
}

fn part1(input: &str) -> i64 {
    todo!("Part 1")
}

fn part2(input: &str) -> i64 {
    todo!("Part 2")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "";
    assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}
