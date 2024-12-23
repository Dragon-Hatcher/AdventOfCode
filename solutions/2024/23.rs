use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 23)
}

fn part1(input: &str) -> i64 {
    let conn: HashSet<_> = input.lines().map(|l| l.split_once('-').unwrap()).collect();

    let all: HashSet<_> = conn.iter().flat_map(|x| [x.0, x.1]).collect();

    let conn = |a: &str, b: &str| conn.contains(&(a, b)) || conn.contains(&(b, a));

    let mut sum = 0;
    for x in &all {
        for y in &all {
            if x >= y {
                continue;
            }

            for z in &all {
                if y >= z || x >= z {
                    continue;
                }

                if conn(x, y) && conn(x, z) && conn(y, z) {
                    if x.starts_with('t') || y.starts_with('t') || z.starts_with('t') {
                        sum += 1;
                    }
                }
            }
        }
    }

    sum
}

fn part2(input: &str) -> String {
    let conns: HashSet<_> = input.lines().map(|l| l.split_once('-').unwrap()).collect();
    let all: HashSet<_> = conns.iter().flat_map(|x| [x.0, x.1]).collect();
    let all = all.into_iter().collect_vec();
    let conn = |a: &str, b: &str| conns.contains(&(a, b)) || conns.contains(&(b, a));

    fn find<'a, F>(all: &[&'a str], include: Vec<&'a str>, conn: &'a F) -> Vec<&'a str>
    where F: Fn(&str, &str) -> bool {
        if all.is_empty() {
            return include;
        }

        let curr = all[0];
        let rest = &all[1..];

        let opt_a = if include.iter().all(|i| conn(i, curr)) {
            let mut new = include.clone();
            new.push(curr);
            find(rest, new, conn)
        } else {
            Vec::new()
        };

        let opt_b = find(rest, include, conn);

        if opt_a.len() > opt_b.len() { opt_a } else { opt_b }
    }

    find(&all, Vec::new(), &conn).into_iter().sorted().join(",")
}

fn main() {
    advent::new(2024, 23, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    // assert_eq!(part1(input), 0);
    assert_eq!(part2(input), "xx");
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
