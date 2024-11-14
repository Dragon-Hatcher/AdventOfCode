use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 02)
}

fn part1(input: &str) -> i64 {
    _ = input;
    todo!("Part 1")
}

fn part2(input: &str) -> i64 {
    _ = input;
    todo!("Part 2")
}

fn main() {
    advent::new(2015, 02, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "2x3x4";
    assert_eq!(part1(input), 58);
    assert_eq!(part2(input), 34);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 15863000);
    assert_eq!(part2(input), 3737498);
}
