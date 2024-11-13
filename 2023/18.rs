use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 18)
}

fn solve(is: &[(Direction, i64)]) -> i64 {
    let mut vertices = Vec::new();
    let mut pos = Vector2::ZERO;

    for (dir, dist) in is {
        pos += dir.vector() * dist;
        vertices.push(pos);
    }

    let area = vertices
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| (a.y + b.y) * (a.x - b.x))
        .sum::<i64>()
        / 2;

    let boundary_points: i64 = is.iter().map(|i| i.1).sum();

    // Pick's theorem
    let interior_points = area - boundary_points / 2 + 1;

    boundary_points + interior_points
}

fn part1(input: &str) -> i64 {
    let mut instructions = input
        .lines()
        .map(|l| {
            let (dir, dist, _) = l.split(' ').tup();
            let dir = Direction::from_char(dir.chars().nu());
            let dist = dist.parse().unwrap();
            (dir, dist)
        })
        .collect_vec();

    solve(&mut instructions)
}

fn part2(input: &str) -> i64 {
    let mut instructions = input
        .lines()
        .map(|l| {
            let (_, _, color) = l.split(' ').tup();

            let dist: String = color.chars().skip(2).take(5).collect();
            let dist = i64::from_str_radix(&dist, 16).unwrap();

            let dir = match color.chars().nth(7).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                _ => Direction::Up,
            };

            (dir, dist)
        })
        .collect_vec();

    solve(&mut instructions)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    assert_eq!(part1(input), 62);
    assert_eq!(part2(input), 952408144115);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 106459);
    assert_eq!(part2(input), 63806916814808);
}
