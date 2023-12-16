use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 16)
}

fn energized(grid: &Grid<char>, start: Vector2, dir: Direction) -> i64 {
    let mut seen = HashSet::new();
    let mut seen2 = HashSet::new();

    let mut edge = VecDeque::new();
    edge.push_back((start, dir));

    while let Some((p, dir)) = edge.pop_back() {
        if seen2.contains(&(p, dir)) {
            continue;
        }

        if !grid.in_bounds(p) {
            continue;
        }

        seen2.insert((p, dir));
        seen.insert(p);

        let c = grid[p];

        match c {
            '/' => {
                let nd = dir.turn(if dir == Direction::North || dir == Direction::South {
                    Turn::Right
                } else {
                    Turn::Left
                });
                let np = p + nd.vector();
                edge.push_back((np, nd));
            }
            '\\' => {
                let nd = dir.turn(if dir == Direction::North || dir == Direction::South {
                    Turn::Left
                } else {
                    Turn::Right
                });
                let np = p + nd.vector();
                edge.push_back((np, nd));
            }
            '-' => {
                if dir == Direction::East || dir == Direction::West {
                    let np = p + dir.vector();
                    edge.push_back((np, dir));
                } else {
                    let nd1 = dir.turn(Turn::Left);
                    let nd2 = dir.turn(Turn::Right);
                    edge.push_back((p + nd1.vector(), nd1));
                    edge.push_back((p + nd2.vector(), nd2));
                }
            }
            '|' => {
                if dir == Direction::North || dir == Direction::South {
                    let np = p + dir.vector();
                    edge.push_back((np, dir));
                } else {
                    let nd1 = dir.turn(Turn::Left);
                    let nd2 = dir.turn(Turn::Right);
                    edge.push_back((p + nd1.vector(), nd1));
                    edge.push_back((p + nd2.vector(), nd2));
                }
            }
            _ => {
                let np = p + dir.vector();
                edge.push_back((np, dir));
            }
        }

    }

    seen.len() as i64

}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);
    energized(&grid, Vector2::ZERO, Direction::East)
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);

    let a = grid.col(0).points().map(|p| energized(&grid, p, Direction::East)).max().unwrap_or_default();
    let b = grid.col(grid.width() - 1).points().map(|p| energized(&grid, p, Direction::West)).max().unwrap_or_default();
    let c = grid.row(0).points().map(|p| energized(&grid, p, Direction::South)).max().unwrap_or_default();
    let d = grid.row(grid.height() - 1).points().map(|p| energized(&grid, p, Direction::North)).max().unwrap_or_default();

    a.max(b).max(c).max(d)
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
