use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 11)
}

fn stretch(in_row: bool, factor: i64) -> i64 {
    if in_row {
        1
    } else {
        factor
    }
}

fn solve(input: &str, stretch_factor: i64) -> i64 {
    let grid = Grid::new_by_char(input, |c| c == '#');
    
    let galaxies = grid.points().filter(|&p| grid[p]).collect_vec();

    let x_values = (0..grid.width())
        .map(|x| stretch(galaxies.iter().any(|p| p.x == x), stretch_factor))
        .collect_vec();
    let y_values = (0..grid.height())
        .map(|y| stretch(galaxies.iter().any(|p| p.y == y), stretch_factor))
        .collect_vec();

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let min_x = a.x.min(b.x);
            let max_x = a.x.max(b.x);
            let min_y = a.y.min(b.y);
            let max_y = a.y.max(b.y);

            let xs: i64 = (min_x..max_x).map(|x| x_values[x as usize]).sum();
            let ys: i64 = (min_y..max_y).map(|y| y_values[y as usize]).sum();
            xs + ys
        })
        .sum()
}

fn part1(input: &str) -> i64 {
    solve(input, 2)
}

fn part2(input: &str) -> i64 {
    solve(input, 1000000)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(solve(input, 2), 374);
    assert_eq!(solve(input, 10), 1030);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 10033566);
    assert_eq!(part2(input), 560822911938);
}
