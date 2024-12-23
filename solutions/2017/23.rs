use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 23)
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Reg(char),
    Lit(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

fn parse_value(val: &str) -> Value {
    if let Some(n) = val.nums().next() {
        Value::Lit(n)
    } else {
        Value::Reg(val.chars().nu())
    }
}

fn parse_instruction(ins: &str) -> Instruction {
    if let Some(rest) = ins.strip_prefix("set ") {
        let (x, y) = rest.split_once(' ').unwrap();
        Instruction::Set(x.chars().nu(), parse_value(y))
    } else if let Some(rest) = ins.strip_prefix("sub ") {
        let (x, y) = rest.split_once(' ').unwrap();
        Instruction::Sub(x.chars().nu(), parse_value(y))
    } else if let Some(rest) = ins.strip_prefix("mul ") {
        let (x, y) = rest.split_once(' ').unwrap();
        Instruction::Mul(x.chars().nu(), parse_value(y))
    } else if let Some(rest) = ins.strip_prefix("jnz ") {
        let (x, y) = rest.split_once(' ').unwrap();
        Instruction::Jnz(parse_value(x), parse_value(y))
    } else {
        panic!()
    }
}

struct Computer {
    pc: usize,
    instructions: Vec<Instruction>,
    registers: HashMap<char, i64>,
    muls: i64,
}

fn parse_computer(input: &str) -> Computer {
    Computer {
        pc: 0,
        instructions: input.lines().map(parse_instruction).collect(),
        registers: Default::default(),
        muls: 0,
    }
}

impl Computer {
    fn eval(&self, val: Value) -> i64 {
        match val {
            Value::Reg(r) => self.registers.get(&r).copied().unwrap_or_default(),
            Value::Lit(n) => n,
        }
    }

    fn exec_ins(&mut self, ins: Instruction) {
        match ins {
            Instruction::Set(r, v) => {
                self.registers.insert(r, self.eval(v));
            }
            Instruction::Sub(r, v) => {
                self.registers
                    .insert(r, self.eval(Value::Reg(r)) - self.eval(v));
            }
            Instruction::Mul(r, v) => {
                self.registers
                    .insert(r, self.eval(Value::Reg(r)) * self.eval(v));
                self.muls += 1;
            }
            Instruction::Jnz(cond, off) => {
                if self.eval(cond) != 0 {
                    self.pc += self.eval(off) as usize;
                    return;
                }
            }
        };

        self.pc += 1
    }

    fn run(&mut self) {
        while let Some(&ins) = self.instructions.get(self.pc) {
            self.exec_ins(ins);
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut computer = parse_computer(input);
    computer.run();
    computer.muls
}

fn part2(_input: &str) -> i64 {
    fn is_composite(b: &i64) -> bool {
        (2..b / 2).any(|n| b % n == 0)
    }

    (106500..=123500).step_by(17).filter(is_composite).count() as i64

    // b := 106500
    // c := 123500
    // while b != c {
    //     f := 1
    //     d := 2
    //     while d != b {
    //         e := 2
    //         while e != b {
    //             if d * e == b {
    //                 f := 0
    //             }
    //             e += 1
    //         }
    //         d += 1
    //     }
    //     if f == 0 {
    //         h += 1
    //     }
    //     b += 17
    // }
}

fn main() {
    advent::new(2017, 23, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 3969);
    assert_eq!(part2(input), 917);
}
