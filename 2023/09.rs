use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 09)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let mut all: Vec<Vec<i64>> = Vec::new();

            let mut nums = l.nums().collect_vec();
            all.push(nums.clone());
            loop {
                let next = nums
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec();
                all.push(next.clone());
                if next.iter().all(|n| *n == 0) {
                    break;
                }
                nums = next;
            }

            all.last_mut().unwrap().push(0);
            for i in (0..all.len() - 1).rev() {
                let add = all[i + 1].last().unwrap();
                let val = all[i].last().unwrap() + add;
                all[i].push(val);
            }

            *all[0].last().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let mut all: Vec<Vec<i64>> = Vec::new();

            let mut nums = l.nums().collect_vec();
            all.push(nums.clone());
            loop {
                let next = nums
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec();
                all.push(next.clone());
                if next.iter().all(|n| *n == 0) {
                    break;
                }
                nums = next;
            }

            all.last_mut().unwrap().insert(0, 0);
            for i in (0..all.len() - 1).rev() {
                let sub = all[i + 1].first().unwrap();
                let val = all[i].first().unwrap() - sub;
                all[i].insert(0, val);
            }

            *all[0].first().unwrap()
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
    assert_eq!(part1(input), 114);
    assert_eq!(part2(input), 2);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1842168671);
    assert_eq!(part2(input), 903);
}
