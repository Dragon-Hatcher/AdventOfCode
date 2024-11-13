use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 16)
}

fn checksum(file: &str) -> String {
    let mut chk = file.to_owned();

    while chk.len() % 2 == 0 {
        chk = chk
            .chars()
            .tuples()
            .map(|(a, b)| if a == b { '1' } else { '0' })
            .collect();
    }

    chk
}

fn dragon(start: &str, len: usize) -> String {
    let mut dragon = start.to_owned();

    while dragon.len() < len {
        let right: String = dragon
            .chars()
            .rev()
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect();
        dragon.push('0');
        dragon.push_str(&right);
    }

    dragon
}

fn solve(input: &str, size: usize) -> String {
    let dragon = dragon(input.trim(), size);
    let sized: String = dragon.chars().take(size).collect();
    checksum(&sized)
}

fn part1(input: &str) -> String {
    solve(input, 272)
}

fn part2(input: &str) -> String {
    solve(input, 35651584)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(dragon("10000", 20), "10000011110010000111110");
    assert_eq!(checksum("10000011110010000111"), "01100");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "10111110010110110");
    assert_eq!(part2(input), "01101100001100100");
}
