use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 18)
}

fn part1(input: &str) -> i64 {
    let mut grid = Grid::new_by_char(input, |c| c == '#');

    for _ in 0..100 {
        grid = Grid::new_with(grid.width(), grid.height(), |p| {
            let neighbors = grid.neighbors8(p).filter(|&p| grid[p]).count();
            let on = grid[p];

            neighbors == 3 || (on && neighbors == 2)
        });
    }

    grid.elements().filter(|b| **b).count() as i64
}

fn part2(input: &str) -> i64 {
    let mut grid = Grid::new_by_char(input, |c| c == '#');
    grid[Vector2::new(0, 0)] = true;
    grid[Vector2::new(0, 99)] = true;
    grid[Vector2::new(99, 0)] = true;
    grid[Vector2::new(99, 99)] = true;

    for _ in 0..100 {
        grid = Grid::new_with(grid.width(), grid.height(), |p| {
            let neighbors = grid.neighbors8(p).filter(|&p| grid[p]).count();
            let on = grid[p];
            let is_corner = (p.x == 0 || p.x == 99) && (p.y == 0 || p.y == 99);

            is_corner || neighbors == 3 || (on && neighbors == 2)
        });
    }

    grid.elements().filter(|b| **b).count() as i64
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 814);
    assert_eq!(part2(input), 924);
}
