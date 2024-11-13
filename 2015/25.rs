use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 25)
}

fn part1(input: &str) -> i64 {
    let (row, col) = input.nums().tup();
    let start_row = row + col - 1;
    let idx = sum_to(start_row - 1) + col;

    let mut code = 20151125;
    for _ in 1..idx {
        code = (code * 252533) % 33554393;
    }
    code
}

fn main() {
    let solution = advent::new(default_input).part(part1).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "3 4";
    assert_eq!(part1(input), 7981243);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 2650453);
}
