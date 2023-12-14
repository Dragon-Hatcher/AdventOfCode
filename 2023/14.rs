use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 14)
}

fn cycle(grid: &mut Grid<char>) {
    for _ in 0..grid.height() {
        for i in 1..grid.height() {
            for col in 0..grid.width() {
                let p = Vector2::new(col, i);
                let p2 = Vector2::new(col, i - 1);
                if grid[p] == 'O' && grid[p2] == '.' {
                    grid[p] = '.';
                    grid[p2] = 'O';
                }
            }
        }
    }

    for _ in 0..grid.width() {
        for i in 1..grid.width() {
            for row in 0..grid.height() {
                let p = Vector2::new(i, row);
                let p2 = Vector2::new(i - 1, row);
                if grid[p] == 'O' && grid[p2] == '.' {
                    grid[p] = '.';
                    grid[p2] = 'O';
                }
            }
        }
    }

    for _ in 0..grid.height() {
        for i in (0..grid.height() - 1).rev() {
            for col in 0..grid.width() {
                let p = Vector2::new(col, i);
                let p2 = Vector2::new(col, i + 1);
                if grid[p] == 'O' && grid[p2] == '.' {
                    grid[p] = '.';
                    grid[p2] = 'O';
                }
            }
        }
    }

    for _ in 0..grid.width() {
        for i in (0..grid.width() - 1).rev() {
            for row in 0..grid.height() {
                let p = Vector2::new(i, row);
                let p2 = Vector2::new(i + 1, row);
                if grid[p] == 'O' && grid[p2] == '.' {
                    grid[p] = '.';
                    grid[p2] = 'O';
                }
            }
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut grid = Grid::new_by_char(input, |c| c);

    for _ in 0..grid.height() {
        for i in 1..grid.height() {
            for col in 0..grid.width() {
                let p = Vector2::new(col, i);
                let p2 = Vector2::new(col, i - 1);
                if grid[p] == 'O' && grid[p2] == '.' {
                    grid[p] = '.';
                    grid[p2] = 'O';
                }
            }
        }
    }

    grid.points()
        .filter(|p| grid[*p] == 'O')
        .map(|p| grid.height() - p.y)
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut seen: HashMap<Grid<char>, i64> = HashMap::new();

    let mut grid = Grid::new_by_char(input, |c| c);
    let mut idx = 0;

    while idx < 1000000000 {
        if seen.contains_key(&grid) {
            let diff = idx - seen[&grid];
            if idx + diff < 1000000000 {
                idx += ((1000000000 - idx) / diff) * diff;
            }
        }

        seen.insert(grid.clone(), idx);
        cycle(&mut grid);
        idx += 1;
    }

    grid.points()
        .filter(|p| grid[*p] == 'O')
        .map(|p| grid.height() - p.y)
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(part1(input), 136);
    assert_eq!(part2(input), 64);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 109345);
    assert_eq!(part2(input), 112452);
}
