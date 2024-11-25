use advent::prelude::*;
use std::{collections::VecDeque, ops::ControlFlow};

fn default_input() -> &'static str {
    include_input!(2017 / 18)
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Reg(char),
    Lit(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Send(char),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Recover(char),
    Jgz(Value, Value),
}

fn parse_value(val: &str) -> Value {
    if let Some(n) = val.trim().nums().next() {
        Value::Lit(n)
    } else {
        Value::Reg(val.trim().chars().nu())
    }
}

fn parse_instruction(instr: &str) -> Instruction {
    if let Some(reg) = instr.strip_prefix("snd ") {
        Instruction::Send(reg.chars().nu())
    } else if let Some(rest) = instr.strip_prefix("set ") {
        let (reg, val) = rest.split_once(' ').unwrap();
        Instruction::Set(reg.chars().nu(), parse_value(val))
    } else if let Some(rest) = instr.strip_prefix("add ") {
        let (reg, val) = rest.split_once(' ').unwrap();
        Instruction::Add(reg.chars().nu(), parse_value(val))
    } else if let Some(rest) = instr.strip_prefix("mul ") {
        let (reg, val) = rest.split_once(' ').unwrap();
        Instruction::Mul(reg.chars().nu(), parse_value(val))
    } else if let Some(rest) = instr.strip_prefix("mod ") {
        let (reg, val) = rest.split_once(' ').unwrap();
        Instruction::Mod(reg.chars().nu(), parse_value(val))
    } else if let Some(reg) = instr.strip_prefix("rcv ") {
        Instruction::Recover(reg.chars().nu())
    } else if let Some(val) = instr.strip_prefix("jgz ") {
        let (x, y) = val.split_once(' ').unwrap();
        Instruction::Jgz(parse_value(x), parse_value(y))
    } else {
        panic!()
    }
}

struct ComputerPart1 {
    pc: i64,
    registers: HashMap<char, i64>,
    playing: i64,
    instructions: Vec<Instruction>,
}

impl ComputerPart1 {
    fn new(instructions: Vec<Instruction>) -> Self {
        ComputerPart1 {
            pc: 0,
            registers: Default::default(),
            playing: 0,
            instructions,
        }
    }

    fn get_reg(&self, r: char) -> i64 {
        self.registers.get(&r).copied().unwrap_or_default()
    }

    fn get_val(&self, val: Value) -> i64 {
        match val {
            Value::Reg(r) => self.get_reg(r),
            Value::Lit(n) => n,
        }
    }

    fn set(&mut self, r: char, v: i64) {
        self.registers.insert(r, v);
    }

    fn tick(&mut self, i: Instruction) -> ControlFlow<()> {
        match i {
            Instruction::Send(r) => self.playing = self.get_reg(r),
            Instruction::Set(r, v) => self.set(r, self.get_val(v)),
            Instruction::Add(r, v) => self.set(r, self.get_reg(r) + self.get_val(v)),
            Instruction::Mul(r, v) => self.set(r, self.get_reg(r) * self.get_val(v)),
            Instruction::Mod(r, v) => self.set(r, self.get_reg(r) % self.get_val(v)),
            Instruction::Recover(r) => {
                if self.get_reg(r) != 0 {
                    return ControlFlow::Break(());
                };
            }
            Instruction::Jgz(condition, offset) => {
                if self.get_val(condition) > 0 {
                    self.pc += self.get_val(offset);
                    return ControlFlow::Continue(());
                }
            }
        }

        self.pc += 1;
        ControlFlow::Continue(())
    }

    fn run(&mut self) -> i64 {
        while let ControlFlow::Continue(_) = self.tick(self.instructions[self.pc as usize]) {}

        self.playing
    }
}

fn part1(input: &str) -> i64 {
    let mut computer = ComputerPart1::new(input.lines().map(parse_instruction).collect());
    computer.run()
}

struct ComputerPart2 {
    pc: i64,
    registers: HashMap<char, i64>,
    receive_queue: VecDeque<i64>,
    instructions: Vec<Instruction>,
}

enum RunResult {
    Send(i64),
    OutOfReceives,
}

impl ComputerPart2 {
    fn new(instructions: Vec<Instruction>, part: i64) -> Self {
        ComputerPart2 {
            pc: 0,
            registers: hashmap!('p' => part),
            receive_queue: VecDeque::new(),
            instructions,
        }
    }

    fn get_reg(&self, r: char) -> i64 {
        self.registers.get(&r).copied().unwrap_or_default()
    }

    fn get_val(&self, val: Value) -> i64 {
        match val {
            Value::Reg(r) => self.get_reg(r),
            Value::Lit(n) => n,
        }
    }

    fn set(&mut self, r: char, v: i64) {
        self.registers.insert(r, v);
    }

    fn tick(&mut self, i: Instruction) -> ControlFlow<RunResult> {
        match i {
            Instruction::Send(r) => {
                self.pc += 1;
                return ControlFlow::Break(RunResult::Send(self.get_reg(r)));
            }
            Instruction::Set(r, v) => self.set(r, self.get_val(v)),
            Instruction::Add(r, v) => self.set(r, self.get_reg(r) + self.get_val(v)),
            Instruction::Mul(r, v) => self.set(r, self.get_reg(r) * self.get_val(v)),
            Instruction::Mod(r, v) => self.set(r, self.get_reg(r) % self.get_val(v)),
            Instruction::Recover(r) => {
                if let Some(v) = self.receive_queue.pop_front() {
                    self.set(r, v);
                } else {
                    return ControlFlow::Break(RunResult::OutOfReceives);
                }
            }
            Instruction::Jgz(condition, offset) => {
                if self.get_val(condition) > 0 {
                    self.pc += self.get_val(offset);
                    return ControlFlow::Continue(());
                }
            }
        }

        self.pc += 1;
        ControlFlow::Continue(())
    }

    fn run(&mut self) -> RunResult {
        loop {
            if let ControlFlow::Break(r) = self.tick(self.instructions[self.pc as usize]) {
                return r;
            }
        }
    }
}

fn part2(input: &str) -> i64 {
    let instructions: Vec<_> = input.lines().map(parse_instruction).collect();
    let mut c0 = ComputerPart2::new(instructions.clone(), 0);
    let mut c1 = ComputerPart2::new(instructions, 1);

    let mut c1_sends = 0;

    loop {
        let mut rcv_count = 0;

        match c0.run() {
            RunResult::Send(n) => c1.receive_queue.push_back(n),
            RunResult::OutOfReceives => rcv_count += 1,
        }

        match c1.run() {
            RunResult::Send(n) => {
                c0.receive_queue.push_back(n);
                c1_sends += 1
            }
            RunResult::OutOfReceives => rcv_count += 1,
        }

        if rcv_count == 2 {
            break;
        }
    }

    c1_sends
}

fn main() {
    advent::new(2017, 18, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
    assert_eq!(part1(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1187);
    // assert_eq!(part2(input), 0);
}
