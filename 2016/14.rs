use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 14)
}

fn has_triple(str: &str) -> Option<char> {
    str.chars()
        .tuple_windows()
        .find_map(|(a, b, c)| if a == b && b == c { Some(a) } else { None })
}

fn has_quint(str: &str, t: char) -> bool {
    str.chars()
        .tuple_windows()
        .any(|(a, b, c, d, e)| a == t && b == t && c == t && d == t && e == t)
}

fn solve(key: &str, md5_iters: usize) -> i64 {
    let mut active: VecDeque<(char, i64)> = VecDeque::new();
    let mut keys = Vec::new();

    for i in 0i64.. {
        if keys.len() >= 64 && active.is_empty() {
            break;
        }

        let mut md5 = format!("{key}{i}");
        for _ in 0..md5_iters {
            md5 = format!("{:032x}", md5_str(md5));
        }

        if let Some((_, pi)) = active.front() {
            if pi + 1000 < i {
                active.pop_front();
            }
        }

        for (c, pi) in &active {
            if has_quint(&md5, *c) {
                keys.push(*pi);
            }
        }

        active.retain(|(c, _)| !has_quint(&md5, *c));

        if let Some(triple_char) = has_triple(&md5) {
            if keys.len() < 64 {
                active.push_back((triple_char, i));
            }
        }
    }

    keys.sort();
    keys[63]
}

fn part1(input: &str) -> i64 {
    solve(input.trim(), 1)
}

fn part2(input: &str) -> i64 {
    solve(input.trim(), 2017)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "abc";
    assert_eq!(part1(input), 22728);
    assert_eq!(part2(input), 22551);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 23890);
    assert_eq!(part2(input), 22696);
}
