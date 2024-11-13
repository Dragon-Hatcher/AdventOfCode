use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 07)
}

fn part1(input: &str) -> i64 {
    fn has_abba(str: &str) -> bool {
        str.chars()
            .tuple_windows()
            .any(|(a, b, c, d)| a == d && b == c && a != b)
    }

    input
        .lines()
        .filter(|l| {
            let mut inside = false;
            let mut outside = false;

            for (i, section) in l.split(['[', ']']).enumerate() {
                if i % 2 == 0 {
                    inside = inside || has_abba(section);
                } else {
                    outside = outside || has_abba(section);
                }
            }

            inside && !outside
        })
        .count() as i64
}

fn part2(input: &str) -> i64 {
    fn has_aba(inside: &str, outside: &str) -> bool {
        inside.chars().tuple_windows().any(|(a, b, c)| {
            a == c
                && a != b
                && outside
                    .chars()
                    .tuple_windows()
                    .any(|(d, e, f)| d == b && e == a && f == b)
        })
    }
    
    input
        .lines()
        .filter(|l| {
            let mut inside = "".to_owned();
            let mut outside = "".to_owned();

            for (i, section) in l.split(['[', ']']).enumerate() {
                if i % 2 == 0 {
                    inside.push_str(section);
                    inside.push_str("123");
                } else {
                    outside.push_str(section);
                    outside.push_str("123");
                }
            }

            has_aba(&inside, &outside)
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
            "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn"
        ),
        2
    );
    assert_eq!(
        part2(
            "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb"
        ),
        3
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 105);
    assert_eq!(part2(input), 258);
}
