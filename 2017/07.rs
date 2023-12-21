use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 07)
}

fn parse(prog: &str) -> (&str, i64, Vec<&str>) {
    let (name, rest) = prog.split_once(" (").unwrap();
    let (weight, children) = rest.split_once(")").unwrap();
    let weight = weight.parse().unwrap();
    let children = if let Some(children) = children.strip_prefix(" -> ") {
        children.split(", ").collect()
    } else {
        vec![]
    };
    (name, weight, children)
}

fn part1(input: &str) -> &str {
    let mut parents = HashMap::new();

    for (name, _, children) in input.lines().map(parse) {
        for child in children {
            parents.insert(child, name);
        }
    }

    let mut prog = *parents.keys().next().unwrap();
    while let Some(parent) = parents.get(prog) {
        prog = &parent;
    }
    prog
}

fn part2(input: &str) -> i64 {
    let mut parents = HashMap::new();
    let mut children = HashMap::new();
    let mut weights = HashMap::new();

    for (name, weight, c) in input.lines().map(parse) {
        for child in c.iter() {
            parents.insert(*child, name);
        }
        children.insert(name, c);
        weights.insert(name, weight);
    }

    let mut root = *parents.keys().next().unwrap();
    while let Some(parent) = parents.get(root) {
        root = &parent;
    }

    fn solve_weights<'a>(
        root: &'a str,
        children: &HashMap<&'a str, Vec<&'a str>>,
        weights: &HashMap<&str, i64>,
        tot_weights: &mut HashMap<&'a str, i64>,
    ) -> i64 {
        match tot_weights.get(root) {
            Some(&w) => w,
            None => {
                let w = *weights.get(root).unwrap()
                    + children
                        .get(root)
                        .unwrap()
                        .iter()
                        .map(|c| solve_weights(c, children, weights, tot_weights))
                        .sum::<i64>();
                tot_weights.insert(root, w);
                w
            }
        }
    }

    fn solve(
        node: &str,
        target_weight: i64,
        parents: &HashMap<&str, &str>,
        children: &HashMap<&str, Vec<&str>>,
        weights: &HashMap<&str, i64>,
        tot_weights: &HashMap<&str, i64>,
    ) -> Option<i64> {
        let my_weight = *tot_weights.get(node).unwrap();
        let my_children = children.get(node).unwrap();
        let my_children_weights = my_children
            .iter()
            .map(|c| tot_weights.get(c).unwrap())
            .collect_vec();

        if my_children_weights.iter().all_equal() {
            // children are balanced
            if my_weight == target_weight {
                // search my children
                for c in my_children {
                    let s = solve(
                        c,
                        *my_children_weights[0],
                        parents,
                        children,
                        weights,
                        tot_weights,
                    );
                    if s.is_some() {
                        return s;
                    }
                }

                None
            } else {
                // this node is the incorrect one
                Some(target_weight - my_children_weights.iter().copied().sum::<i64>())
            }
        } else {
            // children are unbalanced
            let mut counts: HashMap<i64, i64> = HashMap::new();
            for &w in &my_children_weights {
                *counts.entry(*w).or_default() += 1;
            }
            let (most_common, _) = counts.into_iter().max_by_key(|(_, c)| *c).unwrap();

            for (c, w) in my_children.iter().zip(my_children_weights.iter()) {
                if **w != most_common {
                    let s = solve(*c, most_common, parents, children, weights, tot_weights);
                    if s.is_some() {
                        return s;
                    }
                }
            }

            unreachable!()
        }
    }

    let mut tot_weights = HashMap::new();
    solve_weights(root, &children, &weights, &mut tot_weights);

    let root_weight = *tot_weights.get(root).unwrap();
    solve(
        root,
        root_weight,
        &parents,
        &children,
        &weights,
        &tot_weights,
    )
    .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    assert_eq!(part1(input), "tknk");
    assert_eq!(part2(input), 60);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "xegshds");
    assert_eq!(part2(input), 299);
}
