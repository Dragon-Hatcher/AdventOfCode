use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 03)
}

fn part1(input: &str) -> i64 {
    regex!(r#"mul\(\d+,\d+\)"#)
        .find_iter(input)
        .map(|m| m.as_str().nums().product::<i64>())
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut d = true;
    let mut sum = 0;

    for m in regex!(r#"mul\(\d+,\d+\)|do\(\)|don't\(\)"#).find_iter(input) {
        match m.as_str() {
            "do()" => d = true,
            "don't()" => d = false,
            _ => sum += m.as_str().nums().product::<i64>() * d as i64,
        }
    }

    sum
}

fn main() {
    advent::new(2024, 3, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    assert_eq!(
        part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        161
    );
    assert_eq!(
        part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        48
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 183669043);
    assert_eq!(part2(input), 59097164);
}
