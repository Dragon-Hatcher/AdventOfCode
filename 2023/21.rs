use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 21)
}

fn parse(input: &str) -> (Grid<bool>, Vector2) {
    let grid = Grid::new_by_char(input, |c| c == '#');
    let start = {
        let g = Grid::new_by_char(input, |c| c == 'S');
        g.points().find(|p| g[*p]).unwrap()
    };
    (grid, start)
}

fn solve(grid: &Grid<bool>, start: Vector2, steps: i64) -> i64 {
    let mut edge = HashSet::new();
    edge.insert(start);

    for _ in 0..steps {
        edge = edge
            .iter()
            .flat_map(|p| p.neighbors4().filter(|&p| grid.in_bounds(p) && !grid[p]))
            .collect();
    }

    edge.len() as i64
}

fn part1(input: &str) -> i64 {
    let (grid, start) = parse(input);
    solve(&grid, start, 64)
}

fn part2(input: &str) -> i64 {
    let (grid, start) = parse(input);

    let steps = 26501365;

    assert_eq!(start.x, start.y);
    assert_eq!(grid.width(), grid.height());
    let cc = start.x;
    let size = grid.width();

    let zero_parity_fill = solve(&grid, start, cc + cc);
    let one_parity_fill = solve(&grid, start, cc + cc + 1);

    let steps_left_for_center = (steps - cc - 1) % size;
    let cardinal_fills = solve(&grid, Vector2::new(cc, 0), steps_left_for_center)
        + solve(&grid, Vector2::new(0, cc), steps_left_for_center)
        + solve(&grid, Vector2::new(cc, size - 1), steps_left_for_center)
        + solve(&grid, Vector2::new(size - 1, cc), steps_left_for_center);

    let mut full_fills = one_parity_fill;
    let full_steps = (steps - cc - 1) / size;

    for i in 1..=full_steps {
        let x = if i % 2 == 0 {
            one_parity_fill * i * 4
        } else {
            zero_parity_fill * i * 4
        };
        full_fills += x;
    }

    let edge_count = (steps - cc - 1) / size;
    let outer_edge_count = edge_count + 1;

    let edge_steps = steps - 2 * cc - 2 - (full_steps - 1) * size;
    let outer_edge_steps = steps - 2 * cc - 2 - full_steps * size;

    let mut edge_fills = solve(&grid, Vector2::new(0, 0), edge_steps) * edge_count
        + solve(&grid, Vector2::new(0, size - 1), edge_steps) * edge_count
        + solve(&grid, Vector2::new(size - 1, 0), edge_steps) * edge_count
        + solve(&grid, Vector2::new(size - 1, size - 1), edge_steps) * edge_count;

    if outer_edge_steps >= 0 {
        edge_fills += solve(&grid, Vector2::new(0, 0), outer_edge_steps) * outer_edge_count;
        edge_fills += solve(&grid, Vector2::new(0, size - 1), outer_edge_steps) * outer_edge_count;
        edge_fills += solve(&grid, Vector2::new(size - 1, 0), outer_edge_steps) * outer_edge_count;
        edge_fills +=
            solve(&grid, Vector2::new(size - 1, size - 1), outer_edge_steps) * outer_edge_count;
    }

    cardinal_fills + edge_fills + full_fills
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 3562);
    assert_eq!(part2(input), 592723929260582);
}
