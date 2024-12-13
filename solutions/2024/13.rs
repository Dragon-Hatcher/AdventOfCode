use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 13)
}

fn parse(input: &str) -> impl Iterator<Item = (Vec2, Vec2, Vec2)> + '_ {
    input.sections().map(|sec| {
        let (ax, ay, bx, by, px, py) = sec.nums().tup();
        (v2(ax, ay), v2(bx, by), v2(px, py))
    })
}

fn solve(a: Vec2, b: Vec2, goal: Vec2) -> Option<i64> {
    let ap = (b.x * goal.y - b.y * goal.x) / (b.x * a.y - a.x * b.y);
    let bp = (a.y * goal.x - a.x * goal.y) / (b.x * a.y - a.x * b.y);

    (goal == a * ap + b * bp).then_some(3 * ap + bp)
}

fn part1(input: &str) -> i64 {
    parse(input)
        .filter_map(|(a, b, goal)| solve(a, b, goal))
        .sum()
}

fn part2(input: &str) -> i64 {
    const OFFSET: Vec2 = Vec2::new(10000000000000, 10000000000000);

    parse(input)
        .map(|(a, b, goal)| (a, b, goal + OFFSET))
        .filter_map(|(a, b, goal)| solve(a, b, goal))
        .sum()
}

fn main() {
    advent::new(2024, 13, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    assert_eq!(part1(input), 480);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 32026);
    assert_eq!(part2(input), 89013607072065);
}
