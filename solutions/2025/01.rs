use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2025 / 01)
}

fn parse_line(line: &str) -> i64 {
    let n = line.nums().nu();
    n * if line.starts_with('L') { -1 } else { 1 }
}

fn part1(input: &str) -> i64 {
    let input = input.lines().map(parse_line);

    let mut dial = 50;
    let mut zero_count = 0;
    for turn in input {
        dial += turn;
        dial = dial.rem_euclid(100);
        zero_count += (dial == 0) as i64;
    }

    zero_count
}

fn part2(input: &str) -> i64 {
    let input = input.lines().map(parse_line);

    let mut dial = 50;
    let mut zero_count = 0;
    for turn in input {
        for _ in 0..turn.abs() {
            dial += turn.signum();
            dial = dial.rem_euclid(100);
            zero_count += (dial == 0) as i64;
        }
    }

    zero_count
}

fn main() {
    advent::new(2026, 1, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    assert_eq!(part1(input), 3);
    assert_eq!(part2(input), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1152);
    assert_eq!(part2(input), 6671);
}
