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

fn part1(input: &str) -> i64 {
    let (grid_str, moves) = input.sections().tup();
    let mut grid = Grid::new_by_char(grid_str, |c| match c {
        '#' => Type::Wall,
        'O' => Type::Box,
        _ => Type::Empty,
    });
    let moves = moves.trim().chars().filter(|c| !c.is_ascii_whitespace()).map(Direction::from_char).collect_vec();

    let robo_grid = Grid::new_by_char(grid_str, |c| c == '@');

    let mut pos = robo_grid.points().find(|p| robo_grid[*p]).unwrap();
    'outer: for m in moves {
        let mut box_shift = vec![];

        let mut check = pos + m.vector();
        loop {
            if grid[check] == Type::Wall {
                println!("no move");
                continue 'outer;
            } else if grid[check] == Type::Box {
                box_shift.push(check);
            } else {
                break;
            }
            check += m.vector();
        }

        println!("move shifting: {:?}", &box_shift);

        pos += m.vector();

        for b in box_shift.iter() {
            grid[*b] = Type::Empty;
        }
        for b in box_shift.iter() {
            grid[*b + m.vector()] = Type::Box;
        }


    }

    let mut sum = 0;
    for p in grid.points() {
        if grid[p] == Type::Box {
            sum += p.x + p.y * 100

        }
    }

    sum

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type2 {
    Empty,
    Wall,
    BoxL,
    BoxR,
}

fn part2(input: &str) -> i64 {
//     let input = "##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########

// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    let (grid_str, moves) = input.sections().tup();
    let grid_sm = Grid::new_by_char(grid_str, |c| match c {
        '#' => Type::Wall,
        'O' => Type::Box,
        _ => Type::Empty,
    });
    let mut grid = Grid::new_with(grid_sm.width() * 2, grid_sm.height(), |p| {
        let old_coord = v2(p.x / 2, p.y);

        match (grid_sm[old_coord], p.x % 2) {
            (Type::Empty, _) => Type2::Empty,
            (Type::Wall, _) => Type2::Wall,
            (Type::Box, 0) => Type2::BoxL,
            (Type::Box, _) => Type2::BoxR,
        }
    });

    let moves = moves.trim().chars().filter(|c| !c.is_ascii_whitespace()).map(Direction::from_char).collect_vec();

    let robo_grid = Grid::new_by_char(grid_str, |c| c == '@');

    let mut pos = robo_grid.points().find(|p| robo_grid[*p]).unwrap();
    pos.x *= 2;


    'outer: for m in moves {
        let mut box_shift_l = vec![];
        let mut box_shift_r = vec![];

        let mut check_from = vec![pos + m.vector()];
        loop {
            let mut new_check_from = vec![];

            for check in check_from {
                if grid[check] == Type2::Wall {
                    println!("no move");
                    continue 'outer;
                } else if grid[check] == Type2::BoxL {
                    box_shift_l.push(check);
                    box_shift_r.push(check + v2(1, 0));
                    new_check_from.push(check + m.vector());
                    if m.vertical() {
                        new_check_from.push(check + m.vector() + v2(1, 0));

                    }
                } else if grid[check] == Type2::BoxR {
                    box_shift_r.push(check);
                    box_shift_l.push(check - v2(1, 0));
                    new_check_from.push(check + m.vector());
                    if m.vertical() {
                        new_check_from.push(check + m.vector() - v2(1, 0));

                    }
                } else {
                    // break;
                }    
            }

            if new_check_from.is_empty() {
                break;
            }
            new_check_from.dedup();
            check_from = new_check_from;
        }

        // println!("move shifting: {:?}", &box_shift);

        pos += m.vector();

        println!("{:?} + {:?}", box_shift_l, box_shift_r);
        for b in box_shift_l.iter() {
            grid[*b] = Type2::Empty;
        }
        for b in box_shift_r.iter() {
            grid[*b] = Type2::Empty;
        }
        for b in box_shift_l.iter() {
            grid[*b + m.vector()] = Type2::BoxL;
        }
        for b in box_shift_r.iter() {
            grid[*b + m.vector()] = Type2::BoxR;
        }


    }

    let mut sum = 0;
    for p in grid.points() {
        if grid[p] == Type2::BoxL {
            sum += p.x + p.y * 100

        }
    }

    sum
}

fn main() {
    advent::new(2024, 15, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    assert_eq!(part1(input), 2028);
    assert_eq!(part2("##########
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
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"), 0);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
