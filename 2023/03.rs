use advent::prelude::*;
use std::convert::identity;

fn default_input() -> &'static str {
    include_input!(2023 / 03)
}

fn get_numbers(grid: &Grid<char>) -> Vec<(i64, Vec<Vector2>)> {
    let mut numbers = Vec::new();

    for y in 0..grid.height() {
        let mut number = "".to_owned();
        let mut squares = FxHashSet::default();

        for x in 0..grid.width() {
            let p = Vector2::new(x, y);
            let c = grid[p];

            if c.is_ascii_digit() {
                number.push(c);
                squares.extend(grid.neighbors8(p));
            } else if !number.is_empty() {
                numbers.push((number.parse().unwrap(), squares.into_iter().collect()));
                number = "".to_owned();
                squares = FxHashSet::default();
            }
        }

        if !number.is_empty() {
            numbers.push((number.parse().unwrap(), squares.into_iter().collect()));
        }
    }

    numbers
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, identity);

    fn is_special(char: char) -> bool {
        char != '.' && !char.is_ascii_digit()
    }

    get_numbers(&grid)
        .iter()
        .filter(|(_, adjecencies)| adjecencies.iter().any(|&p| is_special(grid[p])))
        .map(|(num, _)| num)
        .sum()
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, identity);

    let mut gears: FxHashMap<Vector2, Vec<i64>> = FxHashMap::default();
    for (num, adjecencies) in get_numbers(&grid) {
        for pos in adjecencies {
            if grid[pos] == '*' {
                gears.entry(pos).or_default().push(num);
            }
        }
    }

    gears
        .values()
        .filter(|s| s.len() == 2)
        .map(|s| s[0] * s[1])
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
