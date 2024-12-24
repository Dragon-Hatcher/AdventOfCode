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
    fn act(self, a: i64, b: i64) -> i64 {
        match self {
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
        }
    }
}

fn part1(input: &str) -> i64 {
    //     let input = "x00: 1
    // x01: 0
    // x02: 1
    // x03: 1
    // x04: 0
    // y00: 1
    // y01: 1
    // y02: 1
    // y03: 1
    // y04: 1

    // ntg XOR fgs -> mjb
    // y02 OR x01 -> tnw
    // kwq OR kpj -> z05
    // x00 OR x03 -> fst
    // tgd XOR rvg -> z01
    // vdt OR tnw -> bfw
    // bfw AND frj -> z10
    // ffh OR nrd -> bqk
    // y00 AND y03 -> djm
    // y03 OR y00 -> psh
    // bqk OR frj -> z08
    // tnw OR fst -> frj
    // gnj AND tgd -> z11
    // bfw XOR mjb -> z00
    // x03 OR x00 -> vdt
    // gnj AND wpb -> z02
    // x04 AND y00 -> kjc
    // djm OR pbm -> qhw
    // nrd AND vdt -> hwm
    // kjc AND fst -> rvg
    // y04 OR y02 -> fgs
    // y01 AND x02 -> pbm
    // ntg OR kjc -> kwq
    // psh XOR fgs -> tgd
    // qhw XOR tgd -> z09
    // pbm OR djm -> kpj
    // x03 XOR y03 -> ffh
    // x00 XOR y04 -> ntg
    // bfw OR bqk -> z06
    // nrd XOR fgs -> wpb
    // frj XOR qhw -> z04
    // bqk OR frj -> z07
    // y03 OR x01 -> nrd
    // hwm AND bqk -> z03
    // tgd XOR rvg -> z12
    // tnw OR pbm -> gnj";

    let (start, form) = input.sections().tup();

    let mut vals: HashMap<&str, i64> = start
        .lines()
        .map(|l| {
            let (var, val) = l.split_once(": ").unwrap();
            let val = val.nums().nu();
            (var, val)
        })
        .collect();

    let formulas = form
        .lines()
        .map(|l| {
            let (v1, op, v2, _, vo) = l.split(' ').tup();
            let op = match op {
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => Op::And,
            };
            (v1, op, v2, vo)
        })
        .collect_vec();

    // dbg!(&formulas);

    loop {
        let mut cont = false;

        for (v1, op, v2, vo) in &formulas {
            // dbg!(v1, v2);
            let Some(v1) = vals.get(v1) else {
                cont = true;
                continue;
            };
            let Some(v2) = vals.get(v2) else {
                cont = true;
                continue;
            };
            // println!("put {vo}");
            vals.insert(vo, op.act(*v1, *v2));
        }

        // dbg!(&vals.len());

        if !cont {
            break;
        }
    }

    vals.into_iter()
        .filter(|(v, _)| v.starts_with('z'))
        .sorted_by_key(|(v, _)| v.to_owned())
        .map(|(_, val)| val)
        .rev()
        .fold(0, |acc, x| (acc << 1 | x))
}

fn get<'a>(
    form: &HashMap<(&'a str, Op, &'a str), &'a str>,
    v1: &'a str,
    op: Op,
    v2: &'a str,
) -> Option<&'a str> {
    form.get(&(v1, op, v2)).or(form.get(&(v2, op, v1))).copied()
}

