use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 08)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    LT,
    GT,
    LTE,
    GTE,
    EQ,
    NEQ,
}

impl Condition {
    fn exec(&self, lhs: i64, rhs: i64) -> bool {
        match self {
            Condition::LT => lhs < rhs,
            Condition::GT => lhs > rhs,
            Condition::LTE => lhs <= rhs,
            Condition::GTE => lhs >= rhs,
            Condition::EQ => lhs == rhs,
            Condition::NEQ => lhs != rhs,
        }
    }
}

fn parse_condition(cond: &str) -> Condition {
    match cond {
        "<" => Condition::LT,
        "<=" => Condition::LTE,
        ">" => Condition::GT,
        ">=" => Condition::GTE,
        "==" => Condition::EQ,
        "!=" => Condition::NEQ,
        _ => panic!("Invalid condition {cond}"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction<'a> {
    write_reg: &'a str,
    delta: i64,

    test_reg: &'a str,
    cond: Condition,
    test_val: i64,
}

fn parse(i: &str) -> Instruction<'_> {
    let (action, condition) = i.split_once(" if ").unwrap();

    let (write_reg, inc_dec, delta) = action.split(" ").tup();
    let delta = delta.parse::<i64>().unwrap() * if inc_dec == "dec" { -1 } else { 1 };

    let (test_reg, cond, test_val) = condition.split(" ").tup();
    let cond = parse_condition(cond);
    let test_val = test_val.parse().unwrap();

    Instruction {
        write_reg,
        delta,
        test_reg,
        cond,
        test_val,
    }
}

fn part1(input: &str) -> i64 {
    let instructions = input.lines().map(parse);
    let mut reg = HashMap::new();

    for i in instructions {
        let test_reg_val = reg.get(i.test_reg).copied().unwrap_or_default();
        if i.cond.exec(test_reg_val, i.test_val) {
            *reg.entry(i.write_reg).or_default() += i.delta;
        }
    }

    reg.values().copied().max().unwrap_or_default()
}

fn part2(input: &str) -> i64 {
    let instructions = input.lines().map(parse);
    let mut reg = HashMap::new();
    let mut max = 0;

    for i in instructions {
        let test_reg_val = reg.get(i.test_reg).copied().unwrap_or_default();
        if i.cond.exec(test_reg_val, i.test_val) {
            let new = reg.get(i.write_reg).copied().unwrap_or_default() + i.delta;
            max = max.max(new);
            reg.insert(i.write_reg, new);
        }
    }

    max
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
    assert_eq!(part1(input), 1);
    assert_eq!(part2(input), 10);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 6343);
    assert_eq!(part2(input), 7184);
}
