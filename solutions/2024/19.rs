use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 19)
}

fn can_do(design: &[char], with: &[Vec<char>]) -> bool {
    if design.is_empty() {
        return true;
    }

    for towel in with {
        if design.starts_with(&towel) && can_do(&design[towel.len()..], with) {
            return true;
        }
    }

    false
}

fn part1(input: &str) -> i64 {
    let (towels, designs) = input.sections().tup();
    let towels = towels.split(", ").map(|t| t.trim().chars().collect_vec()).collect_vec();
    let designs = designs.lines().map(|l| l.trim().chars().collect_vec());

    designs
        .filter(|d| can_do(d, &towels))
        .count()
        as i64
}

#[memoize]
fn ways(design: Vec<char>, with: Vec<Vec<char>>) -> i64 {
    if design.is_empty() {
        return 1;
    }

    let mut sum = 0;

    for towel in with.iter() {
        if design.starts_with(&towel)  {
            sum += ways(design[towel.len()..].to_owned(), with.clone());
        }
    }

    sum
}

fn part2(input: &str) -> i64 {
    let (towels, designs) = input.sections().tup();
    let towels = towels.split(", ").map(|t| t.trim().chars().collect_vec()).collect_vec();
    let designs = designs.lines().map(|l| l.trim().chars().collect_vec());

    designs
        .map(|d| ways(d, towels.clone()))
        .sum()
}

fn main() {
    advent::new(2024, 19, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    // assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
