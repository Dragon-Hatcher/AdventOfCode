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

fn part1(key: &str) -> i64 {
    let key = key.trim();
    find(key, "00000")
}

fn part2(key: &str) -> i64 {
    let key = key.trim();
    find(key, "000000")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("abcdef"), 609043);
    assert_eq!(part1("pqrstuv"), 1048970);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 254575);
    assert_eq!(part2(input), 1038736);
}
