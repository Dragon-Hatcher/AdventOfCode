use std::iter::once;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 24)
}

fn dist(grid: &Grid<char>, from: Vector2, to: Vector2) -> i64 {
    let mut dist = 0;
    let mut edge = HashSet::new();
    let mut seen: HashSet<Vector2> = HashSet::new();

    edge.insert(from);

    loop {
        let mut next = HashSet::new();

        for p in edge {
            if p == to {
                return dist;
            }
            for n in p.neighbors4() {
                if grid.in_bounds(n) && grid[n] != '#' && !seen.contains(&n) {
                    next.insert(n);
                }
            }
        }

        edge = next;
        seen.extend(&edge);
        dist += 1;
    }
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);
    let start = grid.points().find(|&p| grid[p] == '0').unwrap();
    let targets = grid
        .points()
        .filter(|&p| grid[p] != '.' && grid[p] != '#' && grid[p] != '0')
        .collect_vec();

    let distances: HashMap<(Vector2, Vector2), i64> = once(start)
        .chain(targets.iter().copied())
        .permutations(2)
        .map(|p| {
            let from = p[0];
            let to = p[1];
            ((from, to), dist(&grid, from, to))
        })
        .collect();

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
    let grid = Grid::new_by_char(input, |c| c);
    let start = grid.points().find(|&p| grid[p] == '0').unwrap();
    let targets = grid
        .points()
        .filter(|&p| grid[p] != '.' && grid[p] != '#' && grid[p] != '0')
        .collect_vec();

    let distances: HashMap<(Vector2, Vector2), i64> = once(start)
        .chain(targets.iter().copied())
        .permutations(2)
        .map(|p| {
            let from = p[0];
            let to = p[1];
            ((from, to), dist(&grid, from, to))
        })
        .collect();

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
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
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
