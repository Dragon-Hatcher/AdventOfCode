use std::collections::HashMap;

use advent::prelude::*;
use maplit::hashmap;

fn default_input() -> &'static str {
    include_input!(2016 / 02)
}

fn solve(input: &str, keys: &HashMap<Vector2, char>) -> String {
    let mut code = "".to_owned();

    let mut pos = Vector2::new(0, 0);
    for line in input.lines() {
        for dir in line.chars() {
            let next = pos + Direction::from_char(dir).vector();
            if keys.contains_key(&next) {
                pos = next;
            }
        }
        code.push(keys[&pos]);
    }

    code
}

fn part1(input: &str) -> String {
    let keys = hashmap!(
        Vector2::new(-1, 1) => '1',
        Vector2::new(-1, 0) => '4',
        Vector2::new(-1, -1) => '7',
        Vector2::new(0, 1) => '2',
        Vector2::new(0, 0) => '5',
        Vector2::new(0, -1) => '8',
        Vector2::new(1, 1) => '3',
        Vector2::new(1, 0) => '6',
        Vector2::new(1, -1) => '9',
    );

    solve(input, &keys)
}

fn part2(input: &str) -> String {
    let keys = hashmap!(
        Vector2::new(0, 0) => '5',
        Vector2::new(1, 0) => '6',
        Vector2::new(1, 1) => '2',
        Vector2::new(1, -1) => 'A',
        Vector2::new(2, 0) => '7',
        Vector2::new(2, 1) => '3',
        Vector2::new(2, 2) => '1',
        Vector2::new(2, -1) => 'B',
        Vector2::new(2, -2) => 'D',
        Vector2::new(3, 0) => '8',
        Vector2::new(3, 1) => '4',
        Vector2::new(3, -1) => 'C',
        Vector2::new(4, 0) => '9',
    );

    solve(input, &keys)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "ULL
RRDDD
LURDL
UUUUD";
    assert_eq!(part1(input), "1985");
    assert_eq!(part2(input), "5DB3");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "95549");
    assert_eq!(part2(input), "D87AD");
}
