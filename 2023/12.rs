use std::iter::once;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 12)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Working,
    Damaged,
    Unknown,
}

impl Spring {
    fn is_gap(&self) -> bool {
        matches!(self, Spring::Working | Spring::Unknown)
    }

    fn is_damaged(&self) -> bool {
        matches!(self, Spring::Damaged | Spring::Unknown)
    }
}

fn parse_spring(char: char) -> Spring {
    match char {
        '.' => Spring::Working,
        '#' => Spring::Damaged,
        _ => Spring::Unknown,
    }
}

fn count_arrangements(springs: &[Spring], groups: &[i64]) -> i64 {
    if springs.is_empty() || groups.is_empty() {
        return (groups.is_empty() && springs.iter().all(Spring::is_gap)) as i64;
    }

    let group = groups[0] as usize;
    let rest_groups = &groups[1..];

    let min_remaining = rest_groups.iter().sum::<i64>() as usize + rest_groups.len() - 1;

    if group > springs.len() {
        return 0;
    }

    let mut sum = 0;

    for i in 0..=(springs.len() - group).min(springs.len() - min_remaining) {
        if springs[0..i].iter().all(Spring::is_gap)
            && springs[i..i + group].iter().all(Spring::is_damaged)
            && (i + group == springs.len() || springs[i + group].is_gap())
        {
            let rest_springs = &springs[(i + group + 1).min(springs.len())..];
            sum += count_arrangements(rest_springs, rest_groups);
        }
    }

    sum
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (springs, group_sizes) = l.split_once(' ').unwrap();
            let springs = springs.chars().map(parse_spring).collect_vec();
            let group_sizes = group_sizes.nums().collect_vec();
            count_arrangements(&springs, &group_sizes)
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (springs, group_sizes) = l.split_once(' ').unwrap();
            let spring_cnt = springs.len();
            let group_cnt = group_sizes.nums().count();

            let springs = springs
                .chars()
                .map(parse_spring)
                .chain(once(Spring::Unknown))
                .cycle()
                .take((spring_cnt + 1) * 5 - 1)
                .collect_vec();
            let group_sizes = group_sizes.nums().cycle().take(group_cnt * 5).collect_vec();

            count_arrangements(&springs, &group_sizes)
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    assert_eq!(part1(input), 21);
    assert_eq!(part2(input), 525152);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
