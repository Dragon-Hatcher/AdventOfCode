use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 02)
}

fn solve(input: &str, keys: &HashMap<Vec2, char>) -> String {
    let mut code = "".to_owned();

    let mut pos = Vec2::ZERO;
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
        Vec2::new(-1, -1) => '1',
        Vec2::new(-1, 0) => '4',
        Vec2::new(-1, 1) => '7',
        Vec2::new(0, -1) => '2',
        Vec2::new(0, 0) => '5',
        Vec2::new(0, 1) => '8',
        Vec2::new(1, -1) => '3',
        Vec2::new(1, 0) => '6',
        Vec2::new(1, 1) => '9',
    );

    solve(input, &keys)
}

fn part2(input: &str) -> String {
    let keys = hashmap!(
        Vec2::new(0, 0) => '5',
        Vec2::new(1, 0) => '6',
        Vec2::new(1, -1) => '2',
        Vec2::new(1, 1) => 'A',
        Vec2::new(2, 0) => '7',
        Vec2::new(2, -1) => '3',
        Vec2::new(2, -2) => '1',
        Vec2::new(2, 1) => 'B',
        Vec2::new(2, 2) => 'D',
        Vec2::new(3, 0) => '8',
        Vec2::new(3, -1) => '4',
        Vec2::new(3, 1) => 'C',
        Vec2::new(4, 0) => '9',
    );

    solve(input, &keys)
}

fn main() {
    advent::new(2016, 2, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
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
