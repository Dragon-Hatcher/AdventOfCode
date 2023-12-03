use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 03)
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c != '.' && !c.is_ascii_digit());

    let mut sum = 0;
    for (y, row) in input.lines().enumerate() {
        let mut adj = false;
        let mut accum = "".to_owned();

        for (x, c) in row.chars().enumerate() {
            let p = Vector2::new(x as i64, y as i64);

            if c.is_ascii_digit() {
                accum.push(c);
                adj = adj || grid.neighbors8(p).any(|p| grid[p]);
            } else {
                if !accum.is_empty() && adj {
                    sum += accum.parse::<i64>().unwrap();
                }
                adj = false;
                accum = "".to_owned();
            }
        }

        if !accum.is_empty() && adj {
            sum += accum.parse::<i64>().unwrap();
        }
    }

    sum
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c == '*');

    let mut gears: FxHashMap<Vector2, FxHashSet<i64>> = FxHashMap::default();

    for (y, row) in input.lines().enumerate() {
        let mut adj = FxHashSet::default();
        let mut accum = "".to_owned();

        for (x, c) in row.chars().enumerate() {
            let p = Vector2::new(x as i64, y as i64);

            if c.is_ascii_digit() {
                accum.push(c);
                for n in grid.neighbors8(p).filter(|p| grid[*p]) {
                    adj.insert(n);
                }
            } else {
                if !accum.is_empty() && !adj.is_empty() {
                    let num = accum.parse::<i64>().unwrap();
                    for gear in &adj {
                        gears.entry(*gear).or_default().insert(num);
                    }
                }
                adj.clear();
                accum = "".to_owned();
            }
        }

        if !accum.is_empty() && !adj.is_empty() {
            let num = accum.parse::<i64>().unwrap();
            for gear in adj {
                gears.entry(gear).or_default().insert(num);
            }
        }
    }

    gears
        .values()
        .filter(|s| s.len() == 2)
        .map(|s| s.iter().product::<i64>())
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(part1(input), 4361);
    assert_eq!(part2(input), 467835);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 549908);
    assert_eq!(part2(input), 81166799);
}
