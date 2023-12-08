use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 08)
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Left,
    Right,
}

fn parse(input: &str) -> (Vec<Side>, HashMap<String, (String, String)>) {
    let (instructions, paths_str) = input.sections().tup();

    let instructions = instructions
        .chars()
        .map(|c| if c == 'L' { Side::Left } else { Side::Right })
        .collect_vec();

    let paths: HashMap<_, _> = paths_str
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" = (").unwrap();
            let (l, r) = to.split_once(", ").unwrap();
            let r = r.strip_suffix(')').unwrap();

            (from.to_owned(), (l.to_owned(), r.to_owned()))
        })
        .collect();

    (instructions, paths)
}

fn iters_to(
    instructions: &[Side],
    paths: &HashMap<String, (String, String)>,
    start: &str,
    is_end: impl Fn(&str) -> bool,
) -> i64 {
    let mut loc = start.to_owned();
    let mut steps = 0;

    loop {
        loc = match instructions[steps as usize % instructions.len()] {
            Side::Left => paths[&loc].0.clone(),
            Side::Right => paths[&loc].1.clone(),
        };
        steps += 1;

        if is_end(&loc) {
            return steps;
        }
    }
}

fn part1(input: &str) -> i64 {
    let (instructions, paths) = parse(input);
    iters_to(&instructions, &paths, "AAA", |l| l == "ZZZ")
}

fn part2(input: &str) -> i64 {
    let (instructions, paths) = parse(input);
    paths
        .keys()
        .filter(|l| l.ends_with('A'))
        .map(|start| iters_to(&instructions, &paths, start, |l| l.ends_with('Z')))
        .reduce(lcm)
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(
        part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
        ),
        6
    );
    assert_eq!(
        part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
        ),
        6
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 17263);
    assert_eq!(part2(input), 14631604759649);
}
