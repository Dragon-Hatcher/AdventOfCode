use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 13)
}

fn is_caught(encounter_time: i64, range: i64) -> bool {
    let cycle_time = (range - 1) * 2;
    encounter_time % cycle_time == 0
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.nums().tup())
        .filter(|&(depth, range)| is_caught(depth, range))
        .map(|(depth, range)| depth * range)
        .sum()
}

fn part2(input: &str) -> i64 {
    let info: Vec<(i64, i64)> = input.lines().map(|l| l.nums().tup()).collect_vec();

    for wait in 0.. {
        if !info.iter().any(|&(depth, range)| is_caught(depth + wait, range)) {
            return wait;
        }
    }

    unreachable!()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "0: 3
1: 2
4: 4
6: 4";
    assert_eq!(part1(input), 24);
    assert_eq!(part2(input), 10);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1476);
    assert_eq!(part2(input), 3937334);
}
