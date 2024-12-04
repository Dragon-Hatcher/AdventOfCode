use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 05)
}

fn reduce(molecule: &str) -> Option<String> {
    let mut new = String::new();
    let mut prev: Option<char>  = None;

    for c in molecule.chars() {
        if let Some(p) = prev {
            if p.is_ascii_lowercase() != c.is_ascii_lowercase() && p.to_ascii_uppercase() == c.to_ascii_uppercase() {
                new.pop();
                prev = None;
                continue;
            }
        }

        new.push(c);
        prev = Some(c);
    }

    (molecule != new).then_some(new)
}

fn part1(input: &str) -> i64 {
    let mut molecule = input.trim().to_owned();
    while let Some(next) = reduce(&molecule) {
        molecule = next;
    }

    molecule.len() as i64
}

fn part2(input: &str) -> i64 {
    let types = input.chars().map(|c| c.to_ascii_lowercase()).unique();
    types
        .map(|c| {
            let removed = input.replace([c, c.to_ascii_uppercase()], "");
            part1(&removed)
        })
        .min()
        .unwrap_or_default()
}

fn main() {
    advent::new(2018, 5, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "dabAcCaCBAcCcaDA";
    assert_eq!(part1(input), 10);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 9172);
    assert_eq!(part2(input), 6550);
}
