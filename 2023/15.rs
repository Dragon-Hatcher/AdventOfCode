use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 15)
}

fn part1(input: &str) -> i64 {
    input
        .trim()
        .split(',')
        .map(|l| {
            let mut hash = 0;
            for c in l.chars() {
                hash += c as i64;
                hash *= 17;
                hash %= 256;
            }
            hash
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut map: HashMap<i64, Vec<(String, i64)>> = HashMap::new();

    for i in input.trim().split(',') {
        let (code, rest) = i.split_once(['=', '-']).unwrap();
        let mut hash = 0;
        for c in code.chars() {
            hash += c as i64;
            hash *= 17;
            hash %= 256;
        }

        let e = map.entry(hash).or_default();
        if rest.is_empty() {
            if let Some(idx) = e.iter().position(|(c, _)| code == c) {
                e.remove(idx);
            }
        } else {
            if let Some(idx) = e.iter().position(|(c, _)| code == c) {
                e[idx] = (code.to_owned(), rest.nums().nu());
            } else {
                e.push((code.to_owned(), rest.nums().nu()))
            }
        }
    }

    map.iter()
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
