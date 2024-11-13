use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 03)
}

fn calculate_priority(char: char) -> i64 {
    match char {
        'a'..='z' => char as i64 - 'a' as i64 + 1,
        'A'..='Z' => char as i64 - 'A' as i64 + 27,
        _ => 0,
    }
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let first_half = &l[..l.len() / 2];
            let second_half = &l[l.len() / 2..];

            for c in first_half.chars() {
                if second_half.contains(c) {
                    return calculate_priority(c);
                }
            }

            0
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .tuples()
        .map(|(s1, s2, s3)| {
            for c in s1.chars() {
                if s2.contains(c) && s3.contains(c) {
                    return calculate_priority(c);
                }
            }

            0
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(part1(input), 157);
    assert_eq!(part2(input), 70);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 7826);
    assert_eq!(part2(input), 2577);
}
