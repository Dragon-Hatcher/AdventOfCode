use advent::prelude::*;
use std::cmp::Ordering;

fn default_input() -> &'static str {
    include_input!(2024 / 05)
}

fn parse(input: &str) -> (impl Iterator<Item = Vec<i64>> + '_, Vec<(i64, i64)>) {
    let (rules, updates) = input.sections().tup();
    let rules = rules.lines().map(|r| r.nums().tup()).collect_vec();
    let updates = updates.lines().map(|l| l.nums().collect_vec());
    (updates, rules)
}

fn is_valid(update: &[i64], rules: &[(i64, i64)]) -> bool {
    for (less, greater) in rules {
        let Some(less_pos) = update.iter().find_position(|x| *x == less) else {
            continue;
        };
        let Some(greater_pos) = update.iter().find_position(|x| *x == greater) else {
            continue;
        };

        if less_pos > greater_pos {
            return false;
        }
    }

    true
}

fn part1(input: &str) -> i64 {
    let (updates, rules) = parse(input);

    updates
        .filter(|update| is_valid(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(input: &str) -> i64 {
    let (updates, rules) = parse(input);

    fn cmp_by_rules(a: i64, b: i64, rules: &[(i64, i64)]) -> Ordering {
        if rules.contains(&(a, b)) {
            Ordering::Less
        } else if rules.contains(&(b, a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    updates
        .filter(|update| !is_valid(update, &rules))
        .map(|mut update| {
            update.sort_by(|&a, &b| cmp_by_rules(a, b, &rules));
            update[update.len() / 2]
        })
        .sum()
}

fn main() {
    advent::new(2024, 5, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(part1(input), 143);
    assert_eq!(part2(input), 123);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 4957);
    assert_eq!(part2(input), 6938);
}
