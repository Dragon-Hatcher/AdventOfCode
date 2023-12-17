use advent::prelude::*;

fn parse(input: &str) -> Vec<(Vector2, Vector2)> {
    input
        .non_empty()
        .skip(1)
        .map(|l| {
            let (x1, y1, x2, y2) = l.nums().tup();

            (Vector2::new(x1, y1), Vector2::new(x2, y2))
        })
        .collect()
}

fn default_input() -> (i64, Vec<(Vector2, Vector2)>) {
    (2000000, parse(include_input!(2022 / 15)))
}

fn part1((height, positions): (i64, Vec<(Vector2, Vector2)>)) -> i64 {
    let mut ranges = positions
        .iter()
        .filter_map(|(s, b)| {
            let d = s.manhattan_dist(*b);
            let dy = (s.y - height).abs();
            let dx = d - dy;
            if dx < 0 {
                None
            } else {
                Some((s.x - dx)..(s.x + dx))
            }
        })
        .sorted_by_key(|r| r.start);

    let mut excluded = 0;
    let mut curr = ranges.nu();
    for r in ranges {
        if r.start <= curr.end {
            curr.end = r.end.max(curr.end);
        } else {
            excluded += curr.end - curr.start;
            curr = r;
        }
    }
    excluded += curr.end - curr.start;

    excluded
}

fn part2((height, mut positions): (i64, Vec<(Vector2, Vector2)>)) -> i64 {
    let height = height * 2;

    for y in 0..=height {
        positions.sort_by_key(|(s, _)| (s.y - y).abs());

        let mut x = 0;
        'out: while x <= height {
            let p = Vector2::new(x, y);

            for (s, b) in &positions {
                let d = s.manhattan_dist(*b);
                if s.manhattan_dist(p) <= d {
                    let dy = (p.y - s.y).abs();
                    let dx = (s.x - p.x) + (d - dy) + 1;
                    x += dx;
                    continue 'out;
                }
            }

            return x * 4000000 + y;
        }
    }

    unreachable!();
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    assert_eq!(part1((10, parse(input))), 26);
    assert_eq!(part2((10, parse(input))), 56000011);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4985193);
    assert_eq!(part2(input), 11583882601918);
}
