use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 06)
}

fn parse(input: &str) -> (Grid<bool>, Vec2) {
    let grid = Grid::new_by_char(input, |c| c == '#');
    let guard_pos = Grid::new_by_char(input, |c| c == '^');
    let guard_pos = guard_pos.points().find(|&p| guard_pos[p]).unwrap();
    (grid, guard_pos)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PathResult {
    Loop,
    OutOfBounds,
}

fn trace_path(
    grid: &Grid<bool>,
    mut pos: Vec2,
    mut facing: Direction,
) -> (HashSet<(Vec2, Direction)>, PathResult) {
    let mut visited = HashSet::default();

    while grid.in_bounds(pos) {
        if visited.contains(&(pos, facing)) {
            // we've entered an infinite loop
            return (visited, PathResult::Loop);
        }

        visited.insert((pos, facing));

        while grid.get(pos + facing.vector()) == Some(&true) {
            facing = facing.turn(Turn::Right);
        }
        pos += facing.vector();
    }

    (visited, PathResult::OutOfBounds)
}

fn part1(input: &str) -> i64 {
    let (grid, start_pos) = parse(input);
    let (path, _) = trace_path(&grid, start_pos, Direction::Up);

    path.into_iter().map(|(p, _)| p).unique().count() as i64
}

fn part2(input: &str) -> i64 {
    let (mut grid, start_pos) = parse(input);

    let mut count = 0;
    for p in grid.points() {
        if grid[p] || p == start_pos {
            continue;
        }

        grid[p] = true;
        count += (trace_path(&grid, start_pos, Direction::Up).1 == PathResult::Loop) as i64;
        grid[p] = false;
    }

    count
}

fn main() {
    advent::new(2024, 6, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part1(input), 41);
    assert_eq!(part2(input), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 5531);
    assert_eq!(part2(input), 2165);
}
