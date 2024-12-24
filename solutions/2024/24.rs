use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 24)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn eval(self, a: i64, b: i64) -> i64 {
        match self {
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction<'a> {
    v1: &'a str,
    v2: &'a str,
    op: Op,
    out: &'a str,
}

fn parse_instruction(i: &str) -> Instruction<'_> {
    let (v1, op, v2, _, out) = i.split(' ').tup();
    let op = match op {
        "OR" => Op::Or,
        "XOR" => Op::Xor,
        _ => Op::And,
    };
    Instruction { v1, v2, op, out }
}

fn parse_start_val(l: &str) -> (&str, i64) {
    let (var, val) = l.split_once(": ").unwrap();
    let val = val.nums().nu();
    (var, val)
}

fn parse<'a>(input: &'a str) -> (HashMap<&str, i64>, Vec<Instruction<'a>>) {
    let (vals, instructions) = input.sections().tup();

    (
        vals.lines().map(parse_start_val).collect(),
        instructions.lines().map(parse_instruction).collect(),
    )
}

fn part1(input: &str) -> i64 {
    let (mut values, instructions) = parse(input);

    let mut cont = true;
    while cont {
        cont = false;

        for ins in &instructions {
            if let (Some(&v1), Some(&v2)) = (values.get(ins.v1), values.get(ins.v2)) {
                values.insert(ins.out, ins.op.eval(v1, v2));
            } else {
                cont = true;
            }
        }
    }

    values
        .into_iter()
        .filter(|(v, _)| v.starts_with('z'))
        .sorted_by_key(|(v, _)| v.to_owned())
        .map(|(_, val)| val)
        .rev()
        .fold(0, |acc, x| (acc << 1 | x))
}

fn part2(input: &str) -> String {
    let (_, instructions) = parse(input);

    let mut suspicious = vec![];

    for ins in &instructions {
        let is_xy = ins.v1.starts_with(['x', 'y']);
        let is_z = ins.out.starts_with('z');

        let used_in_ops: HashSet<Op> = instructions
            .iter()
            .filter(|i| i.v1 == ins.out || i.v2 == ins.out)
            .map(|i| i.op)
            .unique()
            .collect();

        match ins.op {
            Op::And => {
                if is_z
                    || ((used_in_ops.contains(&Op::Xor) || used_in_ops.contains(&Op::And))
                        && !(ins.v1 == "x00" || ins.v1 == "y00"))
                {
                    suspicious.push(ins.out);
                }
            }
            Op::Or => {
                if is_xy || (is_z && ins.out != "z45") || used_in_ops.contains(&Op::Or) {
                    suspicious.push(ins.out);
                }
            }
            Op::Xor => {
                if !(is_xy || is_z) || used_in_ops.contains(&Op::Or) {
                    suspicious.push(ins.out);
                }
            }
        }
    }

    suspicious.sort();
    suspicious.into_iter().join(",")
}

fn main() {
    advent::new(2024, 24, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    assert_eq!(part1(input), 2024);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 57632654722854);
    assert_eq!(part2(input), "ckj,dbp,fdv,kdf,rpp,z15,z23,z39");
}
