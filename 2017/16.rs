use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 16)
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Action {
    fn exec(&self, chars: &mut [char]) {
        match self {
            Action::Spin(n) => chars.rotate_right(*n),
            Action::Exchange(a, b) => chars.swap(*a, *b),
            Action::Partner(a, b) => {
                let a_idx = chars.iter().position(|x| a == x).unwrap();
                let b_idx = chars.iter().position(|x| b == x).unwrap();
                chars.swap(a_idx, b_idx);
            }
        }
    }
}

fn parse_act(act: &str) -> Action {
    if let Some(n) = act.strip_prefix('s') {
        Action::Spin(n.parse().unwrap())
    } else if let Some(idx) = act.strip_prefix('x') {
        let (a, b) = idx.split_once('/').unwrap();
        Action::Exchange(a.parse().unwrap(), b.parse().unwrap())
    } else if let Some(names) = act.strip_prefix('p') {
        let (a, _, b) = names.chars().tup();
        Action::Partner(a, b)
    } else {
        panic!("Invalid action {act:?}.")
    }
}

fn part1(input: &str) -> String {
    let mut chars = (0..16).map(|i| ('a' as u8 + i) as char).collect_vec();

    for act in input.trim().split(',').map(parse_act) {
        act.exec(&mut chars);
    }

    chars.iter().join("")
}

fn part2(input: &str) -> String {
    let mut chars = (0..16).map(|i| ('a' as u8 + i) as char).collect_vec();
    let moves = input.trim().split(',').map(parse_act).collect_vec();

    let mut seen = HashMap::new();

    let mut i = 0;
    const GOAL: i32 = 1000000000;
    while i < GOAL {
        for act in &moves {
            act.exec(&mut chars);
        }

        if let Some(last) = seen.get(&chars) {
            let diff = i - last;
            i += ((GOAL - i) / diff) * diff;
        } else {
            seen.insert(chars.clone(), i);
        }
        i += 1;
    }

    chars.iter().join("")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "pkgnhomelfdibjac");
    assert_eq!(part2(input), "pogbjfihclkemadn");
}
