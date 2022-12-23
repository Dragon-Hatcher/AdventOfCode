use crate::{
    grid::{Grid, Point},
    standard_parsers::{AocParsed, IntoTup},
};
use itertools::Itertools;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Void,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Void => write!(f, "_"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Forward(i64),
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn value(self) -> i64 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn delta(self) -> Point {
        match self {
            Direction::Right => Point::new(1, 0),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Up => Point::new(0, -1),
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn left(self) -> Self {
        self.right().right().right()
    }
}

fn parse(input: &str) -> (Grid<Cell>, Vec<Instruction>) {
    let (grid, instructions) = input.sections().tup();

    let grid = grid.lines().map(|l| {
        l.chars().map(|c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            _ => Cell::Void,
        })
    });
    let grid = Grid::new(grid);

    let instruction_nums = instructions.nums().map(Instruction::Forward);
    let instruction_turns = instructions.chars().filter_map(|c| match c {
        'L' => Some(Instruction::Left),
        'R' => Some(Instruction::Right),
        _ => None,
    });
    let instructions = instruction_nums.interleave(instruction_turns).collect_vec();

    (grid, instructions)
}

///
/// --- Day 22: Monkey Map ---
///
/// The monkeys take you on a surprisingly easy trail through the jungle. They're
/// even going in roughly the right direction according to your handheld device's
/// Grove Positioning System.
///
/// As you walk, the monkeys explain that the grove is protected by a *force field*.
/// To pass through the force field, you have to enter a password; doing so involves
/// tracing a specific *path* on a strangely-shaped board.
///
/// At least, you're pretty sure that's what you have to do; the elephants aren't
/// exactly fluent in monkey.
///
/// The monkeys give you notes that they took when they last saw the password entered
/// (your puzzle input).
///
/// For example:
///
/// ```
///         ...#
///         .#..
///         #...
///         ....
/// ...#.......#
/// ........#...
/// ..#....#....
/// ..........#.
///         ...#....
///         .....#..
///         .#......
///         ......#.
///
/// 10R5L5R10L4R5L5
///
/// ```
///
/// The first half of the monkeys' notes is a *map of the board*. It is comprised
/// of a set of *open tiles* (on which you can move, drawn `.`) and *solid walls*
/// (tiles which you cannot enter, drawn `#`).
///
/// The second half is a description of *the path you must follow*. It consists of
/// alternating numbers and letters:
///
/// * A *number* indicates the *number of tiles to move* in the direction you are
/// facing. If you run into a wall, you stop moving forward and continue with the
/// next instruction.
/// * A *letter* indicates whether to turn 90 degrees *clockwise* (`R`) or *counterclockwise*
/// (`L`). Turning happens in-place; it does not change your current tile.
///
/// So, a path like `10R5` means "go forward 10 tiles, then turn clockwise 90 degrees,
/// then go forward 5 tiles".
///
/// You begin the path in the leftmost open tile of the top row of tiles. Initially,
/// you are facing *to the right* (from the perspective of how the map is drawn).
///
/// If a movement instruction would take you off of the map, you *wrap around* to
/// the other side of the board. In other words, if your next tile is off of the
/// board, you should instead look in the direction opposite of your current facing
/// as far as you can until you find the opposite edge of the board, then reappear
/// there.
///
/// For example, if you are at `A` and facing to the right, the tile in front of
/// you is marked `B`; if you are at `C` and facing down, the tile in front of you
/// is marked `D`:
///
/// ```
///         ...#
///         .#..
///         #...
///         ....
/// ...#.D.....#
/// ........#...
/// B.#....#...A
/// .....C....#.
///         ...#....
///         .....#..
///         .#......
///         ......#.
///
/// ```
///
/// It is possible for the next tile (after wrapping around) to be a *wall*; this
/// still counts as there being a wall in front of you, and so movement stops before
/// you actually wrap to the other side of the board.
///
/// By drawing the *last facing you had* with an arrow on each tile you visit, the
/// full path taken by the above example looks like this:
///
/// ```
///         >>v#    
///         .#v.    
///         #.v.    
///         ..v.    
/// ...#...v..v#    
/// >>>v...>#.>>    
/// ..#v...#....    
/// ...>>>>v..#.    
///         ...#....
///         .....#..
///         .#......
///         ......#.
///
/// ```
///
/// To finish providing the password to this strange input device, you need to determine
/// numbers for your final *row*, *column*, and *facing* as your final position appears
/// from the perspective of the original map. Rows start from `1` at the top and
/// count downward; columns start from `1` at the left and count rightward. (In the
/// above example, row 1, column 1 refers to the empty space with no tile on it in
/// the top-left corner.) Facing is `0` for right (`>`), `1` for down (`v`), `2`
/// for left (`<`), and `3` for up (`^`). The *final password* is the sum of 1000
/// times the row, 4 times the column, and the facing.
///
/// In the above example, the final row is `6`, the final column is `8`, and the
/// final facing is `0`. So, the final password is 1000 \* 6 + 4 \* 8 + 0: `*6032*`.
///
/// Follow the path given in the monkeys' notes. *What is the final password?*
///
pub fn part1(input: &str) -> i64 {
    let (grid, instructions) = parse(input);

    let mut loc = Point::new(0, 0);
    let mut facing = Direction::Right;

    for x in 0..grid.width() {
        let p = Point::new(x, 0);
        if grid[p] == Cell::Empty {
            loc = p;
            break;
        }
    }

    fn calc_next(loc: Point, facing: Direction, grid: &Grid<Cell>) -> Point {
        let delta = facing.delta();
        let new_loc = loc + delta;

        if !grid.in_bounds(new_loc) || grid[new_loc] == Cell::Void {
            match facing {
                Direction::Right => {
                    for x in 0..grid.width() {
                        let p = Point::new(x, new_loc.y);
                        if grid[p] != Cell::Void {
                            return p;
                        }
                    }
                    unreachable!()
                }
                Direction::Down => {
                    for y in 0..grid.height() {
                        let p = Point::new(new_loc.x, y);
                        if grid[p] != Cell::Void {
                            return p;
                        }
                    }
                    unreachable!()
                }
                Direction::Left => {
                    for x in (0..grid.width()).rev() {
                        let p = Point::new(x, new_loc.y);
                        if grid[p] != Cell::Void {
                            return p;
                        }
                    }
                    unreachable!()
                }
                Direction::Up => {
                    for y in (0..grid.height()).rev() {
                        let p = Point::new(new_loc.x, y);
                        if grid[p] != Cell::Void {
                            return p;
                        }
                    }
                    unreachable!()
                }
            }
        } else {
            new_loc
        }
    }

    for instruction in instructions {
        match instruction {
            Instruction::Forward(n) => {
                for _ in 0..n {
                    let next = calc_next(loc, facing, &grid);
                    if grid[next] == Cell::Wall {
                        break;
                    } else {
                        loc = next;
                    }
                }
            }
            Instruction::Left => facing = facing.left(),
            Instruction::Right => facing = facing.right(),
        }
    }

    1000 * (loc.y + 1) + 4 * (loc.x + 1) + facing.value()
}

///
/// --- Part Two ---
///
/// As you reach the force field, you think you hear some Elves in the distance.
/// Perhaps they've already arrived?
///
/// You approach the strange *input device*, but it isn't quite what the monkeys
/// drew in their notes. Instead, you are met with a large *cube*; each of its six
/// faces is a square of 50x50 tiles.
///
/// To be fair, the monkeys' map *does* have six 50x50 regions on it. If you were
/// to *carefully fold the map*, you should be able to shape it into a cube!
///
/// In the example above, the six (smaller, 4x4) faces of the cube are:
///
/// ```
///         1111
///         1111
///         1111
///         1111
/// 222233334444
/// 222233334444
/// 222233334444
/// 222233334444
///         55556666
///         55556666
///         55556666
///         55556666
///
/// ```
///
/// You still start in the same position and with the same facing as before, but
/// the *wrapping* rules are different. Now, if you would walk off the board, you
/// instead *proceed around the cube*. From the perspective of the map, this can
/// look a little strange. In the above example, if you are at A and move to the
/// right, you would arrive at B facing down; if you are at C and move down, you
/// would arrive at D facing up:
///
/// ```
///         ...#
///         .#..
///         #...
///         ....
/// ...#.......#
/// ........#..A
/// ..#....#....
/// .D........#.
///         ...#..B.
///         .....#..
///         .#......
///         ..C...#.
///
/// ```
///
/// Walls still block your path, even if they are on a different face of the cube.
/// If you are at E facing up, your movement is blocked by the wall marked by the
/// arrow:
///
/// ```
///         ...#
///         .#..
///      -->#...
///         ....
/// ...#..E....#
/// ........#...
/// ..#....#....
/// ..........#.
///         ...#....
///         .....#..
///         .#......
///         ......#.
///
/// ```
///
/// Using the same method of drawing the *last facing you had* with an arrow on each
/// tile you visit, the full path taken by the above example now looks like this:
///
/// ```
///         >>v#    
///         .#v.    
///         #.v.    
///         ..v.    
/// ...#..^...v#    
/// .>>>>>^.#.>>    
/// .^#....#....    
/// .^........#.    
///         ...#..v.
///         .....#v.
///         .#v<<<<.
///         ..v...#.
///
/// ```
///
/// The final password is still calculated from your final position and facing from
/// the perspective of the map. In this example, the final row is `5`, the final
/// column is `7`, and the final facing is `3`, so the final password is 1000 \*
/// 5 + 4 \* 7 + 3 = `*5031*`.
///
/// Fold the map into a cube, *then* follow the path given in the monkeys' notes.
/// *What is the final password?*
///
pub fn part2(input: &str) -> i64 {
    0
}

const PART1_EX_ANSWER: &str = "6032";
const PART1_ANSWER: &str = "136054";
const PART2_EX_ANSWER: &str = "5031";
const PART2_ANSWER: &str = "0";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
