use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 01)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let mut iter = l.chars().filter(char::is_ascii_digit).peekable();
            let first = iter.clone().nu().to_digit(10).unwrap() as i64;
            let last = iter.nbu().to_digit(10).unwrap() as i64;
            first * 10 + last
        })
        .sum()
}

fn parse_match(str: &str) -> i64 {
    match str {
        "zero" | "0" => 0,
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("invalid match {str}"),
    }
}

fn part2(input: &str) -> i64 {
    let start_re = regex!(r#"\d|zero|one|two|three|four|five|six|seven|eight|nine"#);
    let end_re = regex!(r#".*(\d|zero|one|two|three|four|five|six|seven|eight|nine)"#);

    input
        .lines()
        .map(|l| {
            let first = parse_match(start_re.find(l).unwrap().as_str());
            let last = parse_match(end_re.captures(l).unwrap().get(1).unwrap().as_str());
            first * 10 + last
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example1() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(part1(input), 142);
}

#[test]
fn example2() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(part2(input), 281);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 54632);
    assert_eq!(part2(input), 54019);
}
