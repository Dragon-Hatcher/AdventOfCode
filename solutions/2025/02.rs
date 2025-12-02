use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2025 / 02)
}

fn part1(input: &str) -> i64 {
    fn is_invalid(i: i64) -> bool {
        let str = i.to_string();
        str[0..str.len()/2] == str[str.len()/2..]
    }

    let mut sum = 0;
    for (a, b) in input.nums().tuples() {
        for i in a..=b {
            if is_invalid(i) {
                sum += i;
            }
        }
    }
    sum
}

fn part2(input: &str) -> i64 {
    fn is_invalid(i: i64) -> bool {
        let str = i.to_string();
        'outer: for l in 1..=str.len() / 2 {
            if !str.len().is_multiple_of(l) {
                continue;
            }

            for i in 0..str.len() / l {
                if str[0..l] != str[i*l..(i+1)*l] {
                    continue 'outer;
                }
            }

            return true;
        }
        false
    }

    let mut sum = 0;
    for (a, b) in input.nums().tuples() {
        for i in a..=b {
            if is_invalid(i) {
                sum += i;
            }
        }
    }
    sum
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
