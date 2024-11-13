pub use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 01)
}

fn delta(c: char) -> i64 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn part1(input: &str) -> i64 {
    input.chars().map(delta).sum()
}

fn part2(input: &str) -> i64 {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        floor += delta(c);
        if floor == -1 {
            return i as i64 + 1;
        }
    }

    -1
}

fn main() {
    advent::new(default_input).part1(part1).part2(part2).cli();
}
