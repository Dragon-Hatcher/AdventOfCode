use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 04)
}

fn find(key: &str, prefix: &str) -> i64 {
    for i in 1.. {
        let test = format!("{key}{i}");
        let md5 = format!("{:032x}", md5_str(test));

        if md5.starts_with(prefix) {
            return i;
        }
    }

    -1
}

fn part1(input: &str) -> i64 {
    find(input.trim(), "00000")
}

fn part2(input: &str) -> i64 {
    find(input.trim(), "000000")
}

fn main() {
    advent::new(2015, 04, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "abcdef";
    assert_eq!(part1(input), 609043);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 254575);
    assert_eq!(part2(input), 1038736);
}
