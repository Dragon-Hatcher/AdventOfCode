use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2025 / 03)
}

fn solve_line(line: &str, mut take: usize) -> i64 {
    let nums = line.chars().map(|c| c.to_digit(10).unwrap() as i64).collect_vec();

    fn earliest_max(nums: &[i64]) -> (i64, usize) {
        let mut max = -1;
        let mut pos = 0;

        for (i, n) in nums.iter().enumerate() {
            if *n > max {
                max = *n;
                pos = i
            }
        }

        (max, pos)
    }

    let mut sol = 0;
    let mut nums = &nums[..];
    while take > 0 {
        let (n, p) = earliest_max(&nums[..=nums.len() - take]);
        sol *= 10;
        sol += n;
        take -= 1;
        nums = &nums[p+1..];
    }

    sol
}

fn part1(input: &str) -> i64 {
    input.lines().map(|l| solve_line(l, 2)).sum()
}

fn part2(input: &str) -> i64 {
    input.lines().map(|l| solve_line(l, 12)).sum()
}

fn main() {
    advent::new(2025, 3, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(part1(input), 357);
    assert_eq!(part2(input), 3121910778619);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 17405);
    assert_eq!(part2(input), 171990312704598);
}
