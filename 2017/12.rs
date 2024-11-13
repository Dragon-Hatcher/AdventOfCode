use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 12)
}

fn parse(input: &str) -> HashMap<i64, Vec<i64>> {
    input
    .lines()
    .map(|l| {
        let mut nums = l.nums_pos();
        let from = nums.nu();
        let rest = nums.collect_vec();
        (from, rest)
    })
    .collect()
}

fn find_group(connections: &HashMap<i64, Vec<i64>>, seed: i64) -> HashSet<i64> {
    let mut edge = HashSet::new();
    let mut seen = HashSet::new();
    edge.insert(seed);
    seen.insert(seed);

    while !edge.is_empty() {
        edge = edge
            .iter()
            .flat_map(|id| connections.get(id).unwrap())
            .copied()
            .filter(|id| !seen.contains(id))
            .collect();
        seen.extend(&edge);
    }

    seen
}

fn part1(input: &str) -> i64 {
    let connections = parse(input);
    find_group(&connections, 0).len() as i64
}

fn part2(input: &str) -> i64 {
    let connections = parse(input);
    let mut unseen: HashSet<i64> = connections.keys().copied().collect();
    let mut groups = 0;

    while let Some(next) = unseen.pop() {
        let group = find_group(&connections, next);
        unseen.retain(|el| !group.contains(el));
        groups += 1;
    }

    groups
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
    assert_eq!(part1(input), 6);
    assert_eq!(part2(input), 2);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 130);
    assert_eq!(part2(input), 189);
}
