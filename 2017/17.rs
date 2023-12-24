use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 17)
}

fn part1(input: &str) -> i64 {
    let steps = input.nums().nu() as usize;

    let mut nums = vec![0];
    let mut pos: usize = 0;

    for i in 1..=2017 {
        pos += steps;
        pos %= nums.len();
        pos += 1;
        nums.insert(pos, i);
    }

    nums[(pos + 1) % nums.len()]
}

fn part2(input: &str) -> i64 {
    let steps = input.nums().nu() as usize;

    let mut len = 1;
    let mut pos: usize = 0;
    let mut answer = 0;

    for i in 1..=50000000 {
        pos += steps;
        pos %= len;
        len += 1;
        if pos == 0 {
            answer = i;
        }
        pos += 1;
    }

    answer
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("3"), 638);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1670);
    assert_eq!(part2(input), 2316253);
}
