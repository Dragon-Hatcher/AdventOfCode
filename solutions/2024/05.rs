use std::cmp::Ordering;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 05)
}

fn part1(input: &str) -> i64 {
    let (rules, updates) = input.sections().tup();
    let rules: Vec<(i64, i64)> = rules.lines().map(|r| r.nums().tup()).collect_vec();

    let mut s = 0;
    'outer: for update in updates.lines() {
        let n = update.nums().collect_vec();
        for (l, r) in rules.iter() {
            if let Some(rp) = n.iter().find_position(|x| *x == r) {
                if let Some(lp) = n.iter().find_position(|x| *x == l) {
                    if lp > rp {
                        // dbg!(r, l);
                        continue 'outer;
                    }
                }
    
            }
        }
        // dbg!(update);
        s += n[n.len() / 2];
    }
    s
}

fn part2(input: &str) -> i64 {
    let (rules, updates) = input.sections().tup();
    let rules: Vec<(i64, i64)> = rules.lines().map(|r| r.nums().tup()).collect_vec();

    let mut s = 0;
    'outer: for update in updates.lines() {
        let mut invalid = false;

        let mut n = update.nums().collect_vec();
        for (l, r) in rules.iter() {
            if let Some(rp) = n.iter().find_position(|x| *x == r) {
                if let Some(lp) = n.iter().find_position(|x| *x == l) {
                    if lp > rp {
                        // dbg!(r, l);
                        invalid = true;
                    }
                }
    
            }
        }

        if !invalid { 
            continue;
        }

        n.sort_by(|x, y| {
            if rules.contains(&(*x, *y)) {
                Ordering::Less
            } else if rules.contains(&(*y, *x)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        // dbg!(update);
        s += n[n.len() / 2];
    }
    s
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
