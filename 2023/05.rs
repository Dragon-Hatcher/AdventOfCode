use advent::prelude::*;
use std::ops::Range;

fn default_input() -> &'static str {
    include_input!(2023 / 05)
}

fn parse_rule(rule: &str) -> Rule {
    let (dest, match_start, len) = rule.nums().tup();
    Rule {
        m: match_start..match_start + len,
        dest,
    }
}

fn parse_ruleset(rules: &str) -> Vec<Vec<Rule>> {
    rules
        .sections()
        .map(|sec| sec.lines().skip(1).map(parse_rule).collect())
        .collect()
}

type SeedRange = Range<i64>;

#[derive(Debug, Clone)]
struct Rule {
    m: SeedRange,
    dest: i64,
}

impl Rule {
    fn cut(&self, seed: &SeedRange) -> (Option<SeedRange>, Option<SeedRange>) {
        let dest = self.dest;
        let m = self.m.clone();

        if seed.start >= m.start && seed.end <= m.end {
            // Full intersection
            (
                Some(dest + (seed.start - m.start)..dest + (seed.end - m.start)),
                None,
            )
        } else if seed.end <= m.start || seed.start >= m.end {
            // No intersection
            (None, Some(seed.clone()))
        } else if seed.start < m.start {
            // Left intersection
            (
                Some(dest..dest + (seed.end - m.start)),
                Some(seed.start..m.start),
            )
        } else {
            // Right intersection
            (
                Some(dest + (seed.start - m.start)..dest + (m.end - m.start)),
                Some(m.end..seed.end),
            )
        }
    }
}

fn solve(rule_sets: &[Vec<Rule>], mut seeds: Vec<SeedRange>) -> i64 {
    for rule_set in rule_sets {
        let mut new_ranges = Vec::default();
        let mut left_to_process = seeds;

        for rule in rule_set {
            let mut leftovers = Vec::default();

            for seed in &left_to_process {
                let (intersection, leftover) = rule.cut(seed);
                if let Some(intersection) = intersection {
                    new_ranges.push(intersection);
                }
                if let Some(leftover) = leftover {
                    leftovers.push(leftover);
                }
            }

            left_to_process = leftovers;
        }

        new_ranges.extend(left_to_process);
        seeds = new_ranges;
    }

    seeds.iter().map(|s| s.start).min().unwrap_or_default()
}

fn part1(input: &str) -> i64 {
    let (seeds, rules) = input.split_once("\n\n").unwrap();
    let seeds = seeds.nums().map(|start| start..start + 1).collect();
    let rules = parse_ruleset(rules);

    solve(&rules, seeds)
}

fn part2(input: &str) -> i64 {
    let (seeds, rules) = input.split_once("\n\n").unwrap();
    let seeds = seeds
        .nums()
        .tuples()
        .map(|(start, len)| start..start + len)
        .collect();
    let rules = parse_ruleset(rules);

    solve(&rules, seeds)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(part1(input), 35);
    assert_eq!(part2(input), 46);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 322500873);
    assert_eq!(part2(input), 108956227);
}
