use crate::standard_parsers::nums;

pub fn part1(input: &str) -> i64 {
    let nums = nums(input);

    nums.reduce(|a, b| a + b).unwrap_or_default()
}

pub fn part2(input: &str) -> i64 {
    let nums = nums(input);

    nums.reduce(|a, b| a * b).unwrap_or(1)
}
