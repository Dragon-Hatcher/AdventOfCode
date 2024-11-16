use advent::prelude::*;
use serde_json::{self, Value};

fn default_input() -> &'static str {
    include_input!(2015 / 12)
}

fn part1(input: &str) -> i64 {
    input.nums().sum()
}

fn part2(input: &str) -> i64 {
    fn sum(v: &Value) -> i64 {
        match v {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(n) => n.as_i64().unwrap(),
            Value::String(_) => 0,
            Value::Array(vec) => vec.iter().map(sum).sum(),
            Value::Object(map) => {
                if map.values().any(|v| v == "red") {
                    0
                } else {
                    map.values().map(sum).sum()
                }
            }
        }
    }

    let v: Value = serde_json::from_str(input).unwrap();
    sum(&v)
}

fn main() {
    advent::new(2015, 12, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
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
