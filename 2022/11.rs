use advent::prelude::*;

fn default_input() -> MonkeyPen {
    MonkeyPen(
        include_input!(2022 / 11)
            .sections()
            .map(parse_monkey)
            .collect(),
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Equation {
    op: Operation,
    by: Option<i64>,
}

impl Equation {
    fn exec(self, on: i64) -> i64 {
        match self.op {
            Operation::Add => on + self.by.unwrap_or(on),
            Operation::Multiply => on * self.by.unwrap_or(on),
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    equation: Equation,
    divisor: i64,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: i64,
}

impl Monkey {
    fn target_monkey(&self, worry_level: i64) -> usize {
        if worry_level % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let (_, l_items, l_equation, l_divisor, l_true_monkey, l_false_monkey) = input.lines().tup();
    Monkey {
        items: l_items.nums().collect(),
        equation: Equation {
            op: if l_equation.contains('+') {
                Operation::Add
            } else {
                Operation::Multiply
            },
            by: l_equation.nums().next(),
        },
        divisor: l_divisor.nums().nu(),
        true_monkey: l_true_monkey.nums().nu() as usize,
        false_monkey: l_false_monkey.nums().nu() as usize,
        inspection_count: 0,
    }
}

#[derive(Clone)]
struct MonkeyPen(Vec<Monkey>);

impl MonkeyPen {
    fn common_denominator(&self) -> i64 {
        self.0.iter().map(|m| m.divisor).product()
    }

    fn run_round(&mut self, worry_divisor: i64) {
        let cd = self.common_denominator();
        let monkeys = &mut self.0;

        for mi in 0..monkeys.len() {
            while !monkeys[mi].items.is_empty() {
                monkeys[mi].inspection_count += 1;

                let mut worry_level = monkeys[mi].items.drain(0..1).nu();
                worry_level = monkeys[mi].equation.exec(worry_level);
                worry_level /= worry_divisor;
                worry_level %= cd; // Prevent worry levels from becoming too high

                let target = monkeys[mi].target_monkey(worry_level);
                monkeys[target].items.push(worry_level);
            }
        }
    }

    fn monkey_business(&self) -> i64 {
        self.0
            .iter()
            .map(|m| m.inspection_count)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

fn part1(mut monkeys: MonkeyPen) -> i64 {
    for _ in 0..20 {
        monkeys.run_round(3);
    }

    monkeys.monkey_business()
}

fn part2(mut monkeys: MonkeyPen) -> i64 {
    for _ in 0..10000 {
        monkeys.run_round(1);
    }

    monkeys.monkey_business()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    let input = MonkeyPen(str.sections().map(parse_monkey).collect());
    assert_eq!(part1(input.clone()), 10605);
    assert_eq!(part2(input), 2713310158);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 61005);
    assert_eq!(part2(input), 20567144694);
}
