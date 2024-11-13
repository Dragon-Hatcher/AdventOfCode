use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 10)
}

fn iterate(digits: &[u8]) -> Vec<u8> {
    let mut cur_val = digits[0];
    let mut run_len = 1;
    let mut out = Vec::new();
    for &d in digits.iter().skip(1) {
        if cur_val == d {
            run_len += 1;
        } else {
            out.push(run_len);
            out.push(cur_val);
            cur_val = d;
            run_len = 1;
        }
    }
    out.push(run_len);
    out.push(cur_val);

    out
}

fn solve(input: &str, iterations: usize) -> i64 {
    let mut start = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect_vec();
    for _ in 0..iterations {
        start = iterate(&start);
    }
    start.len() as i64
}

fn part1(input: &str) -> i64 {
    solve(input, 40)
}

fn part2(input: &str) -> i64 {
    solve(input, 50)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(iterate(&[1]), vec![1, 1]);
    assert_eq!(iterate(&[1, 1]), vec![2, 1]);
    assert_eq!(iterate(&[2, 1]), vec![1, 2, 1, 1]);
    assert_eq!(iterate(&[1, 2, 1, 1]), vec![1, 1, 1, 2, 2, 1]);
    assert_eq!(iterate(&[1, 1, 1, 2, 2, 1]), vec![3, 1, 2, 2, 1, 1]);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 252594);
    assert_eq!(part2(input), 3579328);
}
