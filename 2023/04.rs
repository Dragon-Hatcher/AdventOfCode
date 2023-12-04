use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 04)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (_, rest) = l.split_once(": ").unwrap();
            let (a, b) = rest.split_once(" | ").unwrap();
            let a: FxHashSet<i64> = a.nums().collect();
            let b: FxHashSet<i64> = b.nums().collect();
            let t = a.intersection(&b).count() as u32;
            2_i64.pow(t - 1)
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let wins = input
        .lines()
        .map(|l| {
            let (_, rest) = l.split_once(": ").unwrap();
            let (a, b) = rest.split_once(" | ").unwrap();
            let a: FxHashSet<i64> = a.nums().collect();
            let b: FxHashSet<i64> = b.nums().collect();
            let t = a.intersection(&b).count() as u32;
            t        })
        .collect_vec();

    let mut counts = vec![1; wins.len()];

    for i in 0..wins.len() {
        for j in (i + 1)..(i + 1 + wins[i] as usize).min(wins.len()) {
            counts[j] += counts[i];
        }
    }

    counts.iter().sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 30);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 21568);
    assert_eq!(part2(input), 11827296);
}
