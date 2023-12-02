use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 06)
}

fn different(str: &str) -> bool {
    str.chars().enumerate().all(|(i, c1)| {
        str.chars().skip(i + 1).all(|c2| c1 != c2)
    })
}

fn signal_pos(str: &str, size: usize) -> i64 {
    let mut i = size;
    loop {
        if different(&str[i - size..i]) {
            return i as i64;
        }
        i += 1;
    }
}

fn part1(input: &str) -> i64 {
    input.lines().map(|l| signal_pos(l, 4)).sum()
}

fn part2(input: &str) -> i64 {
    input.lines().map(|l| signal_pos(l, 14)).sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part1(input), 10);
    assert_eq!(part2(input), 29);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1100);
    assert_eq!(part2(input), 2421);
}
