use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 11)
}

fn power_level(cell: Vec2, serial: i64) -> i64 {
    let rack_id = cell.x + 10;
    let power_level = rack_id * cell.y + serial;
    let power_level = power_level * rack_id;
    (power_level / 100 % 10) - 5
}

fn find_best_square(powers: &Grid<i64>, size: i64) -> (Vec2, i64) {
    powers
        .points()
        .filter(|tl| powers.in_bounds(tl + Vec2::new(size - 1, size - 1)))
        .map(|tl| {
            (
                tl,
                Range::new_tl(tl, size, size)
                    .points()
                    .map(|p| powers[p])
                    .sum::<i64>(),
            )
        })
        .max_by_key(|(_, val)| *val)
        .unwrap()
}

fn part1(input: &str) -> Vec2 {
    let serial = input.nums().nu();
    let powers = Grid::new_with(300, 300, |p| power_level(p, serial));

    find_best_square(&powers, 3).0
}

fn part2(input: &str) -> Vec3 {
    let serial = input.nums().nu();
    let powers = Grid::new_with(300, 300, |p| power_level(p, serial));

    let mut best_vec = Vec3::ZERO;
    let mut best_pow = 0;
    for s in 1..300 {
        let (tl, pow) = find_best_square(&powers, s);
        if pow > best_pow {
            best_vec = Vec3::new(tl.x, tl.y, s);
            best_pow = pow;
        }
    }

    best_vec
}

fn main() {
    advent::new(2018, 11, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "18";
    assert_eq!(part1(input), Vec2::new(33, 45));
    assert_eq!(part2(input), Vec3::new(90, 269, 16));
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), Vec2::new(20, 54));
    assert_eq!(part2(input), Vec3::new(233, 93, 13));
}
