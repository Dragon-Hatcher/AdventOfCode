use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 19)
}

fn part1(input: &str) -> i64 {
    let (rules, start_mol) = input.sections().tup();
    let mut molecules = HashSet::default();

    for line in rules.lines() {
        let (pattern, replacement) = line.split_once(" => ").unwrap();

        for (start, matched) in start_mol.match_indices(pattern) {
            let range = start..start + matched.as_bytes().len();
            let mut molecule = start_mol.to_owned();
            molecule.replace_range(range, replacement);
            molecules.insert(molecule);
        }
    }

    molecules.len() as i64
}

fn part2(input: &str) -> i64 {
    let (_, start_mol) = input.sections().tup();

    // https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4etju

    (start_mol.chars().filter(char::is_ascii_uppercase).count()
        - start_mol.matches("Rn").count()
        - start_mol.matches("Ar").count()
        - 2 * start_mol.matches("Y").count()
        - 1) as i64
}

fn main() {
    advent::new(2015, 19, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "H => HO
H => OH
O => HH

HOHOHO";
    assert_eq!(part1(input), 7);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 576);
    assert_eq!(part2(input), 207);
}
