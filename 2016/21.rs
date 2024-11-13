use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!("abcdefgh\n\nfbgdceah\n\n" / 2016 / 21)
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Swap(usize, usize),
    SwapChar(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBased(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            if l.starts_with("swap position") {
                let (a, b) = l.nums().tup();
                Instruction::Swap(a as usize, b as usize)
            } else if l.starts_with("swap letter") {
                let a = l.as_bytes()[12];
                let b = l.as_bytes()[26];
                Instruction::SwapChar(a, b)
            } else if l.starts_with("rotate left") {
                Instruction::RotateLeft(l.nums().nu() as usize)
            } else if l.starts_with("rotate right") {
                Instruction::RotateRight(l.nums().nu() as usize)
            } else if l.starts_with("rotate based") {
                Instruction::RotateBased(l.as_bytes()[35])
            } else if l.starts_with("reverse") {
                let (a, b) = l.nums().tup();
                Instruction::Reverse(a as usize, b as usize)
            } else if l.starts_with("move position") {
                let (a, b) = l.nums().tup();
                Instruction::Move(a as usize, b as usize)
            } else {
                panic!()
            }
        })
        .collect()
}

fn part1(input: &str) -> String {
    let (code, _, instructions) = input.sections().tup();
    let instructions = parse_instructions(instructions);
    let mut code = code.trim().as_bytes().to_owned();

    for i in instructions {
        match i {
            Instruction::Swap(a, b) => {
                code.swap(a, b);
            }
            Instruction::SwapChar(a, b) => {
                let a_idx = code.iter().position(|&c| c == a).unwrap();
                let b_idx = code.iter().position(|&c| c == b).unwrap();
                code.swap(a_idx, b_idx);
            }
            Instruction::RotateLeft(steps) => code.rotate_left(steps),
            Instruction::RotateRight(steps) => code.rotate_right(steps),
            Instruction::RotateBased(a) => {
                let a_idx = code.iter().position(|&c| c == a).unwrap();
                let steps = 1 + a_idx + (a_idx >= 4) as usize;
                let steps = steps % code.len();
                code.rotate_right(steps);
            }
            Instruction::Reverse(start, end) => {
                let sub = &mut code[start..=end];
                sub.reverse();
            }
            Instruction::Move(from, to) => {
                let val = code.remove(from);
                code.insert(to, val);
            }
        }
    }

    String::from_utf8_lossy(&code).into_owned()
}

fn part2(input: &str) -> String {
    let (_, code, instructions) = input.sections().tup();
    let instructions = parse_instructions(instructions);
    let mut code = code.trim().as_bytes().to_owned();

    for i in instructions.into_iter().rev() {
        match i {
            Instruction::Swap(a, b) => {
                code.swap(a, b);
            }
            Instruction::SwapChar(a, b) => {
                let a_idx = code.iter().position(|&c| c == a).unwrap();
                let b_idx = code.iter().position(|&c| c == b).unwrap();
                code.swap(a_idx, b_idx);
            }
            Instruction::RotateLeft(steps) => code.rotate_right(steps),
            Instruction::RotateRight(steps) => code.rotate_left(steps),
            Instruction::RotateBased(a) => {
                let cur_idx = code.iter().position(|&c| c == a).unwrap();

                let i = (cur_idx - 1).rem_euclid(code.len());
                let old_idx = if i % 2 == 0 {
                    // no wrap
                    i / 2
                } else {
                    // wrap
                    (code.len() + i - 1) / 2
                };

                if cur_idx > old_idx {
                    code.rotate_left(cur_idx - old_idx);
                } else {
                    code.rotate_right(old_idx - cur_idx);
                }
            }
            Instruction::Reverse(start, end) => {
                let sub = &mut code[start..=end];
                sub.reverse();
            }
            Instruction::Move(from, to) => {
                let val = code.remove(to);
                code.insert(from, val);
            }
        }
    }

    String::from_utf8_lossy(&code).into_owned()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "abcde

decab

swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";
    assert_eq!(part1(input), "decab");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "aefgbcdh");
    assert_eq!(part2(input), "egcdahbf");
}
