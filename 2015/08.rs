use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 08)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let mut skip = 0;
            let mut total_skipped = 0;
            for c in l.chars() {
                if skip > 0 {
                    skip -= 1;
                    if c == 'x' {
                        skip = 2;
                        total_skipped += 2;
                    }
                } else if c == '\\' {
                    skip = 1;
                    total_skipped += 1;
                }
            }
            total_skipped + 2
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| 2 + l.chars().filter(|&c| c == '\\' || c == '"').count() as i64)
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
    assert_eq!(part1(input), 12);
    assert_eq!(part2(input), 19);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1371);
    assert_eq!(part2(input), 2117);
}
