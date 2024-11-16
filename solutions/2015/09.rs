use advent::prelude::*;

fn parse_line(str: &str) -> (String, String, i64) {
    let (locs, dist) = str.split_once(" = ").unwrap();
    let (from, to) = locs.split_once(" to ").unwrap();
    (from.to_owned(), to.to_owned(), dist.parse().unwrap())
}

fn parse(str: &str) -> (HashSet<String>, HashMap<(String, String), i64>) {
    let mut distances = HashMap::default();
    let mut locs = HashSet::default();
    for (from, to, dist) in str.lines().map(parse_line) {
        locs.insert(from.clone());
        locs.insert(to.clone());
        distances.insert((from.clone(), to.clone()), dist);
        distances.insert((to, from), dist);
    }
    (locs, distances)
}

type Input = (HashSet<String>, HashMap<(String, String), i64>);

fn default_input() -> Input {
    parse(include_input!(2015 / 09))
}

fn all_distances((locs, distances): &Input) -> impl Iterator<Item = i64> + '_ {
    let count = locs.len();
    locs.into_iter().permutations(count).map(|path| {
        path.iter()
            .tuple_windows()
            .map(|(&from, &to)| distances[&(from.clone(), to.clone())])
            .sum()
    })
}

fn part1(input: Input) -> i64 {
    all_distances(&input).min().unwrap_or_default()
}

fn part2(input: Input) -> i64 {
    all_distances(&input).max().unwrap_or_default()
}

fn main() {
    advent::new(2015, 09, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
    assert_eq!(part1(parse(input)), 605);
    assert_eq!(part2(parse(input)), 982);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 251);
    assert_eq!(part2(input), 898);
}
