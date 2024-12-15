use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 15)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Empty,
    Wall,
    Box,
}

fn parse(input: &str) -> (Grid<Type>, impl Iterator<Item = Direction> + '_, Vec2) {
    let (grid, moves) = input.sections().tup();

    let moves = moves
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(Direction::from_char);

    let robot_grid = Grid::new_by_char(grid, |c| c == '@');
    let pos = robot_grid.points().find(|&p| robot_grid[p]).unwrap();

    fn parse_cell(c: char) -> Type {
        match c {
            '#' => Type::Wall,
            'O' => Type::Box,
            _ => Type::Empty,
        }
    }

    let grid = Grid::new_by_char(grid, parse_cell);

    (grid, moves, pos)
}

fn gps_coord(p: Vec2) -> i64 {
    p.x + 100 * p.y
}

fn part1(input: &str) -> i64 {
    let (mut grid, moves, mut pos) = parse(input);

    'moves: for m in moves {
        let delta = m.vector();

        let mut check_cell = pos + delta;
        let mut move_boxes = vec![];

        while grid[check_cell] != Type::Empty {
            match grid[check_cell] {
                Type::Empty => {}
                Type::Wall => {
                    continue 'moves;
                }
                Type::Box => {
                    move_boxes.push(check_cell);
                }
            }

            check_cell += delta;
        }

        for b in move_boxes.into_iter().rev() {
            grid[b] = Type::Empty;
            grid[b + delta] = Type::Box;
        }

        pos += delta;
    }

    grid.points()
        .filter(|&p| grid[p] == Type::Box)
        .map(gps_coord)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WideType {
    Empty,
    Wall,
    BoxL,
    BoxR,
}

fn part2(input: &str) -> i64 {
    let (grid, moves, mut pos) = parse(input);

    let mut grid = Grid::new_with(grid.width() * 2, grid.height(), |p| {
        match grid[v2(p.x / 2, p.y)] {
            Type::Empty => WideType::Empty,
            Type::Wall => WideType::Wall,
            Type::Box if p.x % 2 == 0 => WideType::BoxL,
            Type::Box => WideType::BoxR,
        }
    });

    pos.x *= 2;

    'moves: for m in moves {
        let delta = m.vector();

        let mut move_boxes = vec![];
        let mut check_from = vec![pos + delta];

        while !check_from.is_empty() {
            let mut new_check_from = vec![];

            for check in check_from {
                match grid[check] {
                    WideType::Empty => {}
                    WideType::Wall => {
                        continue 'moves;
                    }
                    ty @ (WideType::BoxL | WideType::BoxR) => {
                        let origin_off = if ty == WideType::BoxL {
                            Vec2::ZERO
                        } else {
                            -v2(1, 0)
                        };
                        let origin = check + origin_off;

                        move_boxes.push(origin);
                        if m.vertical() {
                            new_check_from.push(origin + delta);
                            new_check_from.push(origin + delta + v2(1, 0));
                        } else {
                            new_check_from.push(check + delta);
                        }
                    }
                }
            }

            check_from = new_check_from;
            check_from.dedup();
        }

        pos += delta;

        for b in move_boxes.into_iter().rev() {
            grid[b + v2(0, 0)] = WideType::Empty;
            grid[b + v2(1, 0)] = WideType::Empty;
            grid[b + v2(0, 0) + delta] = WideType::BoxL;
            grid[b + v2(1, 0) + delta] = WideType::BoxR;
        }
    }

    grid.points()
        .filter(|&p| grid[p] == WideType::BoxL)
        .map(gps_coord)
        .sum()
}

fn main() {
    advent::new(2024, 15, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    assert_eq!(part1(input), 10092);
    assert_eq!(part2(input), 9021);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1485257);
    assert_eq!(part2(input), 1475512);
}
