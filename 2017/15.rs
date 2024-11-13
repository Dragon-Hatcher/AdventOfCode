use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 15)
}

const A_MUL: i64 = 16807;
const B_MUL: i64 = 48271;
const MOD: i64 = 2147483647;
const MASK: i64 = 0xFFFF;

fn part1(input: &str) -> i64 {
    let (mut a, mut b) = input.nums().tup();
    let mut matched = 0;

    for _ in 0..40000000 {
        a *= A_MUL;
        a %= MOD;
        b *= B_MUL;
        b %= MOD;

        if a & MASK == b & MASK {
            matched += 1;
        }
    }

    matched
}

fn part2(input: &str) -> i64 {
    let (mut a, mut b) = input.nums().tup();
    let mut matched = 0;

    for _ in 0..5000000 {
        while {
            a *= A_MUL;
            a %= MOD;

            a % 4 != 0
        } {}

        while {
            b *= B_MUL;
            b %= MOD;
    
            b % 8 != 0
        } {}

        if a & MASK == b & MASK {
            matched += 1;
        }
    }

    matched
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "65, 8921";
    assert_eq!(part1(input), 588);
    assert_eq!(part2(input), 309);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 638);
    assert_eq!(part2(input), 343);
}
