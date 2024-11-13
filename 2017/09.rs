use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 09)
}

fn solve(input: &str) -> (i64, i64) {
    let mut total_score = 0;
    let mut garbage_chars = 0;

    let mut level = 0;
    let mut garbage = false;
    let mut ignore = false;

    for c in input.trim().chars() {
        if ignore {
            ignore = false;
            continue;
        }

        match c {
            '<' => {
                if !garbage {
                    garbage = true;
                } else {
                    garbage_chars += 1;
                }
            }
            '>' => {
                garbage = false;
            }
            '{' => {
                if !garbage {
                    level += 1;
                    total_score += level;
                } else {
                    garbage_chars += 1;
                }
            }
            '}' => {
                if !garbage {
                    level -= 1;
                } else {
                    garbage_chars += 1;
                }
            }
            '!' => {
                ignore = true;
            }
            _ => {
                if garbage {
                    garbage_chars += 1;
                }
            }
        }
    }

    (total_score, garbage_chars)
}

fn part1(input: &str) -> i64 {
    solve(input).0
}

fn part2(input: &str) -> i64 {
    solve(input).1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("{}"), 1);
    assert_eq!(part1("{{{}}}"), 6);
    assert_eq!(part1("{{},{}}"), 5);
    assert_eq!(part1("{{{},{},{{}}}}"), 16);
    assert_eq!(part1("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);

    assert_eq!(part2("<>"), 0);
    assert_eq!(part2("<random characters>"), 17);
    assert_eq!(part2("<<<<>"), 3);
    assert_eq!(part2("<{!>}>"), 2);
    assert_eq!(part2("<!!>"), 0);
    assert_eq!(part2("<!!!>>"), 0);
    assert_eq!(part2("<{o\"i!a,<{i<a>"), 10);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 15922);
    assert_eq!(part2(input), 7314);
}
