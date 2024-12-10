use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 10)
}

fn search(input: &str) -> (i64, i64) {
    let grid = Grid::new_by_char(input, |c| c.to_digit(10).unwrap());

    fn count_nines(pos: Vec2, grid: &Grid<u32>, nines: &mut HashSet<Vec2>) -> i64 {
        if grid[pos] == 9 {
            nines.insert(pos);
            return 1;
        }

        grid.neighbors4(pos)
            .filter(|n| grid[*n] == grid[pos] + 1)
            .map(|n| count_nines(n, grid, nines))
            .sum()
    }

    grid.points()
        .filter(|&p| grid[p] == 0)
        .map(|p| {
            let mut nines = HashSet::default();
            let trail_score = count_nines(p, &grid, &mut nines);
            (nines.len() as i64, trail_score)
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap()
}

fn part1(input: &str) -> i64 {
    search(input).0
}

fn part2(input: &str) -> i64 {
    search(input).1
}

fn main() {
    advent::new(2024, 10, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!(part1(input), 36);
    assert_eq!(part2(input), 81);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 552);
    assert_eq!(part2(input), 1225);
}
