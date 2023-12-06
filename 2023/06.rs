use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 06)
}

fn ways_to_win(time: i64, dist: i64) -> i64 {
    let t = time as f64;
    let d = dist as f64;

    // If charging time is c distance is given by c * (t - c).
    // We want the distance to be greater than d so -c^2 + tc -d > 0.
    // Use the quadratic formula to obtain that (t - √t^2 - 4*d) / 2 < c < (t - √t^2 - 4*d) / 2

    let root = (t*t - 4.0*d).sqrt();
    let upper = (t + root) / 2.0; 
    let lower = (t - root) / 2.0; 

    upper.ceil() as i64 - lower.floor() as i64 - 1
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
