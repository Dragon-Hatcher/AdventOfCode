use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 14)
}

const TILE_SIZE: Vec2 = Vec2::new(101, 103);

#[derive(Debug, Clone, Copy)]
struct Bot {
    pos: Vec2,
    vel: Vec2,
}

impl Bot {
    fn tick(&mut self) {
        self.pos += self.vel;
        self.pos.x = self.pos.x.rem_euclid(TILE_SIZE.x);
        self.pos.y = self.pos.y.rem_euclid(TILE_SIZE.y);
    }

    fn quadrant(&self) -> Option<i64> {
        if self.pos.x == TILE_SIZE.x / 2 || self.pos.y == TILE_SIZE.y / 2 {
            return None;
        }

        let x_pos = self.pos.x * 2 / TILE_SIZE.x;
        let y_pos = self.pos.y * 2 / TILE_SIZE.y;
        Some(x_pos + 2 * y_pos)
    }
}

fn parse(input: &str) -> Vec<Bot> {
    input
        .lines()
        .map(|l| {
            let (px, py, vx, vy) = l.nums().tup();
            Bot {
                pos: Vec2::new(px, py),
                vel: Vec2::new(vx, vy),
            }
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let mut bots = parse(input);

    for _ in 0..100 {
        bots.iter_mut().for_each(Bot::tick);
    }

    let mut quads = bots.iter().counts_by(Bot::quadrant);
    quads.remove(&None);
    quads.values().product::<usize>() as i64
}

fn part2(input: &str) -> i64 {
    let mut bots = parse(input);

    for i in 1.. {
        bots.iter_mut().for_each(Bot::tick);

        let mut grid = Grid::new_homogenous(TILE_SIZE.x, TILE_SIZE.y, false);
        for Bot { pos, .. } in bots.iter() {
            grid[*pos] = true;
        }

        const DENSE_COUNT: usize = 30;

        let dense_rows = grid
            .rows()
            .filter(|row| row.points().filter(|&p| grid[p]).count() > DENSE_COUNT)
            .count();
        let dense_cols = grid
            .cols()
            .filter(|col| col.points().filter(|&p| grid[p]).count() > DENSE_COUNT)
            .count();

        if dense_rows >= 2 && dense_cols >= 2 {
            return i;
        }
    }

    unreachable!()
}

fn main() {
    advent::new(2024, 14, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 228421332);
    assert_eq!(part2(input), 7790);
}
