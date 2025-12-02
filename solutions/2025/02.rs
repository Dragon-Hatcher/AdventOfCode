use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2025 / 02)
}

fn solve<F>(f: F, input: &str) -> i64
where
    F: Fn(i64) -> bool,
{
    input
        .nums()
        .tuples()
        .flat_map(|(a, b)| a..=b)
        .filter(|&n| f(n))
        .sum()
}

fn part1(input: &str) -> i64 {
    fn is_invalid(i: i64) -> bool {
        let digits = i.ilog10() + 1;
        let mask = 10i64.pow(digits / 2);
        i / mask == i % mask
    }

    solve(is_invalid, input)
}

fn part2(input: &str) -> i64 {
    fn is_invalid(i: i64) -> bool {
        let digits = i.ilog10() + 1;

        'outer: for split in 1..=digits/2 {
            if !digits.is_multiple_of(split) {
                continue;
            }

            let mask = 10i64.pow(split);
            let target = i % mask;
            let mut i = i / mask;
            while i > 0 {
                if i % mask != target { continue 'outer; }
                i /= mask;
            }

            return true;
        }
        false
    }

    solve(is_invalid, input)
}

fn main() {
    advent::new(2025, 2, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
    assert_eq!(part1(input), 1227775554);
    assert_eq!(part2(input), 4174379265);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 40398804950);
    assert_eq!(part2(input), 65794984339);
}
