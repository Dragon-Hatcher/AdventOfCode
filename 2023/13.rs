use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 13)
}

fn solve(pattern: &str, smudges: usize) -> (i64, i64) {
    let grid = Grid::new_by_char(pattern, |c| c == '#');

    for i in 1..grid.width() {
        let left = 0..i;
        let right = i..grid.width();
        let size = (left.end - left.start).min(right.end - right.start);

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
            return (0, i);
        }
    }

    for i in 1..grid.height() {
        let left = 0..i;
        let right = i..grid.height();
        let size = (left.end - left.start).min(right.end - right.start);

        let top = (i - size)..i;
        let bottom = i..i + size;

        if top
            .zip(bottom.rev())
            .map(|(t, b)| {
                grid.row(t)
                    .points()
                    .zip(grid.row(b).points())
                    .filter(|(tp, bp)| grid[*tp] != grid[*bp])
                    .count()
            })
            .sum::<usize>()
            == smudges
        {
            return (i, 0);
        }
    }

    (0, 0)
}

fn part1(input: &str) -> i64 {
    input
        .sections()
        .map(|s| solve(s, 0))
        .map(|(rows, columns)| columns + rows * 100)
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .sections()
        .map(|s| solve(s, 1))
        .map(|(rows, columns)| columns + rows * 100)
        .sum()
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
