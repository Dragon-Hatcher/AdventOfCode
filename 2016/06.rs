use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 06)
}

fn get_counts(input: &str) -> Vec<FxHashMap<char, i64>> {
    let width = input.lines().nu().len();
    let mut counts = vec![FxHashMap::<char, i64>::default(); width];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            *counts[i].entry(c).or_default() += 1;
        }
    }

    counts
}

fn part1(input: &str) -> String {
    get_counts(input)
        .iter()
        .map(|m| m.iter().max_by_key(|(_, c)| **c).unwrap().0)
        .collect()
}

fn part2(input: &str) -> String {
    get_counts(input)
        .iter()
        .map(|m| m.iter().min_by_key(|(_, c)| **c).unwrap().0)
        .collect()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";
    assert_eq!(part1(input), "easter");
    assert_eq!(part2(input), "advent");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "tkspfjcc");
    assert_eq!(part2(input), "xrlmbypn");
}
