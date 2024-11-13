use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 13)
}

fn find_reflection(grid: &Grid<bool>, smudges: usize) -> Option<i64> {
    for i in 1..grid.width() {
        let size = i.min(grid.width() - i);
        let left = (i - size)..i;
        let right = i..i + size;

        if left
            .zip(right.rev())
            .map(|(l, r)| {
                grid.col(l)
                    .points()
                    .zip(grid.col(r).points())
                    .filter(|(lp, rp)| grid[*lp] != grid[*rp])
                    .count()
            })
            .sum::<usize>()
            == smudges
        {
            return Some(i);
        }
    }

    None
}

fn solve(pattern: &str, smudges: usize) -> i64 {
    let grid = Grid::new_by_char(pattern, |c| c == '#');

    find_reflection(&grid, smudges)
        .or_else(|| find_reflection(&grid.transpose(), smudges).map(|r| r * 100))
        .unwrap_or_default()
}

fn part1(input: &str) -> i64 {
    input.sections().map(|s| solve(s, 0)).sum()
}

fn part2(input: &str) -> i64 {
    input.sections().map(|s| solve(s, 1)).sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    assert_eq!(part1(input), 405);
    assert_eq!(part2(input), 400);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 36041);
    assert_eq!(part2(input), 35915);
}
