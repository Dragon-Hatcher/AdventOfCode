use advent::prelude::*;

fn parse_line(str: &str) -> (String, String, i64) {
    let (locs, dist) = str.split_once(" = ").unwrap();
    let (from, to) = locs.split_once(" to ").unwrap();
    (from.to_owned(), to.to_owned(), dist.parse().unwrap())
}

fn parse(str: &str) -> (FxHashSet<String>, FxHashMap<(String, String), i64>) {
    let mut distances = FxHashMap::default();
    let mut locs = FxHashSet::default();
    for (from, to, dist) in str.lines().map(parse_line) {
        locs.insert(from.clone());
        locs.insert(to.clone());
        distances.insert((from.clone(), to.clone()), dist);
        distances.insert((to, from), dist);
    }
    (locs, distances)
}

fn default_input() -> (FxHashSet<String>, FxHashMap<(String, String), i64>) {
    parse(include_input!(2015 / 09))
}

fn part1((locs, distances): (FxHashSet<String>, FxHashMap<(String, String), i64>)) -> i64 {
    let len = locs.len();
    locs.into_iter()
        .permutations(len)
        .map(|path| {
            path.iter()
                .tuple_windows()
                .map(|(from, to)| distances[&(from.clone(), to.clone())])
                .sum()
        })
        .min()
        .unwrap_or_default()
}

fn part2((locs, distances): (FxHashSet<String>, FxHashMap<(String, String), i64>)) -> i64 {
    let len = locs.len();
    locs.into_iter()
        .permutations(len)
        .map(|path| {
            path.iter()
                .tuple_windows()
                .map(|(from, to)| distances[&(from.clone(), to.clone())])
                .sum()
        })
        .max()
        .unwrap_or_default()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = parse("London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141");
    assert_eq!(part1(input.clone()), 605);
    assert_eq!(part2(input), 982);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 251);
    assert_eq!(part2(input), 898);
}
