use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 23)
}

#[derive(Debug)]
struct Computer {
    a: i64,
    b: i64,
    ip: usize,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn read(&self, reg: Register) -> i64 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    fn write(&mut self, reg: Register, val: i64) {
        match reg {
            Register::A => self.a = val,
            Register::B => self.b = val,
        }
    }

    fn run_to_completion(&mut self) {
        loop {
            if self.ip >= self.instructions.len() {
                break;
            }

            match self.instructions[self.ip] {
                Instruction::Half(r) => {
                    self.write(r, self.read(r) / 2);
                    self.ip += 1;
                }
                Instruction::Triple(r) => {
                    self.write(r, self.read(r) * 3);
                    self.ip += 1;
                }
                Instruction::Increment(r) => {
                    self.write(r, self.read(r) + 1);
                    self.ip += 1;
                }
                Instruction::Jmp(off) => {
                    self.ip = (self.ip as isize + off) as usize;
                }
                Instruction::JmpEven(r, off) => {
                    if self.read(r) % 2 == 0 {
                        self.ip = (self.ip as isize + off) as usize;
                    } else {
                        self.ip += 1;
                    }
                }
                Instruction::JmpOne(r, off) => {
                    if self.read(r) == 1 {
                        self.ip = (self.ip as isize + off) as usize;
                    } else {
                        self.ip += 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jmp(isize),
    JmpEven(Register, isize),
    JmpOne(Register, isize),
}

fn parse_reg(r: &str) -> Register {
    match r {
        "a" => Register::A,
        _ => Register::B,
    }
}

fn parse_instruction(ins: &str) -> Instruction {
    if let Some(r) = ins.strip_prefix("hlf ") {
        Instruction::Half(parse_reg(r))
    } else if let Some(r) = ins.strip_prefix("tpl ") {
        Instruction::Triple(parse_reg(r))
    } else if let Some(r) = ins.strip_prefix("inc ") {
        Instruction::Increment(parse_reg(r))
    } else if let Some(off) = ins.strip_prefix("jmp ") {
        Instruction::Jmp(off.parse().unwrap())
    } else if let Some(args) = ins.strip_prefix("jie ") {
        let (r, off) = args.split_once(", ").unwrap();
        Instruction::JmpEven(parse_reg(r), off.parse().unwrap())
    } else if let Some(args) = ins.strip_prefix("jio ") {
        let (r, off) = args.split_once(", ").unwrap();
        Instruction::JmpOne(parse_reg(r), off.parse().unwrap())
    } else {
        panic!()
    }
}

fn parse_computer(comp: &str) -> Computer {
    Computer {
        a: 0,
        b: 0,
        ip: 0,
        instructions: comp.lines().map(parse_instruction).collect(),
    }
}

fn part1(input: &str) -> i64 {
    let mut computer = parse_computer(input);
    computer.run_to_completion();
    computer.b
}

fn part2(input: &str) -> i64 {
    let mut computer = parse_computer(input);
    computer.a = 1;
    computer.run_to_completion();
    computer.b
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let mut comp = parse_computer(
        "inc a
jio a, +2
tpl a
inc a",
    );
    comp.run_to_completion();
    assert_eq!(comp.a, 2);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 184);
    assert_eq!(part2(input), 231);
}
