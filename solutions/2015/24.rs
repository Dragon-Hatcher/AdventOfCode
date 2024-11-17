use advent::prelude::*;
use std::cmp::Ordering;

fn default_input() -> &'static str {
    include_input!(2015 / 24)
}

struct PackageGroup(Vec<i64>);

impl PackageGroup {
    fn size(&self) -> i64 {
        self.0.len() as i64
    }

    fn qe(&self) -> i64 {
        self.0.iter().product()
    }
}

impl PartialEq for PackageGroup {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size() && self.qe() == other.qe()
    }
}
impl Eq for PackageGroup {}

impl PartialOrd for PackageGroup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PackageGroup {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size()
            .cmp(&other.size())
            .then(self.qe().cmp(&other.qe()))
    }
}

fn find_configs(packages: &[i64], weight_left: i64) -> PackageGroup {
    fn helper(packages: &[i64], so_far: &mut Vec<i64>, weight_left: i64) -> Option<PackageGroup> {
        if weight_left < 0 {
            return None;
        }

        if packages.is_empty() {
            return (weight_left == 0).then(|| PackageGroup(so_far.clone()));
        }

        so_far.push(packages[0]);
        let with = helper(&packages[1..], so_far, weight_left - packages[0]);
        so_far.pop();
        let without = helper(&packages[1..], so_far, weight_left);

        match (with, without) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }

    let mut so_far = vec![];
    helper(&packages, &mut so_far, weight_left).unwrap()
}

fn part1(input: &str) -> i64 {
    let packages = input.nums().collect_vec();
    let target_weight = packages.iter().sum::<i64>() / 3;
    find_configs(&packages, target_weight).qe()
}

fn part2(input: &str) -> i64 {
    let packages = input.nums().collect_vec();
    let target_weight = packages.iter().sum::<i64>() / 4;
    find_configs(&packages, target_weight).qe()
}

fn main() {
    advent::new(2015, 24, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "1 2 3 4 5 7 8 9 10 11";
    assert_eq!(part1(input), 99);
    assert_eq!(part2(input), 44);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 11266889531);
    assert_eq!(part2(input), 77387711);
}
