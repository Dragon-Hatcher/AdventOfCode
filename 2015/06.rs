use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 06)
}

enum Action {
    On,
    Off,
    Toggle,
}

struct Instruction {
    action: Action,
    range: Range,
}

fn parse_line(str: &str) -> Instruction {
    let action = if str.starts_with("turn on") {
        Action::On
    } else if str.starts_with("turn off") {
        Action::Off
    } else {
        Action::Toggle
    };

    let (x1, y1, x2, y2) = str.nums().tup();
    let top_left = Vector2::new(x1, y1);
    let bottom_right = Vector2::new(x2, y2);

    Instruction {
        action,
        range: Range::new_bl_tr(top_left, bottom_right),
    }
}

fn part1(input: &str) -> i64 {
    let mut grid = Grid::new(1000, 1000, false);

    for Instruction { action, range } in input.lines().map(parse_line) {
        match action {
            Action::On => grid.fill_range(true, range),
            Action::Off => grid.fill_range(false, range),
            Action::Toggle => grid.fill_range_with(|_, on| !on, range),
        }
    }

    grid.elements().filter(|b| **b).count() as i64
}

fn part2(input: &str) -> i64 {
    let mut grid = Grid::new(1000, 1000, 0);

    for Instruction { action, range } in input.lines().map(parse_line) {
        let brightness_change = match action {
            Action::On => 1,
            Action::Off => -1,
            Action::Toggle => 2,
        };

        grid.fill_range_with(|_, b| (b + brightness_change).max(0), range);
    }

    grid.elements().sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(
        part1(
            "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500"
        ),
        998996
    );
    assert_eq!(
        part2(
            "turn on 0,0 through 0,0
toggle 0,0 through 999,999
turn off 0,0 through 0,0
turn off 0,0 through 0,0
turn off 0,0 through 0,0
turn off 0,0 through 0,0"
        ),
        1999998
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 377891);
    assert_eq!(part2(input), 14110788);
}
