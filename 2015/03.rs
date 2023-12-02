use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 03)
}

fn parse_move(c: char) -> Vector2 {
    match c {
        '>' => Vector2::E1,
        '<' => -Vector2::E1,
        '^' => Vector2::E2,
        'v' => -Vector2::E2,
        _ => Vector2::ZERO,
    }
}

fn part1(input: &str) -> i64 {
    let mut position = Vector2::ZERO;
    let mut visited = FxHashSet::default();
    visited.insert(position);

    for m in input.chars().map(parse_move) {
        position += m;
        visited.insert(position);
    }

    visited.len() as i64
}

fn part2(input: &str) -> i64 {
    let mut santa_pos = Vector2::ZERO;
    let mut robo_pos = Vector2::ZERO;
    let mut santa = true;

    let mut visited = FxHashSet::default();
    visited.insert(santa_pos);

    for m in input.chars().map(parse_move) {
        if santa {
            santa_pos += m;
            visited.insert(santa_pos);
        } else {
            robo_pos += m;
            visited.insert(robo_pos);
        }
        santa = !santa;
    }

    visited.len() as i64
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "^v^v^v^v^v";
    assert_eq!(part1(input), 2);
    assert_eq!(part2(input), 11);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 2592);
    assert_eq!(part2(input), 2360);
}
