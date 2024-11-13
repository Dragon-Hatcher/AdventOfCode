use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 09)
}

fn fix_tail(head: Vector2, tail: Vector2) -> Vector2 {
    let diff_x: i64 = head.x - tail.x;
    let diff_y: i64 = head.y - tail.y;

    let diff = match diff_x.abs().cmp(&diff_y.abs()) {
        Ordering::Less => Vector2::new(0, diff_y.signum()),
        Ordering::Equal => Vector2::new(diff_x.signum(), diff_y.signum()),
        Ordering::Greater => Vector2::new(diff_x.signum(), 0),
    };

    head - diff
}

fn solve(input: &str, rope_length: usize) -> i64 {
    let mut visited: FxHashSet<Vector2> = FxHashSet::default();
    let mut knots = vec![Vector2::ZERO; rope_length];

    input.lines().for_each(|l| {
        let (dx, dy) = match l.chars().nu() {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            _ => (0, -1),
        };
        let steps = l.nums().nu();

        for _ in 0..steps {
            let last = knots.iter_mut().last().unwrap();
            last.x += dx;
            last.y += dy;

            for i in (0..knots.len()).rev().skip(1) {
                knots[i] = fix_tail(knots[i + 1], knots[i]);
            }

            visited.insert(knots[0]);
        }
    });

    visited.len() as i64
}

fn part1(input: &str) -> i64 {
    solve(input, 2)
}

fn part2(input: &str) -> i64 {
    solve(input, 10)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 1);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 6266);
    assert_eq!(part2(input), 2369);
}