fn part2(input: &str) -> i64 {
    let (start, form) = input.sections().tup();

    // let mut vals: HashMap<&str, i64> = start
    //     .lines()
    //     .map(|l| {
    //         let (var, val) = l.split_once(": ").unwrap();
    //         let val = val.nums().nu();
    //         (var, val)
    //     })
    //     .collect();

    let formulas: HashMap<(&str, Op, &str), &str> = form
        .lines()
        .map(|l| {
            let (v1, op, v2, _, vo) = l.split(' ').tup();
            let op = match op {
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => Op::And,
            };
            ((v1, op, v2), vo)
        })
        .collect();

    let mut carry_in = "rfg";

    // x15 And y15 -> z15
    // rqt Or rdt -> z23
    // vbt And vqr -> z39

    // let get =
    //     |v1: &str, op: Op, v2: &str| formulas.get(&(v1, op, v2)).or(formulas.get(&(v2, op, v1)));

    for i in 1..=44 {
        // let z = format!("z{i:02}").leak();

        // let ((v1, op, v2), _) = formulas
        //     .iter()
        //     .find(|(_, out)| **out == z)
        //     .unwrap();

        // if op != &Op::Xor {
        //     println!("{v1} {op:?} {v2} -> {z}");
        // }

        // assert_eq!(op, &Op::Xor);

        // let ((v11, op1, v12), _) = formulas
        //     .iter()
        //     .find(|(_, out)| *out == v1)
        //     .unwrap();
        // let ((v21, op2, v22), _) = formulas
        //     .iter()
        //     .find(|(_, out)| *out == v2)
        //     .unwrap();

        // if op1 == op2 {
        //     println!("hmm");
        // }
    }
    /*
    "z15","z23","z39","kdf","ckj","z23","rpp","z45",*/

    /*
    jwh Or tmh -> z45
    jwh Or tmh -> z45
    nsr Xor gsd -> kdf
    qbw Xor fqf -> ckj
    rqt Or rdt -> z23
    rqt Or rdt -> z23
    vbt And vqr -> z39
    vqr Xor vbt -> rpp
    x15 And y15 -> z15

nsr Xor gsd -> kdf
qbw Xor fqf -> ckj
x15 And y15 -> z15
rqt Or rdt -> z23
vjf And fdv -> wdv
vqr Xor vbt -> rpp
vbt And vqr -> z39
hnr And kdf -> htp
jwh Or tmh -> z45


nsr Xor gsd -> kdf
qbw Xor fqf -> ckj
x15 And y15 -> z15
rqt Or rdt -> z23
y06 And x06 -> fdv
vqr Xor vbt -> rpp
vbt And vqr -> z39

    */


    // ckj,dbp,fdv,kdf,rpp,z15,z23,z39

    for ((v1, op, v2), vo) in formulas.clone().into_iter() {
        // if v1 == "x00" && op == Op::Xor {
        //     dbg!(v1, v2, op == Op::Xor
        //         && ((v1.starts_with(['x', 'y']) || v2.starts_with(['x', 'y'])) == vo.starts_with('z')));
        //     break;
        // }

        if op == Op::Xor&& !v1.starts_with(['x', 'y']) {
            let ((v11, op1, v12), vo1) = formulas.iter().find(|(_, out)| **out == v1).unwrap();
            let ((v21, op2, v22), vo2) = formulas.iter().find(|(_, out)| **out == v2).unwrap();

            if !(
                op1 != op2
                    && (op1 == &Op::Xor || op2 == &Op::Xor)
                    && (op1 == &Op::Or || op2 == &Op::Or)
            ) {
                println!("??{v1} {op:?} {v2} -> {vo}");
                println!("    {v11} {op1:?} {v12} -> {vo1}");
                println!("    {v21} {op2:?} {v22} -> {vo2}");
            }
        }

        if op == Op::Or {
            let ((v11, op1, v12), vo1) = formulas.iter().find(|(_, out)| **out == v1).unwrap();
            let ((v21, op2, v22), vo2) = formulas.iter().find(|(_, out)| **out == v2).unwrap();

            if (
                op1 != &Op::And || op2 != &Op::And
            ) {
                println!("??{v1} {op:?} {v2} -> {vo}");
                println!("    {v11} {op1:?} {v12} -> {vo1}");
                println!("    {v21} {op2:?} {v22} -> {vo2}");
            }
        }

        if vo.starts_with('z') && op != Op::Xor {
            println!("{v1} {op:?} {v2} -> {vo}");
        } else if op == Op::Xor
            && !(v1.starts_with(['x', 'y']) || v2.starts_with(['x', 'y']) || vo.starts_with('z'))
        {
            println!("{v1} {op:?} {v2} -> {vo}");

            let ((v11, op1, v12), vo1) = formulas.iter().find(|(_, out)| **out == v1).unwrap();
            let ((v21, op2, v22), vo2) = formulas.iter().find(|(_, out)| **out == v2).unwrap();

            // if !(
            //     op1 != op2
            //         && (op1 == &Op::Xor || op2 == &Op::Xor)
            //         && (op1 == &Op::Or || op2 == &Op::Or)
            // ) {
                // println!("  {v11} {op1:?} {v12} -> {vo1}");
                // println!("  {v21} {op2:?} {v22} -> {vo2}");
            // }
        } else if op == Op::Or
            && (v1.starts_with(['x', 'y']) || v2.starts_with(['x', 'y']) || vo.starts_with('z'))
        {
            println!("{v1} {op:?} {v2} -> {vo}");
        } else {
            if op == Op::And && !v1.starts_with(['x', 'y']) {
                // dbg!(v1, v2);
                let ((v11, op1, v12), vo1) = formulas.iter().find(|(_, out)| **out == v1).unwrap();
                let ((v21, op2, v22), vo2) = formulas.iter().find(|(_, out)| **out == v2).unwrap();

                if !(
                    op1 != op2
                        && (op1 == &Op::Xor || op2 == &Op::Xor)
                        && (op1 == &Op::Or || op2 == &Op::Or)
                ) {
                    println!("  {v1} {op:?} {v2} -> {vo}");
                    println!("    {v11} {op1:?} {v12} -> {vo1}");
                    println!("    {v21} {op2:?} {v22} -> {vo2}");
                }
            }
        }
    }

    // for i in 1..43 {
    //     let x1 = format!("x{i:02}").leak();
    //     let y1 = format!("y{i:02}").leak();
    //     // let x2 = format!("x{:02}", i + 1);
    //     // let y2 = format!("y{:02}", i + 1);

    //     let and1 = get(&formulas, x1, Op::And, y1).unwrap();
    //     let xor1 = get(&formulas, x1, Op::Xor, y1).unwrap();

    //     dbg!(xor1, carry_in);

    //     let xor2 = get(&formulas, xor1, Op::Xor, carry_in).unwrap();
    //     let and2 = get(&formulas, xor1, Op::And, carry_in).unwrap();
    //     let cout = get(&formulas, and2, Op::Or, and1).unwrap();

    //     if xor2 != format!("z{i:02}") {
    //         println!("!! xor2 {i} == {xor2}");
    //     }

    //     carry_in = cout;

    //     // let and2 = formulas[&(x2.as_ref(), Op::And, y2.as_ref())];
    //     // let xor2 = formulas[&(x2.as_ref(), Op::Xor, y2.as_ref())];
    // }

    0
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
    assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
