use advent::prelude::*;
use std::cmp::max_by_key;

fn default_input() -> &'static str {
    include_input!(2024 / 23)
}

fn parse(
    input: &str,
) -> (
    Vec<&str>,
    HashSet<(&str, &str)>,
    impl Fn(&str, &str) -> bool + '_,
) {
    let connections: HashSet<(&str, &str)> =
        input.lines().map(|l| l.split_once('-').unwrap()).collect();
    let all = connections
        .iter()
        .flat_map(|pair| [pair.0, pair.1])
        .sorted()
        .dedup()
        .collect_vec();

    let connections_copy = connections.clone();

    let are_connected =
        move |a: &str, b: &str| connections.contains(&(a, b)) || connections.contains(&(b, a));

    (all, connections_copy, are_connected)
}

fn part1(input: &str) -> i64 {
    let (all, connections, are_connected) = parse(input);

    let mut sum = 0;

    for (a, b) in connections {
        for &c in &all {
            if c == a || c == b {
                continue;
            }

            if are_connected(a, c)
                && are_connected(b, c)
                && (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
            {
                sum += 1;
            }
        }
    }

    // We count each connection three times, once for each leg of the triangle.
    sum / 3
}

fn part2(input: &str) -> String {
    let (all, _, are_connected) = parse(input);

    fn find<'a, F>(all: &[&'a str], current: Vec<&'a str>, are_connected: &'a F) -> Vec<&'a str>
    where
        F: Fn(&str, &str) -> bool,
    {
        if all.is_empty() {
            return current;
        }

        let next = all[0];
        let rest = &all[1..];

        let can_include = current.iter().all(|i| are_connected(i, next));

        let include = if can_include {
            let mut new = current.clone();
            new.push(next);
            find(rest, new, are_connected)
        } else {
            Vec::new()
        };

        let exclude = find(rest, current, are_connected);

        max_by_key(include, exclude, |l| l.len())
    }

    find(&all, Vec::new(), &are_connected)
        .into_iter()
        .sorted()
        .join(",")
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
    assert_eq!(part1(input), 7);
    assert_eq!(part2(input), "co,de,ka,ta");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1485);
    assert_eq!(part2(input), "cc,dz,ea,hj,if,it,kf,qo,sk,ug,ut,uv,wh");
}
