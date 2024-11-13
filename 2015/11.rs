use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 11)
}

fn iterate_char(char: char) -> (char, bool) {
    if char == 'z' {
        ('a', true)
    } else {
        ((char as u8 + 1) as char, false)
    }
}

fn iterate_string(str: &str) -> String {
    let mut new = String::with_capacity(8);
    let mut iterate = true;
    for c in str.chars().rev() {
        let (next_c, still_iterate) = if iterate { iterate_char(c) } else { (c, false) };
        new.insert(0, next_c);
        iterate = iterate && still_iterate
    }
    new
}

fn is_valid_password(str: &str) -> bool {
    if str.contains(['i', 'o', 'l'])
        || !str
            .bytes()
            .tuple_windows()
            .any(|(a, b, c)| b == a + 1 && c == a + 2)
    {
        return false;
    }

    let Some(first) = str.chars().tuple_windows().position(|(a, b)| a == b) else {
        return false;
    };

    str.chars()
        .skip(first + 2)
        .tuple_windows()
        .any(|(a, b)| a == b)
}

fn part1(input: &str) -> String {
    let mut password = input.trim().to_owned();
    while !is_valid_password(&password) {
        password = iterate_string(&password);
    }
    password
}

fn part2(input: &str) -> String {
    let mut password = iterate_string(&part1(input));
    while !is_valid_password(&password) {
        password = iterate_string(&password);
    }
    password
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "abcdefgh";
    assert_eq!(part1(input), "abcdffaa");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "hxbxxyzz");
    assert_eq!(part2(input), "hxcaabcc");
}
