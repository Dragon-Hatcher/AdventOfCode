use std::iter::once;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 24)
}

fn parse(input: &str) -> (Grid<bool>, Vec2, Vec<Vec2>) {
    let grid = Grid::new_by_char(input, |c| c);
    let start = grid.points().find(|&p| grid[p] == '0').unwrap();
    let targets = grid
        .points()
        .filter(|&p| grid[p] != '.' && grid[p] != '#' && grid[p] != '0')
        .collect();

    let grid = grid.map(|&c| c == '#');

    (grid, start, targets)
}

fn dist(grid: &Grid<bool>, from: Vec2, to: Vec2) -> i64 {
    bfs()
        .start(from)
        .goal(to)
        .next(|p| p.neighbors4().filter(|&n| !grid[n]))
        .shortest()
        .steps
}

fn all_distances(grid: &Grid<bool>, start: Vec2, targets: &[Vec2]) -> HashMap<(Vec2, Vec2), i64> {
    once(start)
        .chain(targets.iter().copied())
        .tuple_combinations()
        .flat_map(|(a, b)| {
            let d = dist(grid, a, b);
            [((a, b), d), ((b, a), d)]
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let (grid, start, targets) = parse(input);
    let distances = all_distances(&grid, start, &targets);

    targets
        .iter()
        .permutations(targets.len())
        .map(|t| {
            once(&start)
                .chain(t.iter().copied())
                .tuple_windows()
                .map(|(a, b)| distances.get(&(*a, *b)).unwrap())
                .sum()
        })
        .min()
        .unwrap_or_default()
}

fn part2(input: &str) -> i64 {
    let (grid, start, targets) = parse(input);
    let distances = all_distances(&grid, start, &targets);

    targets
        .iter()
        .permutations(targets.len())
        .map(|t| {
            once(&start)
                .chain(t.iter().copied())
                .chain(once(&start))
                .tuple_windows()
                .map(|(a, b)| distances.get(&(*a, *b)).unwrap())
                .sum()
        })
        .min()
        .unwrap_or_default()
}
fn main() {
    advent::new(2016, 24, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########";
    assert_eq!(part1(input), 14);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 460);
    assert_eq!(part2(input), 668);
}
