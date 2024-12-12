use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 22)
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c == '#');

    let mut infected: HashSet<Vec2> = grid.points().filter(|&p| grid[p]).collect();
    let mut pos = Vec2::new(grid.width() / 2, grid.height() / 2);
    let mut facing = Direction::Up;

    let mut infections = 0;

    for _ in 0..10_000 {
        if infected.contains(&pos) {
            facing = facing.turn_right();
            infected.remove(&pos);
        } else {
            facing = facing.turn_left();
            infected.insert(pos);
            infections += 1;
        }

        pos += facing.vector();
    }

    infections
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c == '#');

    let mut state: HashMap<Vec2, i64> =
        grid.points().filter(|&p| grid[p]).map(|p| (p, 2)).collect();
    let mut pos = Vec2::new(grid.width() / 2, grid.height() / 2);
    let mut facing = Direction::Up;

    let mut infections = 0;

    for _ in 0..10_000_000 {
        facing = match state.get(&pos) {
            None => facing.turn_left(),
            Some(1) => facing,
            Some(2) => facing.turn_right(),
            _ => facing.reverse(),
        };

        let new_state = (state.get(&pos).copied().unwrap_or_default() + 1) % 4;
        if new_state == 0 {
            state.remove(&pos);
        } else {
            state.insert(pos, new_state);
        }

        if new_state == 2 {
            infections += 1
        }

        pos += facing.vector();
    }

    infections
}

fn main() {
    advent::new(2017, 22, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "..#
#..
...";
    assert_eq!(part1(input), 5587);
    assert_eq!(part2(input), 2511944);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 5305);
    assert_eq!(part2(input), 2511424);
}
