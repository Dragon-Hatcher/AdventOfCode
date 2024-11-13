use advent::prelude::*;
use std::convert::identity;

fn default_input() -> &'static str {
    include_input!(2023 / 14)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Wall,
    Rock,
}

fn parse_cell(c: char) -> Cell {
    match c {
        '#' => Cell::Wall,
        'O' => Cell::Rock,
        _ => Cell::Empty,
    }
}

fn calc_weight(grid: &Grid<Cell>) -> i64 {
    grid.points()
        .filter(|&p| grid[p] == Cell::Rock)
        .map(|p| grid.height() - p.y)
        .sum()
}

fn tilt(grid: &mut Grid<Cell>, f: impl Fn(Vector2) -> Vector2) {
    for x in 0..grid.width() {
        let mut valid_y = 0;
        for y in 0..grid.height() {
            let p = f(Vector2::new(x, y));
            match grid[p] {
                Cell::Empty => {}
                Cell::Wall => valid_y = y + 1,
                Cell::Rock => {
                    grid[p] = Cell::Empty;
                    grid[f(Vector2::new(x, valid_y))] = Cell::Rock;
                    valid_y += 1;
                }
            }
        }
    }
}

fn cycle(grid: &mut Grid<Cell>) {
    let size = grid.width();

    tilt(grid, identity); // north
    tilt(grid, |v| Vector2::new(v.y, v.x)); // west
    tilt(grid, |v| Vector2::new(v.x, size - v.y - 1)); // south
    tilt(grid, |v| Vector2::new(size - v.y - 1, v.x)); // east
}

fn part1(input: &str) -> i64 {
    let mut grid = Grid::new_by_char(input, parse_cell);
    tilt(&mut grid, identity);
    calc_weight(&grid)
}

fn part2(input: &str) -> i64 {
    const TARGET_IDX: usize = 1000000000;

    let mut grid = Grid::new_by_char(input, parse_cell);

    let mut seen = HashMap::new();
    let mut idx = 0;

    while idx < TARGET_IDX {
        if let Some(prev_idx) = seen.get(&grid) {
            let diff = idx - prev_idx;
            idx += ((TARGET_IDX - idx) / diff) * diff;
        }

        seen.insert(grid.clone(), idx);
        cycle(&mut grid);
        idx += 1;
    }

    calc_weight(&grid)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(part1(input), 136);
    assert_eq!(part2(input), 64);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 109345);
    assert_eq!(part2(input), 112452);
}
