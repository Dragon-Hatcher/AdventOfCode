use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 10)
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c.to_digit(10).unwrap());

    let mut sum = 0;
    for p in grid.points() {
        if grid[p] != 0 {
            continue;
        }

        let visited = bfs()
            .start(p)
            .no_goal()
            .next(|p| {
                let grid = &grid;
                let p = *p;
                grid.neighbors4(p)
                    .filter(move |pp| grid[*pp] == grid[p] + 1)
            })
            .find_all()
            .visited;

        sum += visited.into_values().filter(|v| *v == 9).count() as i64;
    }

    sum
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c.to_digit(10).unwrap());

    let mut sum = 0;
    for p in grid.points() {
        if grid[p] != 0 {
            continue;
        }

        fn find(pos: Vec2, grid: &Grid<u32>) -> i64 {
            if grid[pos] == 9 {
                return 1;
            }

            grid.neighbors4(pos)
                .filter(|pp| grid[*pp] == grid[pos] + 1)
                .map(|pp| find(pp, grid))
                .sum()
        }

        sum += find(p, &grid)
    }

    sum
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
