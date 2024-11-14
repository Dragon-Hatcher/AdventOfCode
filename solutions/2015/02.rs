use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 02)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (l, w, h) = line.nums().tup();
            let a = l * w;
            let b = w * h;
            let c = h * l;
            2 * (a + b + c) + a.min(b).min(c)
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (l, w, h) = line.nums().tup();
            let volume = l * w * h;
            let perimeter = (l * 2 + w * 2).min(l * 2 + h * 2).min(w * 2 + h * 2);

            volume + perimeter
        })
        .sum()
}

fn main() {
    advent::new(2015, 02, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "2x3x4";
    assert_eq!(part1(input), 58);
    assert_eq!(part2(input), 34);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1586300);
    assert_eq!(part2(input), 3737498);
}
