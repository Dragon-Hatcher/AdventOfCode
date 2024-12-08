use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 08)
}

fn parse(input: &str) -> (Grid<char>, Vec<Vec<Vec2>>) {
    let grid = Grid::new_by_char(input, |c| c);
    let antenna_groups = grid
        .points()
        .map(|p| (grid[p], p))
        .filter(|(freq, _)| *freq != '.')
        .into_group_map()
        .into_values()
        .collect();

    (grid, antenna_groups)
}

fn solve<F>(input: &str, is_match: F) -> i64
where
    F: Fn(Vec2, Vec2, Vec2) -> bool,
{
    let (grid, antenna_groups) = parse(input);

    grid.points()
        .filter(|&p| {
            antenna_groups.iter().any(|group| {
                group
                    .iter()
                    .tuple_combinations()
                    .any(|(&p1, &p2)| is_match(p, p1, p2))
            })
        })
        .count() as i64
}

fn part1(input: &str) -> i64 {
    solve(input, |p, p1, p2| p == p2 * 2 - p1 || p == p1 * 2 - p2)
}

fn part2(input: &str) -> i64 {
    solve(input, |p, p1, p2| (p2 - p1).is_scaling(p - p1))
}

fn main() {
    advent::new(2024, 8, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(part1(input), 14);
    assert_eq!(part2(input), 34);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 398);
    assert_eq!(part2(input), 1333);
}
