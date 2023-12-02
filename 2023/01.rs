use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 01)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let mut iter = l.chars().filter(char::is_ascii_digit).peekable();
            let first = iter.peek().unwrap().to_digit(10).unwrap() as i64;
            let last = iter.next_back().unwrap().to_digit(10).unwrap() as i64;
            first * 10 + last
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    const WORDS: &[&str] = &[
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    input.lines().map(|l| {
        let mut first = 0;
        'out: for i in 0..l.len() {
            let char = l.chars().nth(i).unwrap();
            if char.is_ascii_digit() {
                first = char.to_digit(10).unwrap() as i64;
                break;
            }

            for (idx, word) in WORDS.iter().enumerate() {
                if l.find(word) == Some(i) {
                    first = idx as i64;
                    break 'out;
                }
            }
        }

        let mut second = 0;
        'out: for i in (0..l.len()).rev() {
            let char = l.chars().nth(i).unwrap();
            if char.is_ascii_digit() {
                second = char.to_digit(10).unwrap() as i64;
                break;
            }

            for (idx, word) in WORDS.iter().enumerate() {
                if l.rfind(word) == Some(i) {
                    second = idx as i64;
                    break 'out;
                }
            }
        }

        first * 10 + second
    }).sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 54632);
    assert_eq!(part2(input), 54019);
}
