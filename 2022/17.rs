use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 17)
}

static PIECES: [&[Vector2]; 5] = [
    &[
        Vector2::new(0, 0),
        Vector2::new(1, 0),
        Vector2::new(2, 0),
        Vector2::new(3, 0),
    ],
    &[
        Vector2::new(1, 0),
        Vector2::new(0, 1),
        Vector2::new(1, 1),
        Vector2::new(2, 1),
        Vector2::new(1, 2),
    ],
    &[
        Vector2::new(0, 0),
        Vector2::new(1, 0),
        Vector2::new(2, 0),
        Vector2::new(2, 1),
        Vector2::new(2, 2),
    ],
    &[
        Vector2::new(0, 0),
        Vector2::new(0, 1),
        Vector2::new(0, 2),
        Vector2::new(0, 3),
    ],
    &[
        Vector2::new(0, 0),
        Vector2::new(1, 0),
        Vector2::new(0, 1),
        Vector2::new(1, 1),
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

    fn collision(p: Vector2, board: &[Vec<bool>; WIDTH]) -> bool {
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
                .map(|p| Vector2::new(p.x + current_offset + dx, p.y + current_height))
                .any(|p| collision(p, &board))
            {
                current_offset += dx;
            }

            if !PIECES[current_piece]
                .iter()
                .map(|p| Vector2::new(p.x + current_offset, p.y + current_height - 1))
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
                let p = Vector2::new(p.x + current_offset, p.y + current_height);
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

fn part1(input: &str) -> i64 {
    run_iters(input, 2022)
}

fn part2(input: &str) -> i64 {
    run_iters(input, 1000000000000)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(input), 3068);
    assert_eq!(part2(input), 1514285714288);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 3151);
    assert_eq!(part2(input), 1560919540245);
}
