use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 08)
}

fn part1(input: &str) -> i64 {
    let (lr, lines) = input.sections().tup();

    let mut map = HashMap::new();

    for line in lines.lines() {
        let (from, to) = line.split_once(" = (").unwrap();
        let (l, r) = to.split_once(", ").unwrap();
        let r = r.strip_suffix(")").unwrap();

        map.insert(from.to_owned(), (l.to_owned(), r.to_owned()));
    }

    let mut loc = "AAA".to_owned();
    let mut steps = 0;
    let len = lr.chars().count();

    loop {
        let x = lr.chars().nth(steps % len).unwrap();
        loc = if x == 'L' {
            map[&loc].0.clone()
        } else {
            map[&loc].1.clone()
        };
        steps += 1;

        if loc == "ZZZ" {
            return steps as i64;
        }
    }
}

fn part2(input: &str) -> i64 {
    let (lr, lines) = input.sections().tup();

    let mut map = HashMap::new();

    for line in lines.lines() {
        let (from, to) = line.split_once(" = (").unwrap();
        let (l, r) = to.split_once(", ").unwrap();
        let r = r.strip_suffix(')').unwrap();

        map.insert(from.to_owned(), (l.to_owned(), r.to_owned()));
    }

    let mut loc = map
        .keys()
        .filter(|s| s.ends_with('A'))
        .cloned()
        .collect_vec();
    let mut steps = 0i64;
    let len = lr.chars().count();

    let mut iter_where = vec![0i64; loc.len()];
    let mut offset = vec![0; loc.len()];

    let mut l = 1;

    loop {
        let x = lr.chars().nth(steps as usize % len).unwrap();
        loc = loc
            .iter()
            .map(|l| {
                if x == 'L' {
                    map[l].0.clone()
                } else {
                    map[l].1.clone()
                }
            })
            .collect_vec();
        steps += l;
        dbg!(l, steps);

        // dbg!(steps);

        for i in 0..loc.len() {
            if loc[i].ends_with('Z') {
                let prev = iter_where[i];
                iter_where[i] = steps;
                offset[i] = steps - prev;
            }
        }

        if offset.iter().all(|c| *c != 0) {
            l = 1;
            for o in &offset {
                l = lcm(l, *o);
            }
            return l;
        }

        // dbg!(&offset);

        if loc.iter().all(|l| l.ends_with('Z')) {
            return steps as i64;
        }
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(
        part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
        ),
        6
    );
    assert_eq!(
        part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
        ),
        6
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 17263);
    assert_eq!(part2(input), 14631604759649);
}
