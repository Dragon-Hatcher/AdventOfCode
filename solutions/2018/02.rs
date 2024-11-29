use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 02)
}

fn part1(input: &str) -> i64 {
    let twos = input
        .lines()
        .filter(|l| l.chars().counts().values().any(|v| *v == 2))
        .count() as i64;

    let threes = input
        .lines()
        .filter(|l| l.chars().counts().values().any(|v| *v == 3))
        .count() as i64;

    twos * threes
}

fn part2(input: &str) -> String {
    fn compare(a: &str, b: &str) -> Option<String> {
        let common: String = a
            .chars()
            .zip(b.chars())
            .filter(|(ac, bc)| ac == bc)
            .map(|(ac, _)| ac)
            .collect();

        (common.len() == a.len() - 1).then_some(common)
    }

    input
        .lines()
        .tuple_combinations()
        .filter_map(|(a, b)| compare(a, b))
        .nu()
}

fn main() {
    advent::new(2018, 2, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 3952);
    assert_eq!(part2(input), "vtnikorkulbfejvyznqgdxpaw");
}
