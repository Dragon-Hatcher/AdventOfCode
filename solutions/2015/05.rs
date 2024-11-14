use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 05)
}

fn part1(input: &str) -> i64 {
    fn is_vowel(char: &char) -> bool {
        matches!(char, 'a' | 'e' | 'i' | 'o' | 'u')
    }

    input
        .lines()
        .filter(|l| {
            let vowel_count = l.chars().filter(is_vowel).count();
            let contains_dup = l.chars().dedup().count() != l.chars().count();
            vowel_count >= 3
                && contains_dup
                && !l.contains("ab")
                && !l.contains("cd")
                && !l.contains("pq")
                && !l.contains("xy")
        })
        .count() as i64
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .filter(|l| {
            let chars = l.as_bytes();

            let repeat = chars
                .windows(2)
                .enumerate()
                .any(|(i, w1)| chars.windows(2).skip(i + 2).any(|w2| w1 == w2));
            let sandwich = chars.iter().tuple_windows().any(|(a, _, b)| a == b);

            repeat && sandwich
        })
        .count() as i64
}

fn main() {
    advent::new(2015, 05, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    assert_eq!(part1("ugknbfddgicrmopn"), 1);
    assert_eq!(part1("aaa"), 1);
    assert_eq!(part1("jchzalrnumimnmhp"), 0);
    assert_eq!(part1("haegwjzuvuyypxyu"), 0);
    assert_eq!(part1("dvszwmarrgswjxmb"), 0);

    assert_eq!(part2("qjhvhtzxzqqjkmpb"), 1);
    assert_eq!(part2("xxyxx"), 1);
    assert_eq!(part2("uurcxstgmygtbstg"), 0);
    assert_eq!(part2("ieodomkazucvgmuy"), 0);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 255);
    assert_eq!(part2(input), 55);
}
