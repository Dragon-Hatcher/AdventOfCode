use advent::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn default_input() -> &'static str {
    include_input!(2024 / 22)
}

fn evolve(s: i64) -> i64 {
    let s = (s ^ (s * 64)) % 16777216;
    let s = (s ^ (s / 32)) % 16777216;
    let s = (s ^ (s * 2048)) % 16777216;
    s
}

fn part1(input: &str) -> i64 {
    let mut a = 0;
    for n in input.nums() {
        let mut s = n;
        for _ in 0..2000 {
            s = evolve(s);
        }
        a += s;
    }
    a
}

fn eval(seq: u32, input: &[i64]) -> i64 {
    let mask = 0xffffffff;
    let mut sum = 0;

    for &n in input {
        let mut s = n;
        let mut last = 0xeeeeeeee;

        for _ in 0..2000 {
            let new = evolve(s);

            let diff = ((new % 10) - (s % 10)) as i8;
            last = ((last << 8) | (diff.to_le_bytes()[0] as u32)) & mask;

            if last == seq {
                sum += new % 10;
                break;
            }

            s = new;
        }
    }

    sum
}

fn find_all_seq(input: &[i64]) -> HashSet<u32> {
    let mask = 0xffffffff;
    let mut all = HashSet::default();

    for &n in input {
        let mut s = n;
        let mut last = 0xeeeeeeee;

        for _ in 0..2000 {
            let new = evolve(s);

            let diff = ((new % 10) - (s % 10)) as i8;
            last = ((last << 8) | (diff.to_le_bytes()[0] as u32)) & mask;

            all.insert(last);

            s = new;
        }
    }

    all
}

fn part2(input: &str) -> i64 {
    let input = input.nums().collect_vec();
    let a = find_all_seq(&input);

    a.into_par_iter()
        .map(|seq| eval(seq, &input))
        .max()
        .unwrap()
}

fn main() {
    advent::new(2024, 22, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    assert_eq!(part1("1 10 100 2024"), 37327623);
    assert_eq!(part2("1 2 3 2024"), 23);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 14119253575);
    assert_eq!(part2(input), 1600);
}
