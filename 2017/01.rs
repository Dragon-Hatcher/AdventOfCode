use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 01)
}

fn solve(input: &[u8], skip: usize) -> i64 {
    let mut sum = 0;

    for i in 0..input.len() {
        if input[i] == input[(i + skip) % input.len()] {
            sum += (input[i] - b'0') as i64;
        }
    }

    sum
}

fn part1(input: &str) -> i64 {
    solve(input.trim().as_bytes(), 1)
}

fn part2(input: &str) -> i64 {
    solve(input.trim().as_bytes(), input.len() / 2)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("91212129"), 9);
    assert_eq!(part2("123123"), 12);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1034);
    assert_eq!(part2(input), 1356);
}
