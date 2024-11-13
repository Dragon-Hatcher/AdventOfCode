use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 05)
}

fn solve(jumps: &mut [i64], change: impl Fn(i64) -> i64) -> i64 {
    let mut idx: i64 = 0;
    let mut steps = 0;

    while idx >= 0 && idx < jumps.len() as i64 {
        let off = jumps[idx as usize];
        jumps[idx as usize] += change(off);
        idx += off;

        steps += 1;
    }

    steps
}

fn part1(input: &str) -> i64 {
    let mut jumps = input.nums().collect_vec();
    solve(&mut jumps, |_| 1)
}

fn part2(input: &str) -> i64 {
    let mut jumps = input.nums().collect_vec();
    solve(&mut jumps, |off| if off >= 3 { -1 } else { 1 })
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "0 3 0 1 -3";
    assert_eq!(part1(input), 5);
    assert_eq!(part2(input), 10);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 339351);
    assert_eq!(part2(input), 24315397);
}
