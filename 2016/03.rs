use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 03)
}

fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .filter(|l| {
            let (a, b, c) = l.nums().tup();
            is_triangle(a, b, c)
        })
        .count() as i64
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut ls| {
            let (l1, l2, l3) = ls.tup();
            let (a1, b1, c1) = l1.nums().tup();
            let (a2, b2, c2) = l2.nums().tup();
            let (a3, b3, c3) = l3.nums().tup();

            is_triangle(a1, a2, a3) as i64
                + is_triangle(b1, b2, b3) as i64
                + is_triangle(c1, c2, c3) as i64
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 869);
    assert_eq!(part2(input), 1544);
}
