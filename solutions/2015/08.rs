use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 08)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let mut escape_next = 0;
            let mut skipped = 0;

            for (c, nc) in l.chars().tuple_windows() {
                if escape_next != 0 {
                    escape_next -= 1;
                    skipped += 1;
                } else if c == '\\' && nc == 'x' {
                    escape_next += 3;
                } else if c == '\\' {
                    escape_next += 1;
                }
            }

            skipped + 2
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
    advent::new(2015, 8, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
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
