use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 17)
}

fn run(instructions: &[i64], mut a: i64, mut b: i64, mut c: i64) -> Vec<i64> {
    let mut ip = 0;
    let mut out = vec![];

    while ip < instructions.len() {
        let instr = instructions[ip];
        let operand = instructions[ip + 1];

        // dbg!(ip, instr, operand);

        let combo = match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            _ => c,
        };

        match instr {
            0 => {
                if combo > 63 {
                    a = 0;
                } else {
                    a = a / 2i64.pow(combo as u32);
                }
                ip += 2;
            }
            1 => {
                b = b ^ operand;
                ip += 2;
            }
            2 => {
                b = combo % 8;
                ip += 2;
            }
            3 => {
                if a != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            4 => {
                b = b ^ c;
                ip += 2;
            }
            5 => {
                out.push(combo % 8);
                ip += 2;
            }
            6 => {
                if combo > 63 {
                    b = 0;
                } else {
                    b = a / 2i64.pow(combo as u32);
                }
                ip += 2;
            }
            _ => {
                if combo > 63 {
                    c = 0;
                } else {
                    c = a / 2i64.pow(combo as u32);
                }
                ip += 2;
            }
        }
    }

    out
}

fn part1(input: &str) -> String {
    let (start, program) = input.sections().tup();
    let (a, b, c) = start.nums().tup();
    // let a = 216140366357152;
    // let a = 216148338629303;
    let a = 216148338630253;
    let instructions = program.nums().collect_vec();

    run(&instructions, a, b, c).into_iter().join(",")
}

fn find(a: i64, targets: &[i64], on: usize, unrev: &[i64]) -> Option<i64> {
    if on == targets.len() {
        let a = a / 8;
        dbg!(a);
        let mut out = run(&unrev, a, 0, 0);
        println!("{out:?}");
        println!("{targets:?}");
        if out == unrev {
            return Some(a);
        } else {
            return None;
        }
    }

    for p in 1..8 {
        let a = a + p;

        let b1 = (a % 8) ^ 3;
        let c = a / (2i64.pow(b1 as u32));
        let b2 = b1 ^ 5 ^ c;

        if b2 % 8 == targets[on] {
            println!("{on} -> {p}");
            let found = find(a * 8, targets, on + 1, unrev);
            if let Some(ans) = found {
                return Some(ans);
            }
        }
    }

    None
}

fn part2(input: &str) -> i64 {
    let (start, program) = input.sections().tup();
    let (_, _, _) = start.nums().tup();
    let mut instructions = program.nums().collect_vec();
    let mut targets = instructions.clone();

    targets.reverse();
    dbg!(find(0, &targets, 0, &instructions));

    10

    // let mut a = 0;

    // for target in instructions.into_iter().rev() {
    //     a = a * 8;
    //     let start = a;

    //     loop {
    //         a += 1;

    //         let b1 = (a % 8) ^ 3;
    //         let c = a / (2i64.pow(b1 as u32));
    //         let b2 = b1 ^ 5 ^ c;

    //         if b2 % 8 == target {
    //             break;
    //         }

    //     }

    //     dbg!(a - start);

    // }

    // a
}

/*

Register A: 63687530
Register B: 0
Register C: 0

Program: 2,4,1,3,7,5,0,3,1,5,4,1,5,5,3,0

while a != 0 {
    b = a % 8    (2,4)
    b = b ^ 3    (1,3)
    c = a / 2**b  (7,5)
    a = a / 8    (0,3)
    b = b ^ 5    (1,5)
    b = b ^ c    (4,1)
    print b % 8  (5,5)
}


*/

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
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
