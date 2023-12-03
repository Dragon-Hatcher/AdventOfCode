use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 19)
}

fn part1(input: &str) -> i64 {
    let (replacements, start_mol) = input.sections().tup();
    let mut molecules = FxHashSet::default();

    for line in replacements.lines() {
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
    todo!();
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 576);
    assert_eq!(part2(input), 0);
}
