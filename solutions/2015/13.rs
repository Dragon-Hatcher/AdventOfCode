use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 13)
}

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

fn parse(input: &str) -> (HashSet<Id>, HashMap<(Id, Id), i64>) {
    let mut ids = IdGen::new();
    let mut happiness = HashMap::default();

    for (name1, score, name2) in input.lines().map(parse_line) {
        let (id1, id2) = (ids.get_id(&name1), ids.get_id(&name2));
        happiness.insert((id1, id2), score);
    }

    (ids.all_ids().collect(), happiness)
}

fn part1(input: &str) -> i64 {
    let (names, happiness) = parse(input);
    let count = names.len();

    names
        .iter()
        .permutations(count)
        .map(|seats| {
            seats
                .iter()
                .circular_tuple_windows()
                .map(|(&a, &b)| happiness[&(*a, *b)] + happiness[&(*b, *a)])
                .sum()
        })
        .max()
        .unwrap_or_default()
}

fn part2(input: &str) -> i64 {
    let (names, happiness) = parse(input);
    let count = names.len();

    names
        .iter()
        .permutations(count)
        .map(|seats| {
            seats
                .iter()
                .tuple_windows()
                .map(|(&a, &b)| happiness[&(*a, *b)] + happiness[&(*b, *a)])
                .sum()
        })
        .max()
        .unwrap_or_default()
}

fn main() {
    advent::new(2015, 13, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "Alice would gain 54 happiness units by sitting next to Bob.
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
David would gain 41 happiness units by sitting next to Carol.";
    assert_eq!(part1(input), 330);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 709);
    assert_eq!(part2(input), 668);
}
