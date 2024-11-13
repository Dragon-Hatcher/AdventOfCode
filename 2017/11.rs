use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 11)
}

fn hex_delta(dir: &str) -> (i64, i64, i64) {
    match dir {
        "n" => (0, 1, -1),
        "s" => (0, -1, 1),
        "ne" => (1, 0, -1),
        "sw" => (-1, 0, 1),
        "se" => (1, -1, 0),
        "nw" => (-1, 1, 0),
        _ => panic!(),
    }
}

fn hex_mag(x: i64, y: i64, z: i64) -> i64 {
    (x.abs() + y.abs() + z.abs()) / 2
}

fn part1(input: &str) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut z: i64 = 0;

    for m in input.trim().split(',') {
        let (dx, dy, dz) = hex_delta(m);
        x += dx;
        y += dy;
        z += dz;
    }

    hex_mag(x, y, z)
}

fn part2(input: &str) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut z: i64 = 0;

    let mut max_mag = 0;

    for m in input.trim().split(',') {
        let (dx, dy, dz) = hex_delta(m);
        x += dx;
        y += dy;
        z += dz;
        max_mag = max_mag.max(hex_mag(x, y, z));
    }

    max_mag
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("ne,ne,ne"), 3);
    assert_eq!(part1("ne,ne,sw,sw"), 0);
    assert_eq!(part1("ne,ne,s,s"), 2);
    assert_eq!(part1("se,sw,se,sw,sw"), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 707);
    assert_eq!(part2(input), 1490);
}
