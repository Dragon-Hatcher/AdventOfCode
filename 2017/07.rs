use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 07)
}

#[derive(Debug, Clone, Default)]
struct Node<'a> {
    weight: i64,
    total_weight: i64,
    children: Vec<&'a str>,
    parent: Option<&'a str>,
}

fn parse_line(prog: &str) -> (&str, i64, Vec<&str>) {
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

fn parse(input: &str) -> HashMap<&str, Node<'_>> {
    let mut nodes: HashMap<&str, Node<'_>> = HashMap::new();

    for (name, weight, children) in input.lines().map(parse_line) {
        for &child in &children {
            nodes.entry(child).or_default().parent = Some(name);
        }

        let n = nodes.entry(name).or_default();
        n.weight = weight;
        n.children = children;
    }

    nodes
}

fn root<'a>(nodes: &HashMap<&'a str, Node<'a>>) -> &'a str {
    let mut root = *nodes.keys().nu();
    while let Some(parent) = nodes.get(root).unwrap().parent {
        root = parent;
    }
    root
}

fn part1(input: &str) -> &str {
    let nodes = parse(input);
    root(&nodes)
}

fn part2(input: &str) -> i64 {
    let mut nodes = parse(input);

    fn find_weights(root: &str, nodes: &mut HashMap<&str, Node<'_>>) -> i64 {
        let node = nodes.get_mut(root).unwrap();
        if node.total_weight == 0 {
            let child_weights: i64 = node
                .children
                .clone()
                .iter()
                .map(|c| find_weights(c, nodes))
                .sum();
            let node = nodes.get_mut(root).unwrap();
            node.total_weight = node.weight + child_weights;
        }
        let node = nodes.get_mut(root).unwrap();
        node.total_weight
    }

    let root = root(&nodes);
    find_weights(root, &mut nodes);

    fn solve(node: &str, target_weight: i64, nodes: &HashMap<&str, Node<'_>>) -> Option<i64> {
        let me = nodes.get(node).unwrap();
        let child_weights = me
            .children
            .iter()
            .map(|c| nodes.get(c).unwrap().total_weight)
            .collect_vec();

        if child_weights.iter().all_equal() {
            if me.total_weight == target_weight {
                for (i, child) in me.children.iter().enumerate() {
                    if let Some(w) = solve(*child, child_weights[i], nodes) {
                        return Some(w);
                    }
                }
                None
            } else {
                Some(target_weight - child_weights.iter().sum::<i64>())
            }
        } else {
            let mut counts: HashMap<i64, i64> = HashMap::new();
            for &w in &child_weights {
                *counts.entry(w).or_default() += 1;
            }
            let (most_common, _) = counts.into_iter().max_by_key(|(_, c)| *c).unwrap();
            let (&unbalanced_child, _) = me
                .children
                .iter()
                .zip(child_weights.iter())
                .find(|(_, &w)| w != most_common)
                .unwrap();

            solve(unbalanced_child, most_common, nodes)
        }
    }

    solve(root, nodes.get(root).unwrap().total_weight, &nodes).unwrap()
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
