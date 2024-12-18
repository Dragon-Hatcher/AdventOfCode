use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 17)
}

fn part1(input: &str) -> String {
    let (start, program) = input.sections().tup();
    let (mut a, mut b, mut c) = start.nums().tup();
    let instructions = program.nums().collect_vec();

    let mut ip = 0;
    let mut out = vec![];

    while ip < instructions.len() {
        let instr = instructions[ip];

        let literal = instructions[ip + 1];
        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            _ => c,
        };

        match instr {
            0 => a = a / 2i64.pow(combo.min(64) as u32),
            1 => b = b ^ literal,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => b = b ^ c,
            5 => out.push(combo % 8),
            6 => b = a / 2i64.pow(combo.min(64) as u32),
            _ => c = a / 2i64.pow(combo.min(64) as u32),
        }

        ip += 2;
    }

    out.into_iter().join(",")
}

fn part2(input: &str) -> i64 {
    let (_, program) = input.sections().tup();
    let instructions = program.nums().collect_vec();

    /*
    Program: 2,4,1,3,7,5,0,3,1,5,4,1,5,5,3,0

    while a != 0 {
        b = a % 8    (2,4)
        b = b ^ 3    (1,3)
        c = a / 2**b (7,5)
        a = a / 8    (0,3)
        b = b ^ 5    (1,5)
        b = b ^ c    (4,1)
        print b % 8  (5,5)
    }
    */

    fn find_quine(a: i64, instructions: &[i64], on: usize) -> Option<i64> {
        for p in 1..8 {
            let a = a + p;

            let b1 = (a % 8) ^ 3;
            let c = a / (2i64.pow(b1 as u32));
            let b2 = b1 ^ 5 ^ c;

            if b2 % 8 == instructions[on] {
                if on == 0 {
                    return Some(a);
                }

                if let ans @ Some(_) = find_quine(a * 8, instructions, on - 1) {
                    return ans;
                }
            }
        }

        None
    }

    find_quine(0, &instructions, instructions.len() - 1).unwrap()
}

fn main() {
    advent::new(2024, 17, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "1,6,7,4,3,0,5,0,6");
    assert_eq!(part2(input), 216148338630253);
}
