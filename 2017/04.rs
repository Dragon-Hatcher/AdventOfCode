use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 04)
}

fn part1(input: &str) -> i64 {
    input.lines().filter(|l| l.split(' ').all_unique()).count() as i64
}

fn part2(input: &str) -> i64 {
    fn letter_counts(word: &str) -> [i64; 26] {
        let mut map = [0; 26];
        for c in word.bytes() {
            map[(c - b'a') as usize] += 1;
        }
        map
    }

    input
        .lines()
        .filter(|l| l.split(' ').map(letter_counts).all_unique())
        .count() as i64
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa"), 2);
    assert_eq!(part2("abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio"), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 386);
    assert_eq!(part2(input), 208);
}
