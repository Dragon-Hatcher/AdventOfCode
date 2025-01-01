use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 02)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (min, max) = l.nums().minmax().into_option().unwrap();
            max - min
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            l.nums()
                .permutations(2)
                .find(|n| n[0] % n[1] == 0)
                .map(|n| n[0] / n[1])
                .unwrap()
        })
        .sum()
}

fn main() {
    advent::new(2017, 02, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    assert_eq!(part1("5 1 9 5 \n 7 5 3 \n 2 4 6 8"), 18);
    assert_eq!(part2("5 9 2 8 \n 9 4 7 3 \n 3 8 6 5"), 9);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 44887);
    assert_eq!(part2(input), 242);
}
