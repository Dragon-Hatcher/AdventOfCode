use crate::grid::Point;
use itertools::Itertools;
use rustc_hash::FxHashMap;

static PIECES: [&[Point]; 5] = [
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
    ],
    &[
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(1, 2),
    ],
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(2, 1),
        Point::new(2, 2),
    ],
    &[
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
    ],
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
    ],
];

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
}

impl Move {
    fn dx(self) -> i64 {
        match self {
            Move::Left => -1,
            Move::Right => 1,
        }
    }
}

fn parse_move(char: char) -> Move {
    match char {
        '<' => Move::Left,
        _ => Move::Right,
    }
}

fn run_iters(input: &str, iters: u64) -> i64 {
    const WIDTH: usize = 7;

    let moves = input.trim().chars().map(parse_move).collect_vec();

    let mut current_move = 0;
    let mut current_piece = 0;
    let mut board: [Vec<bool>; WIDTH] = [vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    let mut tallest = 0;
    let mut removed = 0;

    fn path_across(board: &mut [Vec<bool>; WIDTH], y: usize, x: usize, going: i8) -> usize {
        if !board[x][y] {
            return 0;
        }

        if x + 1 >= WIDTH {
            return y;
        }

        let lowest =path_across(board, y, x + 1, 0);
        if lowest != 0 {
            return y.min(lowest);
        }

        if going >= 0 && y + 1 < board[0].len() {
            let lowest = path_across(board, y + 1, x, 1);
            if lowest != 0 {
                return y.min(lowest);
            }
        }

        if going <= 0 && y > 0 {
            let lowest = path_across(board, y - 1, x, -1);
            if lowest != 0 {
                return y.min(lowest);
            }
        }

        0
    }

    fn chop(board: &mut [Vec<bool>; WIDTH], lowest: usize) -> usize {
        for y in (lowest..lowest.saturating_add(6).min(board[0].len())).rev() {
            let lowest = path_across(board, y, 0, 0);
            if lowest != 0 {
                for c in board.iter_mut() {
                    c.drain(0..=lowest);
                }
                return lowest + 1;
            }
        }

        0
    }

    fn collision(p: Point, board: &[Vec<bool>; WIDTH]) -> bool {
        p.x < 0
            || p.x >= WIDTH as i64
            || p.y < 0
            || (board[p.x as usize].len() as i64 > p.y && board[p.x as usize][p.y as usize])
    }

    let mut current_height = tallest + 3;
    let mut current_offset = 2;

    let mut seen = FxHashMap::default();

    let mut i = 0;
    while i < iters {
        loop {
            let dx = moves[current_move].dx();
            current_move = (current_move + 1) % moves.len();
            if !PIECES[current_piece]
                .iter()
                .map(|p| Point::new(p.x + current_offset + dx, p.y + current_height))
                .any(|p| collision(p, &board))
            {
                current_offset += dx;
            }

            if !PIECES[current_piece]
                .iter()
                .map(|p| Point::new(p.x + current_offset, p.y + current_height - 1))
                .any(|p| collision(p, &board))
            {
                current_height -= 1;
            } else {
                break;
            }
        }

        let lowest = PIECES[current_piece]
            .iter()
            .map(|p| {
                let p = Point::new(p.x + current_offset, p.y + current_height);
                if p.y >= board[p.x as usize].len() as i64 {
                    for _ in 0..(p.y - board[p.x as usize].len() as i64 + 1) {
                        board.iter_mut().for_each(|x| x.push(false));
                    }
                }
                board[p.x as usize][p.y as usize] = true;

                p.y as usize
            })
            .min()
            .unwrap();

        removed += chop(&mut board, lowest);
        let key = (board.clone(), current_piece, current_move);

        current_piece = (current_piece + 1) % PIECES.len();
        tallest = board[0].len() as i64;

        if let Some((old_iter, old_height)) = seen.get(&key) {
            let d_i = i - old_iter;
            let d_height = removed as i64 + tallest - old_height;

            let iters_left = iters - i - 1;
            let simulate_steps = iters_left / d_i;
            removed += d_height as usize * simulate_steps as usize;
            i += simulate_steps * d_i;

            seen.clear();
        } else {
            seen.insert(key, (i, removed as i64 + tallest));
        }

        current_height = tallest + 3;
        current_offset = 2;

        i += 1;
    }

    removed as i64 + tallest
}

///
/// --- Day 17: Pyroclastic Flow ---
///
/// Your handheld device has located an alternative exit from the cave for you and
/// the elephants. The ground is rumbling almost continuously now, but the strange
/// valves bought you some time. It's definitely getting warmer in here, though.
///
/// The tunnels eventually open into a very tall, narrow chamber. Large, oddly-shaped
/// rocks are falling into the chamber from above, presumably due to all the rumbling.
/// If you can't work out where the rocks will fall next, you might be crushed!
///
/// The five types of rocks have the following peculiar shapes, where `#` is rock
/// and `.` is empty space:
///
/// ```
/// ####
///
/// .#.
/// ###
/// .#.
///
/// ..#
/// ..#
/// ###
///
/// #
/// #
/// #
/// #
///
/// ##
/// ##
///
/// ```
///
/// The rocks fall in the order shown above: first the `-` shape, then the `+` shape,
/// and so on. Once the end of the list is reached, the same order repeats: the `-`
/// shape falls first, sixth, 11th, 16th, etc.
///
/// The rocks don't spin, but they do get pushed around by jets of hot gas coming
/// out of the walls themselves. A quick scan reveals the effect the jets of hot
/// gas will have on the rocks as they fall (your puzzle input).
///
/// For example, suppose this was the jet pattern in your cave:
///
/// ```
/// >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
///
/// ```
///
/// In jet patterns, `<` means a push to the left, while `>` means a push to the
/// right. The pattern above means that the jets will push a falling rock right,
/// then right, then right, then left, then left, then right, and so on. If the end
/// of the list is reached, it repeats.
///
/// The tall, vertical chamber is exactly *seven units wide*. Each rock appears so
/// that its left edge is two units away from the left wall and its bottom edge is
/// three units above the highest rock in the room (or the floor, if there isn't
/// one).
///
/// After a rock appears, it alternates between *being pushed by a jet of hot gas*
/// one unit (in the direction indicated by the next symbol in the jet pattern) and
/// then *falling one unit down*. If any movement would cause any part of the rock
/// to move into the walls, floor, or a stopped rock, the movement instead does not
/// occur. If a *downward* movement would have caused a falling rock to move into
/// the floor or an already-fallen rock, the falling rock stops where it is (having
/// landed on something) and a new rock immediately begins falling.
///
/// Drawing falling rocks with `@` and stopped rocks with `#`, the jet pattern in
/// the example above manifests as follows:
///
/// ```
/// The first rock begins falling:
/// |..@@@@.|
/// |.......|
/// |.......|
/// |.......|
/// +-------+
///
/// Jet of gas pushes rock right:
/// |...@@@@|
/// |.......|
/// |.......|
/// |.......|
/// +-------+
///
/// Rock falls 1 unit:
/// |...@@@@|
/// |.......|
/// |.......|
/// +-------+
///
/// Jet of gas pushes rock right, but nothing happens:
/// |...@@@@|
/// |.......|
/// |.......|
/// +-------+
///
/// Rock falls 1 unit:
/// |...@@@@|
/// |.......|
/// +-------+
///
/// Jet of gas pushes rock right, but nothing happens:
/// |...@@@@|
/// |.......|
/// +-------+
///
/// Rock falls 1 unit:
/// |...@@@@|
/// +-------+
///
/// Jet of gas pushes rock left:
/// |..@@@@.|
/// +-------+
///
/// Rock falls 1 unit, causing it to come to rest:
/// |..####.|
/// +-------+
///
/// A new rock begins falling:
/// |...@...|
/// |..@@@..|
/// |...@...|
/// |.......|
/// |.......|
/// |.......|
/// |..####.|
/// +-------+
///
/// Jet of gas pushes rock left:
/// |..@....|
/// |.@@@...|
/// |..@....|
/// |.......|
/// |.......|
/// |.......|
/// |..####.|
/// +-------+
///
/// Rock falls 1 unit:
/// |..@....|
/// |.@@@...|
/// |..@....|
/// |.......|
/// |.......|
/// |..####.|
/// +-------+
///
/// Jet of gas pushes rock right:
/// |...@...|
/// |..@@@..|
/// |...@...|
/// |.......|
/// |.......|
/// |..####.|
/// +-------+
///
/// Rock falls 1 unit:
/// |...@...|
/// |..@@@..|
/// |...@...|
/// |.......|
/// |..####.|
/// +-------+
///
/// Jet of gas pushes rock left:
/// |..@....|
/// |.@@@...|
/// |..@....|
/// |.......|
/// |..####.|
/// +-------+
///
/// Rock falls 1 unit:
/// |..@....|
/// |.@@@...|
/// |..@....|
/// |..####.|
/// +-------+
///
/// Jet of gas pushes rock right:
/// |...@...|
/// |..@@@..|
/// |...@...|
/// |..####.|
/// +-------+
///
/// Rock falls 1 unit, causing it to come to rest:
/// |...#...|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// A new rock begins falling:
/// |....@..|
/// |....@..|
/// |..@@@..|
/// |.......|
/// |.......|
/// |.......|
/// |...#...|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// ```
///
/// The moment each of the next few rocks begins falling, you would see this:
///
/// ```
/// |..@....|
/// |..@....|
/// |..@....|
/// |..@....|
/// |.......|
/// |.......|
/// |.......|
/// |..#....|
/// |..#....|
/// |####...|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |..@@...|
/// |..@@...|
/// |.......|
/// |.......|
/// |.......|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |..@@@@.|
/// |.......|
/// |.......|
/// |.......|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |...@...|
/// |..@@@..|
/// |...@...|
/// |.......|
/// |.......|
/// |.......|
/// |.####..|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |....@..|
/// |....@..|
/// |..@@@..|
/// |.......|
/// |.......|
/// |.......|
/// |..#....|
/// |.###...|
/// |..#....|
/// |.####..|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |..@....|
/// |..@....|
/// |..@....|
/// |..@....|
/// |.......|
/// |.......|
/// |.......|
/// |.....#.|
/// |.....#.|
/// |..####.|
/// |.###...|
/// |..#....|
/// |.####..|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |..@@...|
/// |..@@...|
/// |.......|
/// |.......|
/// |.......|
/// |....#..|
/// |....#..|
/// |....##.|
/// |....##.|
/// |..####.|
/// |.###...|
/// |..#....|
/// |.####..|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |..@@@@.|
/// |.......|
/// |.......|
/// |.......|
/// |....#..|
/// |....#..|
/// |....##.|
/// |##..##.|
/// |######.|
/// |.###...|
/// |..#....|
/// |.####..|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// ```
///
/// To prove to the elephants your simulation is accurate, they want to know how
/// tall the tower will get after 2022 rocks have stopped (but before the 2023rd
/// rock begins falling). In this example, the tower of rocks will be `*3068*` units
/// tall.
///
/// *How many units tall will the tower of rocks be after 2022 rocks have stopped
/// falling?*
///
pub fn part1(input: &str) -> i64 {
    run_iters(input, 2022)
}

///
/// --- Part Two ---
///
/// The elephants are not impressed by your simulation. They demand to know how tall
/// the tower will be after `*1000000000000*` rocks have stopped! Only then will
/// they feel confident enough to proceed through the cave.
///
/// In the example above, the tower would be `*1514285714288*` units tall!
///
/// *How tall will the tower be after `1000000000000` rocks have stopped?*
///
pub fn part2(input: &str) -> i64 {
    run_iters(input, 1000000000000)
}

const PART1_EX_ANSWER: &str = "3068";
const PART1_ANSWER: &str = "3151";
const PART2_EX_ANSWER: &str = "1514285714288";
const PART2_ANSWER: &str = "1560919540245";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
