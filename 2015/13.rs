use advent::prelude::*;

fn parse_line(str: &str) -> (String, i64, String) {
    let (name1, rest) = str.split_once(" would ").unwrap();
    let (ty, rest) = rest.split_once(' ').unwrap();
    let (cnt, name2) = rest
        .split_once(" happiness units by sitting next to ")
        .unwrap();
    let name2 = name2.strip_suffix('.').unwrap();
    let cnt = cnt.parse::<i64>().unwrap() * if ty == "gain" { 1 } else { -1 };

    (name1.to_owned(), cnt, name2.to_owned())
}

fn parse(input: &str) -> (FxHashSet<String>, FxHashMap<(String, String), i64>) {
    let mut names = FxHashSet::default();
    let mut happiness = FxHashMap::default();

    for (name1, score, name2) in input.lines().map(parse_line) {
        names.insert(name1.clone());
        names.insert(name2.clone());
        happiness.insert((name1, name2), score);
    }

    (names, happiness)
}

fn default_input() -> (FxHashSet<String>, FxHashMap<(String, String), i64>) {
    parse(include_input!(2015 / 13))
}

fn part1((people, happiness): (FxHashSet<String>, FxHashMap<(String, String), i64>)) -> i64 {
    let cnt = people.len();
    people
        .into_iter()
        .permutations(cnt)
        .map(|seats| {
            seats
                .iter()
                .circular_tuple_windows()
                .map(|(a, b)| {
                    happiness[&(a.clone(), b.clone())] + happiness[&(b.clone(), a.clone())]
                })
                .sum()
        })
        .max()
        .unwrap_or_default()
}

fn part2((people, happiness): (FxHashSet<String>, FxHashMap<(String, String), i64>)) -> i64 {
    let cnt = people.len();
    people
        .into_iter()
        .permutations(cnt)
        .map(|seats| {
            seats
                .iter()
                .tuple_windows()
                .map(|(a, b)| {
                    happiness[&(a.clone(), b.clone())] + happiness[&(b.clone(), a.clone())]
                })
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
    let input = parse(
        "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.",
    );
    assert_eq!(part1(input), 330);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 709);
    assert_eq!(part2(input), 668);
}
