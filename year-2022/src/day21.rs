use crate::standard_parsers::AocParsed;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operator {
    fn eval(self, left: i64, right: i64) -> i64 {
        match self {
            Operator::Plus => left.saturating_add(right),
            Operator::Minus => left.saturating_sub(right),
            Operator::Multiply => left.saturating_mul(right),
            Operator::Divide => left / right,
        }
    }
}

#[derive(Debug, Clone)]
enum Monkey {
    Number(i64),
    Equation {
        left: String,
        op: Operator,
        right: String,
    },
}

fn parse(input: &str) -> FxHashMap<String, Monkey> {
    let mut res = FxHashMap::default();

    input.non_empty().for_each(|l| {
        let name = l[0..4].to_owned();
        if let Some(n) = l.nums().next() {
            res.insert(name, Monkey::Number(n));
        } else {
            let left = l[6..10].to_owned();
            let op = match l.chars().nth(11).unwrap() {
                '+' => Operator::Plus,
                '-' => Operator::Minus,
                '*' => Operator::Multiply,
                _ => Operator::Divide,
            };
            let right = l[13..17].to_owned();
            res.insert(name, Monkey::Equation { left, op, right });
        }
    });

    res
}

fn get_value(monkeys: &mut FxHashMap<String, Monkey>, name: &str) -> i64 {
    match monkeys.get(name).unwrap() {
        Monkey::Number(n) => *n,
        Monkey::Equation { left, op, right } => {
            let left = left.clone();
            let right = right.clone();
            let op = *op;
            let left_val = get_value(monkeys, &left);
            let right_val = get_value(monkeys, &right);
            let value = op.eval(left_val, right_val);
            monkeys.insert(name.to_owned(), Monkey::Number(value));
            value
        }
    }
}

///
/// --- Day 21: Monkey Math ---
///
/// The [monkeys](11) are back! You're worried they're going to try to steal your
/// stuff again, but it seems like they're just holding their ground and making various
/// monkey noises at you.
///
/// Eventually, one of the elephants realizes you don't speak monkey and comes over
/// to interpret. As it turns out, they overheard you talking about trying to find
/// the grove; they can show you a shortcut if you answer their *riddle*.
///
/// Each monkey is given a *job*: either to *yell a specific number* or to *yell
/// the result of a math operation*. All of the number-yelling monkeys know their
/// number from the start; however, the math operation monkeys need to wait for two
/// other monkeys to yell a number, and those two other monkeys might *also* be waiting
/// on other monkeys.
///
/// Your job is to *work out the number the monkey named `root` will yell* before
/// the monkeys figure it out themselves.
///
/// For example:
///
/// ```
/// root: pppw + sjmn
/// dbpl: 5
/// cczh: sllz + lgvd
/// zczc: 2
/// ptdq: humn - dvpt
/// dvpt: 3
/// lfqf: 4
/// humn: 5
/// ljgn: 2
/// sjmn: drzm * dbpl
/// sllz: 4
/// pppw: cczh / lfqf
/// lgvd: ljgn * ptdq
/// drzm: hmdt - zczc
/// hmdt: 32
///
/// ```
///
/// Each line contains the name of a monkey, a colon, and then the job of that monkey:
///
/// * A lone number means the monkey's job is simply to yell that number.
/// * A job like `aaaa + bbbb` means the monkey waits for monkeys `aaaa` and `bbbb`
/// to yell each of their numbers; the monkey then yells the sum of those two numbers.
/// * `aaaa - bbbb` means the monkey yells `aaaa`'s number minus `bbbb`'s number.
/// * Job `aaaa * bbbb` will yell `aaaa`'s number multiplied by `bbbb`'s number.
/// * Job `aaaa / bbbb` will yell `aaaa`'s number divided by `bbbb`'s number.
///
/// So, in the above example, monkey `drzm` has to wait for monkeys `hmdt` and `zczc`
/// to yell their numbers. Fortunately, both `hmdt` and `zczc` have jobs that involve
/// simply yelling a single number, so they do this immediately: `32` and `2`. Monkey
/// `drzm` can then yell its number by finding `32` minus `2`: `*30*`.
///
/// Then, monkey `sjmn` has one of its numbers (`30`, from monkey `drzm`), and already
/// has its other number, `5`, from `dbpl`. This allows it to yell its own number
/// by finding `30` multiplied by `5`: `*150*`.
///
/// This process continues until `root` yells a number: `*152*`.
///
/// However, your actual situation involves considerably more monkeys. *What number
/// will the monkey named `root` yell?*
///
pub fn part1(input: &str) -> i64 {
    let mut monkeys = parse(input);
    get_value(&mut monkeys, "root")
}

// fn newtons_method() -> i64 {
//
// }

///
/// --- Part Two ---
///
/// Due to some kind of monkey-elephant-human mistranslation, you seem to have misunderstood
/// a few key details about the riddle.
///
/// First, you got the wrong job for the monkey named `root`; specifically, you got
/// the wrong math operation. The correct operation for monkey `root` should be `=`,
/// which means that it still listens for two numbers (from the same two monkeys
/// as before), but now checks that the two numbers *match*.
///
/// Second, you got the wrong monkey for the job starting with `humn:`. It isn't
/// a monkey - it's *you*. Actually, you got the job wrong, too: you need to figure
/// out *what number you need to yell* so that `root`'s equality check passes. (The
/// number that appears after `humn:` in your input is now irrelevant.)
///
/// In the above example, the number you need to yell to pass `root`'s equality test
/// is `*301*`. (This causes `root` to get the same number, `150`, from both of its
/// monkeys.)
///
/// *What number do you yell to pass `root`'s equality test?*
///
pub fn part2(input: &str) -> i64 {
    fn eval(monkeys: &FxHashMap<String, Monkey>, n: i64) -> i64 {
        let mut monkeys = monkeys.clone();
        monkeys.insert("humn".to_owned(), Monkey::Number(n));
        get_value(&mut monkeys, "root")
    }

    let mut monkeys = parse(input);

    let Monkey::Equation { op, .. } = monkeys.get_mut("root").unwrap() else { panic!() };
    *op = Operator::Minus;

    let mut low = i64::MIN;
    let mut high = i64::MAX;

    if eval(&monkeys, low) > 0 {
        (low, high) = (high, low);
    }

    loop {
        let midpoint = low / 2 + high / 2 + (low % 2 + high % 2) / 2;
        let value = eval(&monkeys, midpoint);
        match value.cmp(&0) {
            Ordering::Less => low = midpoint,
            Ordering::Equal => break midpoint,
            Ordering::Greater => high = midpoint,
        }
    }
}

const PART1_EX_ANSWER: &str = "152";
const PART1_ANSWER: &str = "110181395003396";
const PART2_EX_ANSWER: &str = "301";
const PART2_ANSWER: &str = "3721298272959";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
