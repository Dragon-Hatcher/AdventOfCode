use advent::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

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
    // I looked at the solution thread to solve this one

    let (rule_lines, goal_mol) = input.sections().tup();
    let mut rules = rule_lines
        .lines()
        .map(|l| l.split_once(" => ").unwrap())
        .collect_vec();

    rules.sort_by_key(|(_, m)| -(m.len() as i64));

    fn try_solution(molecule: &str, rules: &[(&str, &str)]) -> Option<i64> {
        let mut molecule = molecule.to_owned();
        let mut steps = 0;

        'outer: loop {
            if molecule == "e" {
                return Some(steps);
            }

            for (replacement, pattern) in rules {
                if molecule.contains(pattern) {
                    molecule = molecule.replacen(pattern, replacement, 1);
                    steps += 1;
                    continue 'outer;
                }
            }

            return None;
        }
    }

    let mut rng = thread_rng();

    loop {
        if let Some(soln) = try_solution(goal_mol.trim(), &rules) {
            return soln;
        }

        rules.shuffle(&mut rng);
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 576);
    assert_eq!(part2(input), 207);
}
