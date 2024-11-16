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
    let action = match str {
        str if str.starts_with("turn on") => Action::On,
        str if str.starts_with("turn off") => Action::Off,
        _ => Action::Toggle,
    };

    let (x1, y1, x2, y2) = str.nums().tup();
    let tl = v2(x1, y1);
    let br = v2(x2, y2);

    Instruction {
        action,
        range: Range::new_tl_br(tl, br),
    }
}

fn part1(input: &str) -> i64 {
    let mut grid = Grid::new_homogenous(1000, 1000, false);

    for Instruction { action, range } in input.lines().map(parse_line) {
        match action {
            Action::On => grid.fill_range(range, true),
            Action::Off => grid.fill_range(range, false),
            Action::Toggle => grid.fill_range_with(range, |_, on| !on),
        }
    }

    grid.elements().filter(|&&b| b).count() as i64
}

fn part2(input: &str) -> i64 {
    let mut grid = Grid::new_homogenous(1000, 1000, 0);

    for Instruction { action, range } in input.lines().map(parse_line) {
        let bd = match action {
            Action::On => 1,
            Action::Off => -1,
            Action::Toggle => 2,
        };

        grid.fill_range_with(range, |_, b| (b + bd).max(0));
    }

    grid.elements().sum()
}

fn main() {
    advent::new(2015, 6, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input1 = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
";
    assert_eq!(part1(input1), 998996);

    let input2 = "turn on 0,0 through 0,0
toggle 0,0 through 999,999
turn off 0,0 through 0,0
turn off 0,0 through 0,0
turn off 0,0 through 0,0
turn off 0,0 through 0,0";
    assert_eq!(part2(input2), 1999998);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 377891);
    assert_eq!(part2(input), 14110788);
}
