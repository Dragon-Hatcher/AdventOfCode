use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 18)
}

fn solve(is: &mut [(Direction, i64)]) -> i64 {
    if is[0].0 == Direction::Left {
        for i in is.iter_mut() {
            i.0 = i.0.reverse();
        }
    }

    let mut sum = 0;
    let mut height = 0;

    for i in 0..is.len() {
        let pre = is[(i - 1).rem_euclid(is.len())];
        let post = is[(i + 1).rem_euclid(is.len())];
        let t = is[i];

        match t.0 {
            Direction::Up => height -= t.1,
            Direction::Down => height += t.1,
            Direction::Right => {
                // sub
                let width =
                    t.1 + 1 - (pre.0 == Direction::Down) as i64 - (post.0 == Direction::Up) as i64;
                // eprintln!("{width} * {height} sub");
                sum -= width * height;
            }
            Direction::Left => {
                // add
                let width =
                    t.1 + 1 - (pre.0 == Direction::Up) as i64 - (post.0 == Direction::Down) as i64;
                // eprintln!("{width} * {} add", height + 1);
                sum += width * (height + 1);
            }
        }
    }

    sum
}

fn part1(input: &str) -> i64 {
    let mut instructions = input
        .lines()
        .map(|l| {
            let (dir, dist, _color) = l.split(' ').tup();
            (Direction::from_char(dir.chars().nu()), dist.nums().nu())
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
