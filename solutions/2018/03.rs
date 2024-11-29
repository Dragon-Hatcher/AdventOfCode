use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 03)
}

fn parse(line: &str) -> (i64, Range) {
    let (id, x, y, w, h) = line.nums().tup();
    let tl = Vec2::new(x, y);
    (id, Range::new_tl(tl, w, h))
}

fn part1(input: &str) -> i64 {
    let mut grid = Grid::new_with(1000, 1000, |_| 0);

    for (_id, r) in input.lines().map(parse) {
        grid.fill_range_with(r, |_, v| v + 1);
    }

    grid.points().filter(|p| grid[*p] >= 2).count() as i64
}

fn part2(input: &str) -> i64 {
    let ranges = input.lines().map(parse).collect_vec();

    ranges
        .iter()
        .find(|(_, r1)| ranges.iter().all(|(_, r2)| r1 == r2 || !r1.overlaps(r2)))
        .unwrap()
        .0
}

fn main() {
    advent::new(2018, 3, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
    assert_eq!(part1(input), 4);
    assert_eq!(part2(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 110383);
    assert_eq!(part2(input), 129);
}
