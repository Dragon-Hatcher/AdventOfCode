use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 09)
}

fn part1(input: &str) -> i64 {
    let mut cs = input.chars().filter(|c| !c.is_whitespace());
    let mut total_len = 0;

    while let Some(c) = cs.next() {
        if c == '(' {
            let marker: String = cs.by_ref().take_while_inclusive(|c| *c != ')').collect();
            let (len, times) = marker.nums().tup();
            cs = cs.dropping(len as usize);
            total_len += len * times;
        } else {
            total_len += 1;
        }
    }

    total_len
}

fn part2(input: &str) -> i64 {
    fn solve(input: &str) -> i64 {
        let mut cs = input.chars().filter(|c| !c.is_whitespace());
        let mut total_len = 0;

        while let Some(c) = cs.next() {
            if c == '(' {
                let marker: String = cs.by_ref().take_while_inclusive(|c| *c != ')').collect();
                let (len, times) = marker.nums().tup();
                let section: String = cs.by_ref().take(len as usize).collect();
                total_len += solve(&section) * times;
            } else {
                total_len += 1;
            }
        }

        total_len
    }

    solve(input)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("X(8x2)(3x3)ABCY"), 18);
    assert_eq!(
        part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
        445
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 152851);
    assert_eq!(part2(input), 11797310782);
}
