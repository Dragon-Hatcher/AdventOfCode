use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2025 / 05)
}

fn part1(input: &str) -> i64 {
    let (ranges, ingredients) = input.sections().tup();
    let ranges: Vec<(i64, i64)> = ranges.nums().tuples().collect();
    let ingredients = ingredients.nums();

    ingredients
        .filter(|id| ranges.iter().any(|(low, high)| low <= id && id <= high))
        .count() as i64
}

fn part2(input: &str) -> i64 {
    let (ranges, _) = input.sections().tup();
    let mut ranges: Vec<(i64, i64)> = ranges.nums().tuples().collect();
    ranges.sort_by_key(|(low, _high)| *low);

    let (mut low, mut high) = ranges[0];
    let mut count = 0;
    for &(new_low, new_high) in &ranges[1..] {
        if new_low <= high {
            high = high.max(new_high);
        } else {
            count += high - low + 1;
            low = new_low;
            high = new_high;
        }
    }
    count += high - low + 1;

    count
}

fn main() {
    advent::new(2025, 5, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    assert_eq!(part1(input), 3);
    assert_eq!(part2(input), 14);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 737);
    assert_eq!(part2(input), 357485433193284);
}
