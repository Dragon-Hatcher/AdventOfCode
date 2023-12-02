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
                .any(|(i, w)| chars.windows(2).skip(i + 2).any(|w2| w == w2));
            let sandwich = chars.iter().tuple_windows().any(|(a, _, b)| a == b);

            repeat && sandwich
        })
        .count() as i64
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(
        part1(
            "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb"
        ),
        2
    );
    assert_eq!(part2("qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy"), 2);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 255);
    assert_eq!(part2(input), 55);
}
