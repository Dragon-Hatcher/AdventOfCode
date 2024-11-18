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
        inside
            .chars()
            .tuple_windows()
            .any(|(a, b, c)| a == c && a != b && outside.contains(&format!("{b}{a}{b}")))
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
    advent::new(2016, 07, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input1 = "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";
    assert_eq!(part1(&input1), 2);

    let input2 = "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb";
    assert_eq!(part2(&input2), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 105);
    assert_eq!(part2(input), 258);
}
