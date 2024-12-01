use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 01)
}

fn part1(input: &str) -> i64 {
    let (mut l, mut r): (Vec<_>, Vec<_>) = input
        .nums()
        .tuples::<(i64, i64)>()
        .unzip();

    l.sort();
    r.sort();

    l.into_iter().zip(r).map(|(l, r)| (r - l).abs()).sum()
}

fn part2(input: &str) -> i64 {
    let (l, r): (Vec<_>, Vec<_>) = input
        .nums()
        .tuples::<(i64, i64)>()
        .unzip();

    let counts = r.into_iter().counts();
    l.iter().map(|x| x * counts.get(x).copied().unwrap_or_default() as i64).sum()
}

fn main() {
    advent::new(2024, 1, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part1(input), 11);
    assert_eq!(part2(input), 31);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1722302);
    assert_eq!(part2(input), 20373490);
}
