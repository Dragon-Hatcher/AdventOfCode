use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 01)
}

fn parse(input: &str) -> impl Iterator<Item = (Turn, i64)> + '_ {
    input.split(", ").map(|d| {
        let turn = Turn::from_char(d.chars().nu());
        let dist = d.nums().nu();
        (turn, dist)
    })
}

fn part1(input: &str) -> i64 {
    let directions = parse(input);

    let mut facing = Direction::Up;
    let mut pos = Vector2::ZERO;

    for (turn, dist) in directions {
        facing = facing.turn(turn);
        pos += facing.vector() * dist;
    }

    pos.manhatan_mag()
}

fn part2(input: &str) -> i64 {
    let directions = parse(input);

    let mut facing = Direction::Up;
    let mut pos = Vector2::ZERO;
    let mut visited = FxHashSet::default();
    visited.insert(pos);

    for (turn, dist) in directions {
        facing = facing.turn(turn);
        for _ in 0..dist {
            pos += facing.vector();
            if visited.contains(&pos) {
                return pos.manhatan_mag();
            }
            visited.insert(pos);
        }
    }

    panic!()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("R5, L5, R5, R3"), 12);
    assert_eq!(part2("R8, R4, R4, R8"), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 226);
    assert_eq!(part2(input), 79);
}
