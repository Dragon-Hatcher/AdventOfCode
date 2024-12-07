use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 07)
}

fn parse(l: &str) -> (i64, Vec<i64>) {
    let (target, nums) = l.split_once(":").unwrap();
    (target.nums().nu(), nums.nums().collect())
}

fn solve(working: i64, nums: &[i64], all: &mut HashSet<i64>) {
    if nums.is_empty() {
        all.insert(working);
        return;
    }

    let next = nums[0];
    solve(working + next, &nums[1..], all);
    solve(working * next, &nums[1..], all);
}

fn part1(input: &str) -> i64 {
    let mut c = 0;
    for (target, nums) in input.lines().map(parse) {
        let mut all = HashSet::default();
        solve(nums[0], &nums[1..], &mut all);
        if all.into_iter().any(|n| n == target) {
            c += target;
        }
    }
    c
}

fn part2(input: &str) -> i64 {
    let mut c = 0;
    for (target, nums) in input.lines().map(parse) {
        let mut all = HashSet::default();
        solve2(nums[0], &nums[1..], &mut all);
        if all.into_iter().any(|n| n == target) {
            c += target;
        }
    }
    c
}

fn solve2(working: i64, nums: &[i64], all: &mut HashSet<i64>) {
    if nums.is_empty() {
        all.insert(working);
        return;
    }

    let next = nums[0];
    solve2(working + next, &nums[1..], all);
    solve2(working * next, &nums[1..], all);
    solve2(format!("{working}{next}").parse().unwrap(), &nums[1..], all);

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
