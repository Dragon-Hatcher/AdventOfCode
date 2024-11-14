use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 03)
}

fn parse_move(c: char) -> Vec2 {
    match c {
        '>' => v2(1, 0),
        '<' => v2(-1, 0),
        '^' => v2(0, 1),
        'v' => v2(0, -1),
        _ => v2(0, 0),
    }
}

fn part1(input: &str) -> i64 {
    let mut pos = v2(0, 0);
    let mut visited = HashSet::default();
    visited.insert(pos);

    for m in input.chars().map(parse_move) {
        pos += m;
        visited.insert(pos);
    }

    visited.len() as i64
}

fn part2(input: &str) -> i64 {
    let mut santa_pos = v2(0, 0);
    let mut robot_pos = v2(0, 0);
    let mut visited = HashSet::default();

    for (m1, m2) in input.chars().map(parse_move).tuples() {
        santa_pos += m1;
        robot_pos += m2;

        visited.insert(santa_pos);
        visited.insert(robot_pos);
    }

    visited.len() as i64
}

fn main() {
    advent::new(2015, 03, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "^>v<";
    assert_eq!(part1(input), 4);
    assert_eq!(part2(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 2592);
    assert_eq!(part2(input), 2360);
}
