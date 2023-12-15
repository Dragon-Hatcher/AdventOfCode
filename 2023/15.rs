use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 15)
}

fn hash(code: &str) -> i64 {
    let mut hash = 0;
    for c in code.chars() {
        hash += c as i64;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn part1(input: &str) -> i64 {
    input.trim().split(',').map(hash).sum()
}

fn part2(input: &str) -> i64 {
    let mut lenses: HashMap<i64, Vec<(String, i64)>> = HashMap::new();

    for instruction in input.trim().split(',') {
        let (code, rest) = instruction.split_once(['=', '-']).unwrap();
        let hash = hash(code);
        let lens_box = lenses.entry(hash).or_default();
        let current_pos = lens_box.iter().position(|(c, _)| code == c);

        match (rest, current_pos) {
            ("", Some(idx)) => {
                lens_box.remove(idx);
            }
            ("", None) => {}
            (_, Some(idx)) => {
                lens_box[idx].1 = rest.parse().unwrap();
            }
            (_, None) => lens_box.push((code.to_owned(), rest.parse().unwrap())),
        }
    }

    lenses
        .iter()
        .flat_map(|(&b, vs)| {
            vs.iter()
                .enumerate()
                .map(move |(i, (_, f))| (b + 1) * (i as i64 + 1) * f)
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part1(input), 1320);
    assert_eq!(part2(input), 145);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 511416);
    assert_eq!(part2(input), 290779);
}
