use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 21)
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operator {
    fn eval(self, left: i64, right: i64) -> i64 {
        match self {
            Operator::Plus => left.saturating_add(right),
            Operator::Minus => left.saturating_sub(right),
            Operator::Multiply => left.saturating_mul(right),
            Operator::Divide => left / right,
        }
    }
}

#[derive(Debug, Clone)]
enum Monkey {
    Number(i64),
    Equation {
        left: String,
        op: Operator,
        right: String,
    },
}

fn parse(input: &str) -> FxHashMap<String, Monkey> {
    let mut res = FxHashMap::default();

    input.non_empty().for_each(|l| {
        let name = l[0..4].to_owned();
        if let Some(n) = l.nums().next() {
            res.insert(name, Monkey::Number(n));
        } else {
            let left = l[6..10].to_owned();
            let op = match l.chars().nth(11).unwrap() {
                '+' => Operator::Plus,
                '-' => Operator::Minus,
                '*' => Operator::Multiply,
                _ => Operator::Divide,
            };
            let right = l[13..17].to_owned();
            res.insert(name, Monkey::Equation { left, op, right });
        }
    });

    res
}

fn get_value(monkeys: &mut FxHashMap<String, Monkey>, name: &str) -> i64 {
    match monkeys.get(name).unwrap() {
        Monkey::Number(n) => *n,
        Monkey::Equation { left, op, right } => {
            let left = left.clone();
            let right = right.clone();
            let op = *op;
            let left_val = get_value(monkeys, &left);
            let right_val = get_value(monkeys, &right);
            let value = op.eval(left_val, right_val);
            monkeys.insert(name.to_owned(), Monkey::Number(value));
            value
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut monkeys = parse(input);
    get_value(&mut monkeys, "root")
}

fn part2(input: &str) -> i64 {
    fn eval(monkeys: &FxHashMap<String, Monkey>, n: i64) -> i64 {
        let mut monkeys = monkeys.clone();
        monkeys.insert("humn".to_owned(), Monkey::Number(n));
        get_value(&mut monkeys, "root")
    }

    let mut monkeys = parse(input);

    let Monkey::Equation { op, .. } = monkeys.get_mut("root").unwrap() else {
        panic!()
    };
    *op = Operator::Minus;

    let mut low = i64::MIN;
    let mut high = i64::MAX;

    if eval(&monkeys, low) > 0 {
        (low, high) = (high, low);
    }

    loop {
        let midpoint = low / 2 + high / 2 + (low % 2 + high % 2) / 2;
        let value = eval(&monkeys, midpoint);
        match value.cmp(&0) {
            Ordering::Less => low = midpoint,
            Ordering::Equal => break midpoint,
            Ordering::Greater => high = midpoint,
        }
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    assert_eq!(part1(input), 152);
    assert_eq!(part2(input), 301);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 110181395003396);
    assert_eq!(part2(input), 3721298272959);
}
