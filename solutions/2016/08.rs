use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!("width = 50, height = 6\n\n" / 2016 / 08)
}

enum Instruction {
    On(Vec2),
    RotateRow(i64, i64),
    RotateCol(i64, i64),
}

fn parse_instruction(i: &str) -> Instruction {
    if let Some(range) = i.strip_prefix("rect") {
        let (w, h) = range.nums().tup();
        Instruction::On(v2(w, h))
    } else if let Some(shift) = i.strip_prefix("rotate row") {
        let (idx, by) = shift.nums().tup();
        Instruction::RotateRow(idx, by)
    } else {
        let (idx, by) = i.nums().tup();
        Instruction::RotateCol(idx, by)
    }
}

fn do_instruction(grid: &mut Grid<bool>, i: Instruction) {
    match i {
        Instruction::On(Vec2 { x, y }) => {
            grid.fill_range(Range::new_size(x, y), true);
        }
        Instruction::RotateRow(idx, by) => grid.rotate_right(grid.row_range(idx), by),
        Instruction::RotateCol(idx, by) => grid.rotate_down(grid.col_range(idx), by),
    }
}

fn parse(input: &str) -> (&str, Grid<bool>) {
    let (size, instructions) = input.sections().tup();
    let (width, height) = size.nums().tup();
    let grid = Grid::new_with(width, height, |_| false);
    (instructions, grid)
}

fn part1(input: &str) -> i64 {
    let (instructions, mut grid) = parse(input);

    for i in instructions.lines().map(parse_instruction) {
        do_instruction(&mut grid, i);
    }

    grid.elements().filter(|l| **l).count() as i64
}

fn part2(input: &str) -> String {
    let (instructions, mut grid) = parse(input);

    for i in instructions.lines().map(parse_instruction) {
        do_instruction(&mut grid, i);
    }

    grid.parse_str()
}

fn main() {
    advent::new(2016, 8, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "width = 7, height = 3

rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";
    assert_eq!(part1(input), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 123);
    assert_eq!(part2(input), "AFBUPZBJPS");
}
