use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!("31,39 " / 2016 / 13)
}

fn is_open(p: Vec2, key: i64) -> bool {
    p.x >= 0
        && p.y >= 0
        && (p.x * p.x + 3 * p.x + 2 * p.x * p.y + p.y + p.y * p.y + key).count_ones() % 2 == 0
}

fn part1(input: &str) -> i64 {
    let (x, y, key) = input.nums().tup();

    let bfs = bfs()
        .start(Vec2::new(1, 1))
        .is_goal(|&n| n == Vec2::new(x, y))
        .next(|p| p.neighbors4().filter(|&p| is_open(p, key)));

    bfs.solve().steps
}

fn part2(input: &str) -> i64 {
    let (_, _, key) = input.nums().tup();

    let bfs = bfs()
        .start(Vec2::new(1, 1))
        .is_goal(|_| false)
        .next(|p| p.neighbors4().filter(|&p| is_open(p, key)))
        .max_iters(50);

    bfs.finish().total_visited
}

fn main() {
    advent::new(2016, 13, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "7,4 10";
    assert_eq!(part1(input), 11);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 96);
    assert_eq!(part2(input), 141);
}
