use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 16)
}

fn energized(grid: &Grid<char>, start: Pose) -> i64 {
    let mut seen_locs = HashSet::new();
    let mut seen_poses = HashSet::new();

    let mut edge = VecDeque::new();
    edge.push_back(start);

    while let Some(pose) = edge.pop_back() {
        let Pose { pos, dir } = pose;

        if seen_poses.contains(&pose) || !grid.in_bounds(pos) {
            continue;
        }

        seen_poses.insert(pose);
        seen_locs.insert(pos);

        match (grid[pos], dir) {
            ('/', Direction::Up | Direction::Down)
            | ('\\', Direction::Right | Direction::Left) => {
                let nd = dir.turn(Turn::Right);
                let np = pos + nd.vector();
                edge.push_back(Pose { pos: np, dir: nd });
            }
            ('/' | '\\', _) => {
                let nd = dir.turn(Turn::Left);
                let np = pos + nd.vector();
                edge.push_back(Pose { pos: np, dir: nd });
            }
            ('-', Direction::Right | Direction::Left)
            | ('|', Direction::Up | Direction::Down) => {
                let np = pos + dir.vector();
                edge.push_back(Pose { pos: np, dir });
            }
            ('-' | '|', _) => {
                let nd1 = dir.turn(Turn::Left);
                let nd2 = dir.turn(Turn::Right);
                edge.push_back(Pose { pos, dir: nd1 });
                edge.push_back(Pose { pos, dir: nd2 });
            }
            _ => {
                let np = pos + dir.vector();
                edge.push_back(Pose { pos: np, dir });
            }
        }
    }

    seen_locs.len() as i64
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);
    energized(
        &grid,
        Pose {
            pos: Vector2::ZERO,
            dir: Direction::Right,
        },
    )
}

#[rustfmt::skip]
fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);

    chain!(
        grid.col(0).points().map(|pos| energized(&grid, Pose { pos, dir: Direction::Right })),
        grid.col(grid.width() - 1).points().map(|pos| energized(&grid, Pose { pos, dir: Direction::Left })),
        grid.row(0).points().map(|pos| energized(&grid, Pose { pos, dir: Direction::Down })),
        grid.row(grid.height() - 1).points().map(|pos| energized(&grid, Pose { pos, dir: Direction::Up })),
    ).max().unwrap_or_default()

}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    assert_eq!(part1(input), 46);
    assert_eq!(part2(input), 51);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 7185);
    assert_eq!(part2(input), 7616);
}
