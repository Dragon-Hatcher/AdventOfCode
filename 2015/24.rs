use advent::prelude::*;

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

fn find_configs(packages: &[i64], so_far: &mut Vec<i64>, weight_left: i64) -> Option<PackageGroup> {
    if weight_left < 0 {
        None
    } else if packages.is_empty() {
        if weight_left == 0 {
            Some(PackageGroup(so_far.clone()))
        } else {
            None
        }
    } else {
        so_far.push(packages[0]);
        let with = find_configs(&packages[1..], so_far, weight_left - packages[0]);
        so_far.pop();
        let without = find_configs(&packages[1..], so_far, weight_left);

        match (with, without) {
            (None, None) => None,
            (None, Some(p)) => Some(p),
            (Some(p), None) => Some(p),
            (Some(a), Some(b)) => Some(a.min(b)),
        }
    }
}

fn part1(input: &str) -> i64 {
    let packages = input.nums().collect_vec();
    let target_weight = packages.iter().sum::<i64>() / 3;
    let mut so_far = Vec::new();
    find_configs(&packages, &mut so_far, target_weight).unwrap().qe()
}

fn part2(input: &str) -> i64 {
    let packages = input.nums().collect_vec();
    let target_weight = packages.iter().sum::<i64>() / 4;
    let mut so_far = Vec::new();
    find_configs(&packages, &mut so_far, target_weight).unwrap().qe()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
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
