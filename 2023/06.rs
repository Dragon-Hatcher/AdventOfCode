use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 06)
}

fn part1(input: &str) -> i64 {
    let (time, dist) = input.lines().tup();
    let times = time.nums().collect_vec();
    let dists = dist.nums().collect_vec();

    times
        .iter()
        .zip(dists.iter())
        .map(|(time, dist)| {
            let mut sum = 0;

            for charging in 0..*time {
                let moving = time - charging;
                if moving * charging > *dist {
                    sum += 1;
                }
            }
            sum

        })
        .product()

    // input
    //     .lines()
    //     .map(|l| {
    //         0
    //     })
    //     .sum()
}

fn part2(input: &str) -> i64 {
    let (time, dist) = input.lines().tup();
    let time: String = time.chars().filter(|c| !c.is_whitespace()).collect();
    let dist: String = dist.chars().filter(|c| !c.is_whitespace()).collect();
    let times = time.nums().collect_vec();
    let dists = dist.nums().collect_vec();

    times
        .iter()
        .zip(dists.iter())
        .map(|(time, dist)| {
            let mut sum = 0;

            for charging in 0..*time {
                let moving = time - charging;
                if moving * charging > *dist {
                    sum += 1;
                }
            }
            sum

        })
        .product()
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
