use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 19)
}

fn part1(input: &str) -> i64 {
    let elves = input.nums().nu();
    let largest_pow = (elves as u64).next_power_of_two() as i64 / 2;
    (elves - largest_pow) * 2 + 1
}

fn part2(input: &str) -> i64 {
    let size = input.nums().nu();
    let pow_3 = 3i64.pow(size.checked_ilog(3).unwrap());
    let remaining = size - pow_3;

    if remaining < pow_3 {
        remaining
    } else {
        2 * remaining - pow_3
    }
}

fn main() {
    advent::new(2016, 19, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    assert_eq!(part1("5"), 3);
    assert_eq!(part2("5"), 2);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1816277);
    assert_eq!(part2(input), 1410967);
}
