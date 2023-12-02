use advent::prelude::*;
use std::iter::Peekable;

fn default_input() -> &'static str {
    include_input!(2015 / 12)
}

fn part1(input: &str) -> i64 {
    input.nums().sum()
}

enum Json {
    Num(i64),
    Str(String),
    Array(Vec<Json>),
    Obj(FxHashMap<String, Json>),
}

impl Json {
    fn is_red(&self) -> bool {
        match self {
            Json::Str(s) => s == "red",
            _ => false,
        }
    }

    fn redless_sum(&self) -> i64 {
        match self {
            Json::Num(n) => *n,
            Json::Str(_) => 0,
            Json::Array(children) => children.iter().map(Json::redless_sum).sum(),
            Json::Obj(obj) => {
                if obj.values().any(Json::is_red) {
                    0
                } else {
                    obj.values().map(Json::redless_sum).sum()
                }
            }
        }
    }
}

fn parse_json(chars: &mut Peekable<impl Iterator<Item = char>>) -> Json {
    match chars.peek().unwrap() {
        '[' => {
            chars.next();
            let mut children = Vec::new();

            loop {
                match chars.peek().unwrap() {
                    ',' => {
                        chars.next();
                    }
                    ']' => {
                        chars.next();
                        break;
                    }
                    _ => {}
                }
                children.push(parse_json(chars));
            }

            Json::Array(children)
        }
        '{' => {
            chars.next();
            let mut children = FxHashMap::default();

            loop {
                match chars.peek().unwrap() {
                    ',' => {
                        chars.next();
                    }
                    '}' => {
                        chars.next();
                        break;
                    }
                    _ => {}
                }

                let Json::Str(key) = parse_json(chars) else {
                    panic!()
                };
                chars.nu();
                let value = parse_json(chars);

                children.insert(key, value);
            }

            Json::Obj(children)
        }
        '"' => {
            chars.next();
            let mut s = String::new();
            while *chars.peek().unwrap() != '"' {
                s.push(chars.nu());
            }
            chars.next();
            Json::Str(s)
        }
        &c if c.is_ascii_digit() || c == '-' => {
            let mut s = String::new();
            s.push(chars.nu());
            while chars.peek().unwrap().is_ascii_digit() {
                s.push(chars.nu());
            }
            Json::Num(s.parse().unwrap())
        }
        c => panic!("Invalid Json {}", c),
    }
}

fn part2(input: &str) -> i64 {
    parse_json(&mut input.chars().peekable()).redless_sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = r#"[1,{"c":"red","b":2},3]"#;
    assert_eq!(part1(input), 6);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 191164);
    assert_eq!(part2(input), 87842);
}
