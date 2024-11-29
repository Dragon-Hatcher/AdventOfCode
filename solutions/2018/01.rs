use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 01)
}

fn part1(input: &str) -> i64 {
    input.nums().sum()
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;

    let mut seen = HashSet::default();
    seen.insert(sum);

    loop {
        for n in input.nums() {
            sum += n;
            if !seen.insert(sum) {
                return sum;
            }
        }
    }
}

fn main() {
    advent::new(2018, 1, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "+1, -2, +3, +1";
    assert_eq!(part1(input), 3);
    assert_eq!(part2(input), 2);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 585);
    assert_eq!(part2(input), 83173);
}
