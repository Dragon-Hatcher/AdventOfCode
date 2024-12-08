use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 07)
}

fn parse(l: &str) -> (i64, Vec<i64>) {
    let (target, nums) = l.split_once(":").unwrap();
    (target.nums().nu(), nums.nums().collect())
}

fn works(target: i64, nums: &[i64], extra_op: bool) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    
    let (last, rest) = (nums[0], &nums[1..]);
    let mask = 10i64.pow((last as f64).log(10.0).ceil().max(1.0) as u32);
    works(target - last, rest, extra_op)
        || (target % last == 0) && works(target / last, rest, extra_op)
        || extra_op && target % mask == last && works(target / mask, rest, extra_op)
}

fn solve(input: &str, extra_op: bool) -> i64 {
    input
        .lines()
        .map(parse)
        .filter_map(|(target, mut nums)| {
            nums.reverse();
            works(target, &nums, extra_op).then_some(target)
        })
        .sum()
}

fn part1(input: &str) -> i64 {
    solve(input, false)
}

fn part2(input: &str) -> i64 {
    solve(input, true)
}

fn main() {
    advent::new(2024, 7, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    assert_eq!(part1(input), 3749);
    assert_eq!(part2(input), 11387);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 3119088655389);
    assert_eq!(part2(input), 264184041398847);
}
