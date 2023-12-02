use advent::prelude::*;

fn parse(input: &str) -> FxHashMap<String, Instruction> {
    let mut map = FxHashMap::default();
    for (name, instruction) in input.lines().map(parse_line) {
        map.insert(name, instruction);
    }
    map
}

fn default_input() -> FxHashMap<String, Instruction> {
    parse(include_input!(2015 / 07))
}

#[derive(Debug, Clone)]
enum Expr {
    Constant(u16),
    Var(String),
}

#[derive(Debug, Clone)]
enum Instruction {
    Expr(Expr),
    And(Expr, Expr),
    Or(Expr, Expr),
    LeftShift(Expr, u16),
    RightShift(Expr, u16),
    Not(Expr),
}

fn parse_line(str: &str) -> (String, Instruction) {
    let (instruction, name) = str.split_once(" -> ").unwrap();
    let name = name.trim().to_owned();

    fn parse_expr(str: &str) -> Expr {
        if let Some(num) = str.nums().next() {
            Expr::Constant(num as u16)
        } else {
            Expr::Var(str.trim().to_owned())
        }
    }

    let instruction = if let Some((lhs, rhs)) = instruction.split_once(" AND ") {
        Instruction::And(parse_expr(lhs), parse_expr(rhs))
    } else if let Some((lhs, rhs)) = instruction.split_once(" OR ") {
        Instruction::Or(parse_expr(lhs), parse_expr(rhs))
    } else if let Some((lhs, rhs)) = instruction.split_once(" LSHIFT ") {
        Instruction::LeftShift(parse_expr(lhs), rhs.nums().nu() as u16)
    } else if let Some((lhs, rhs)) = instruction.split_once(" RSHIFT ") {
        Instruction::RightShift(parse_expr(lhs), rhs.nums().nu() as u16)
    } else if let Some((_, rhs)) = instruction.split_once("NOT") {
        Instruction::Not(parse_expr(rhs))
    } else {
        Instruction::Expr(parse_expr(instruction))
    };

    (name, instruction)
}

fn eval_name(circuit: &mut FxHashMap<String, Instruction>, name: &str) -> u16 {
    fn eval_expr(circuit: &mut FxHashMap<String, Instruction>, e: &Expr) -> u16 {
        match e {
            Expr::Constant(c) => *c,
            Expr::Var(n) => eval_name(circuit, n),
        }
    }

    let val = match &circuit[name] {
        Instruction::Expr(e) => eval_expr(circuit, &e.clone()),
        Instruction::And(l, r) => {
            let l = l.clone();
            let r = r.clone();
            let lv = eval_expr(circuit, &l);
            let rv = eval_expr(circuit, &r);
            lv & rv
        }
        Instruction::Or(l, r) => {
            let l = l.clone();
            let r = r.clone();
            let lv = eval_expr(circuit, &l);
            let rv = eval_expr(circuit, &r);
            lv | rv
        }
        Instruction::LeftShift(l, shift) => {
            let l = l.clone();
            let shift = *shift;
            let lv = eval_expr(circuit, &l);
            lv << shift
        }
        Instruction::RightShift(l, shift) => {
            let l = l.clone();
            let shift = *shift;
            let lv = eval_expr(circuit, &l);
            lv >> shift
        }
        Instruction::Not(e) => !eval_expr(circuit, &e.clone()),
    };

    circuit.insert(name.to_owned(), Instruction::Expr(Expr::Constant(val)));

    val
}

fn part1(mut input: FxHashMap<String, Instruction>) -> i64 {
    eval_name(&mut input, "a") as i64
}

fn part2(mut input: FxHashMap<String, Instruction>) -> i64 {
    let a = eval_name(&mut input.clone(), "a");
    input.insert("b".to_owned(), Instruction::Expr(Expr::Constant(a)));
    eval_name(&mut input, "a") as i64
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = parse(
        "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
h -> a",
    );
    assert_eq!(part1(input.clone()), 65412);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 956);
    assert_eq!(part2(input), 40149);
}
