use advent::prelude::*;

fn parse(input: &str) -> HashMap<String, Instruction> {
    input.lines().map(parse_line).collect()
}

fn default_input() -> HashMap<String, Instruction> {
    parse(include_input!(2015 / 07))
}

#[derive(Debug, Clone)]
enum Atom {
    Constant(u16),
    Var(String),
}

#[derive(Debug, Clone)]
enum Instruction {
    Expr(Atom),
    And(Atom, Atom),
    Or(Atom, Atom),
    LeftShift(Atom, u16),
    RightShift(Atom, u16),
    Not(Atom),
}

fn parse_line(str: &str) -> (String, Instruction) {
    let (instruction, name) = str.split_once(" -> ").unwrap();
    let name = name.trim().to_owned();

    fn parse_atom(str: &str) -> Atom {
        if let Some(num) = str.nums().next() {
            Atom::Constant(num as u16)
        } else {
            Atom::Var(str.trim().to_owned())
        }
    }

    let instruction = if let Some((lhs, rhs)) = instruction.split_once(" AND ") {
        Instruction::And(parse_atom(lhs), parse_atom(rhs))
    } else if let Some((lhs, rhs)) = instruction.split_once(" OR ") {
        Instruction::Or(parse_atom(lhs), parse_atom(rhs))
    } else if let Some((lhs, rhs)) = instruction.split_once(" LSHIFT ") {
        Instruction::LeftShift(parse_atom(lhs), rhs.nums().nu() as u16)
    } else if let Some((lhs, rhs)) = instruction.split_once(" RSHIFT ") {
        Instruction::RightShift(parse_atom(lhs), rhs.nums().nu() as u16)
    } else if let Some((_, rhs)) = instruction.split_once("NOT") {
        Instruction::Not(parse_atom(rhs))
    } else {
        Instruction::Expr(parse_atom(instruction))
    };

    (name, instruction)
}

fn eval(circuit: &HashMap<String, Instruction>, name: &str) -> u16 {
    fn eval_atom(
        circuit: &HashMap<String, Instruction>,
        atom: &Atom,
        cache: &mut HashMap<String, u16>,
    ) -> u16 {
        match atom {
            Atom::Constant(c) => *c,
            Atom::Var(n) => eval_name(circuit, n, cache),
        }
    }

    fn eval_name(
        circuit: &HashMap<String, Instruction>,
        name: &str,
        cache: &mut HashMap<String, u16>,
    ) -> u16 {
        if let Some(val) = cache.get(name) {
            return *val;
        }

        let val = match circuit.get(name).unwrap() {
            Instruction::Expr(atom) => eval_atom(circuit, atom, cache),
            Instruction::And(l, r) => eval_atom(circuit, l, cache) & eval_atom(circuit, r, cache),
            Instruction::Or(l, r) => eval_atom(circuit, l, cache) | eval_atom(circuit, r, cache),
            Instruction::LeftShift(a, shift) => eval_atom(circuit, a, cache) << shift,
            Instruction::RightShift(a, shift) => eval_atom(circuit, a, cache) >> shift,
            Instruction::Not(a) => !eval_atom(circuit, a, cache),
        };

        cache.insert(name.to_owned(), val);
        val
    }

    let mut cache = HashMap::default();
    eval_name(circuit, name, &mut cache);
    cache[name]
}

fn part1(circuit: HashMap<String, Instruction>) -> i64 {
    eval(&circuit, "a") as i64
}

fn part2(mut circuit: HashMap<String, Instruction>) -> i64 {
    let a = eval(&circuit, "a");
    circuit.insert("b".to_owned(), Instruction::Expr(Atom::Constant(a)));
    eval(&circuit, "a") as i64
}

fn main() {
    advent::new(2015, 7, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
    assert_eq!(eval(&parse(input), "h"), 65412);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 956);
    assert_eq!(part2(input), 40149);
}
