use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 23)
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
}

fn parse_reg(reg: &str) -> Register {
    match reg {
        "a" => Register::A,
        "b" => Register::B,
        "c" => Register::C,
        "d" => Register::D,
        _ => panic!("Invalid register {reg}."),
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Const(i64),
    Reg(Register),
}

fn parse_val(val: &str) -> Value {
    if let Some(n) = val.nums().next() {
        Value::Const(n)
    } else {
        Value::Reg(parse_reg(val))
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Copy(Value, Value),
    Inc(Register),
    Dec(Register),
    Jnz(Value, Value),
    Toggle(Register),
}

impl Instruction {
    fn toggle(self) -> Instruction {
        match self {
            Instruction::Copy(from, to) => Instruction::Jnz(from, to),
            Instruction::Inc(r) => Instruction::Dec(r),
            Instruction::Dec(r) => Instruction::Inc(r),
            Instruction::Jnz(test, off) => Instruction::Copy(test, off),
            Instruction::Toggle(r) => Instruction::Inc(r),
        }
    }
}

struct Machine {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    ip: i64,
    instructions: Vec<Instruction>,
}

fn parse_machine(input: &str) -> Machine {
    let instructions = input
        .lines()
        .map(|l| {
            if let Some(rest) = l.strip_prefix("cpy ") {
                let (from, to) = rest.split_once(' ').unwrap();
                Instruction::Copy(parse_val(from), parse_val(to))
            } else if let Some(reg) = l.strip_prefix("inc ") {
                Instruction::Inc(parse_reg(reg))
            } else if let Some(reg) = l.strip_prefix("dec ") {
                Instruction::Dec(parse_reg(reg))
            } else if let Some(val) = l.strip_prefix("tgl ") {
                Instruction::Toggle(parse_reg(val))
            } else {
                let rest = l.strip_prefix("jnz ").unwrap();
                let (test, off) = rest.split_once(' ').unwrap();
                Instruction::Jnz(parse_val(test), parse_val(off))
            }
        })
        .collect();

    Machine {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        ip: 0,
        instructions,
    }
}

impl Machine {
    fn get(&self, reg: Register) -> i64 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
        }
    }

    fn set(&mut self, reg: Register, val: i64) {
        match reg {
            Register::A => self.a = val,
            Register::B => self.b = val,
            Register::C => self.c = val,
            Register::D => self.d = val,
        }
    }

    fn eval(&self, value: Value) -> i64 {
        match value {
            Value::Const(val) => val,
            Value::Reg(r) => self.get(r),
        }
    }

    fn run_to_completion(&mut self) {
        while self.ip < self.instructions.len() as i64 {
            match self.instructions[self.ip as usize] {
                Instruction::Copy(val, to) => {
                    if let Value::Reg(to) = to {
                        self.set(to, self.eval(val));
                    }
                }
                Instruction::Inc(r) => self.set(r, self.get(r) + 1),
                Instruction::Dec(r) => self.set(r, self.get(r) - 1),
                Instruction::Jnz(test, off) => {
                    if self.eval(test) != 0 {
                        self.ip += self.eval(off);
                        continue;
                    }
                }
                Instruction::Toggle(val) => {
                    let off = self.get(val);
                    let idx = self.ip + off;
                    if idx >= 0 && idx < self.instructions.len() as i64 {
                        self.instructions[idx as usize] = self.instructions[idx as usize].toggle();
                    }
                }
            }
            self.ip += 1;
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut machine = parse_machine(input);
    machine.a = 7;
    machine.run_to_completion();
    machine.a
}

fn part2(input: &str) -> i64 {
    let mut machine = parse_machine(input);
    machine.a = 12;
    machine.run_to_completion();
    machine.a
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
    assert_eq!(part1(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 11514);
    assert_eq!(part2(input), 479008074);
}
