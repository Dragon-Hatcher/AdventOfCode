use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 06)
}

fn ways_to_win(time: i64, dist: i64) -> i64 {
    (0..time)
        .filter(|charging| {
            let moving = time - charging;
            charging * moving > dist
        })
        .count() as i64
}

fn part1(input: &str) -> i64 {
    let (time, dist) = input.lines().tup();
    let times = time.nums();
    let dists = dist.nums();

    times
        .zip(dists)
        .map(|(time, dist)| ways_to_win(time, dist))
        .product()
}

fn part2(input: &str) -> i64 {
    let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let (time, dist) = input.nums().tup();
    ways_to_win(time, dist)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "Time:      7  15   30
    Distance:  9  40  200";
    assert_eq!(part1(input), 288);
    assert_eq!(part2(input), 71503);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 800280);
    assert_eq!(part2(input), 45128024);
}
