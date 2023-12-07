use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 07)
}

fn part1(input: &str) -> i64 {
    fn map(c: char) -> char {
        if ('0'..='9').contains(&c) {
            c
        } else {
            match c {
                'A' => 'E',
                'K' => 'D',
                'Q' => 'C',
                'J' => 'B',
                'T' => 'A',
                _ => panic!(),
            }
        }
    }
    let x = input
        .lines()
        .map(|l| {
            let m: String = l.chars().take(5).map(map).collect();
            let mut s: HashMap<char, i64> = HashMap::new();

            for c in m.chars() {
                *s.entry(c).or_default() += 1;
            }

            let mut v = s.values().collect_vec();
            v.sort();
            v.reverse();

            let n: String = l.chars().skip(5).collect();
            let w = n.nums().nu();

            if v[0] == &5 {
                (format!("z{m}"), w)
            } else if v[0] == &4 {
                (format!("y{m}"), w)
            } else if v[0] == &3 && v[1] == &2 {
                (format!("x{m}"), w)
            } else if v[0] == &3 {
                (format!("w{m}"), w)
            } else if v[0] == &2 && v[1] == &2 {
                (format!("c{m}"), w)
            } else if v[0] == &2 {
                (format!("b{m}"), w)
            } else {
                (format!("a{m}"), w)
            }
        })
        .sorted_by_key(|(s, _)| s.clone())
        .collect_vec();

    x.iter()
        .enumerate()
        .map(|(i, (_, w))| w * (i as i64 + 1))
        .sum()
}

fn part2(input: &str) -> i64 {
    fn map(c: char) -> char {
        if ('0'..='9').contains(&c) {
            c
        } else {
            match c {
                'A' => 'E',
                'K' => 'D',
                'Q' => 'C',
                'J' => '.',
                'T' => 'A',
                _ => panic!(),
            }
        }
    }
    let x = input
        .lines()
        .map(|l| {
            let m: String = l.chars().take(5).map(map).collect();

            let mut s: HashMap<char, i64> = HashMap::new();

            for c in m.chars() {
                *s.entry(c).or_default() += 1;
            }

            let js = s.get(&'.').map(|j| *j).unwrap_or_default();
            s.remove(&'.');

            let mut v = s.values().collect_vec();
            v.sort();
            v.reverse();

            let n: String = l.chars().skip(5).collect();
            let w = n.nums().nu();

            if js == 5 {
                (format!("z{m}"), w)
            } else if *v[0] >= 5 - js {
                (format!("z{m}"), w)
            } else if *v[0] >= 4 - js {
                (format!("y{m}"), w)
            } else if (js == 0 && v[0] >= &3 && v[1] >= &2)
                || (js == 1 && *v[0] >= 2 && v[1] >= &2)
                || (js == 2 && *v[0] >= 2 && *v[1] >= 1)
            {
                (format!("x{m}"), w)
            } else if *v[0] >= 3 - js {
                (format!("w{m}"), w)
            } else if v[0] == &2 && *v[1] >= 2 - js {
                (format!("c{m}"), w)
            } else if *v[0] >= 2 - js {
                (format!("b{m}"), w)
            } else {
                (format!("a{m}"), w)
            }
        })
        .sorted_by_key(|(s, _)| s.clone())
        .collect_vec();

    x.iter()
        .enumerate()
        .map(|(i, (_, w))| w * (i as i64 + 1))
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part1(input), 6440);
    assert_eq!(part2(input), 5905);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 251058093);
    assert_eq!(part2(input), 249781879);
}
