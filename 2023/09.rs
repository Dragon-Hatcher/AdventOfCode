use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 09)
}

fn get_lists(l: &str) -> Vec<Vec<i64>> {
    let mut all = vec![l.nums().collect_vec()];

    while all.last().unwrap().iter().any(|n| *n != 0) {
        all.push(
            all.last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect(),
        );
    }

    all
}

fn solve(input: &str, fold: impl FnMut(i64, &Vec<i64>) -> i64 + Copy) -> i64 {
    input
        .lines()
        .map(|l| get_lists(l).iter().rev().fold(0, fold))
        .sum()
}

fn part1(input: &str) -> i64 {
    solve(input, |a, ns| a + ns.last().unwrap())
}

fn part2(input: &str) -> i64 {
    solve(input, |a, ns| ns.first().unwrap() - a)
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
