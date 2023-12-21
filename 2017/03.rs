use advent::prelude::*;

fn default_input() -> i64 {
    include_input!(2017 / 03).trim().parse().unwrap()
}

fn part1(input: i64) -> i64 {
    let mut ring_width: i64 = 1;
    while (ring_width + 2).pow(2) < input {
        ring_width += 1;
    }

    let delta = input - ring_width.pow(2);
    let side_pos = delta % (ring_width + 1);

    let along_side = (((ring_width / 2) + 1) - side_pos).abs();
    let inwards = (ring_width + 1) / 2;

    along_side + inwards
}

fn part2(input: i64) -> i64 {
    let mut size = 1;
    let mut pos = Vector2::E1;
    let mut dir = Direction::Up;

    let mut values = HashMap::new();
    values.insert(Vector2::ZERO, 1);

    loop {
        let val: i64 = pos.neighbors8().filter_map(|p| values.get(&p)).sum();

        if val > input {
            return val;
        }

        values.insert(pos, val);

        if dir == Direction::Right && pos.x == size {
            size += 1;
            pos += dir.vector();
            dir = Direction::Up;
        } else {
            if (dir.vertical() && pos.y.abs() == size) || (dir.horizontal() && pos.x.abs() == size)
            {
                dir = dir.turn(Turn::Left);
            }
            pos += dir.vector();
        }
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1(23), 2);
    assert_eq!(part1(1024), 31);
    assert_eq!(part2(800), 806);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 438);
    assert_eq!(part2(input), 266330);
}
