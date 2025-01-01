use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 04)
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);

    let mut count = 0;
    for p in grid.points() {
        for delta in Vec2::ZERO.neighbors8() {
            let xmas = grid.get(p) == Some(&'X')
                && grid.get(p + delta * 1) == Some(&'M')
                && grid.get(p + delta * 2) == Some(&'A')
                && grid.get(p + delta * 3) == Some(&'S');
            count += xmas as i64;
        }
    }

    count
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);

    let mut count = 0;
    for p in grid.points() {
        if grid[p] != 'A' {
            continue;
        }

        let ul = grid.get(p - v2(1, 1));
        let dr = grid.get(p + v2(1, 1));
        let ur = grid.get(p - v2(-1, 1));
        let dl = grid.get(p + v2(-1, 1));

        let cross_1 =
            (ul == Some(&'M') && dr == Some(&'S')) || (ul == Some(&'S') && dr == Some(&'M'));
        let cross_2 =
            (ur == Some(&'M') && dl == Some(&'S')) || (ur == Some(&'S') && dl == Some(&'M'));

        count += (cross_1 && cross_2) as i64;
    }

    count
}

fn main() {
    advent::new(2024, 4, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part1(input), 18);
    assert_eq!(part2(input), 9);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 2427);
    assert_eq!(part2(input), 1900);
}
