use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 08)
}

struct Node {
    meta: Vec<i64>,
    children: Vec<Node>,
}

fn parse_node<I: Iterator<Item = i64>>(nums: &mut I) -> Node {
    let child_count = nums.nu();
    let meta_count = nums.nu();
    let children = (0..child_count).map(|_| parse_node(nums)).collect();
    let meta = nums.take(meta_count as usize).collect();

    Node { meta, children }
}

fn part1(input: &str) -> i64 {
    let root = parse_node(&mut input.nums());

    fn sum_meta(node: &Node) -> i64 {
        node.meta.iter().sum::<i64>() + node.children.iter().map(sum_meta).sum::<i64>()
    }

    sum_meta(&root)
}

fn part2(input: &str) -> i64 {
    let root = parse_node(&mut input.nums());

    fn value(node: &Node) -> i64 {
        if node.children.is_empty() {
            return node.meta.iter().sum::<i64>();
        }

        node.meta
            .iter()
            .flat_map(|i| node.children.get(*i as usize - 1))
            .map(value)
            .sum::<i64>()
    }

    value(&root)
}

fn main() {
    advent::new(2018, 08, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    assert_eq!(part1(input), 138);
    assert_eq!(part2(input), 66);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 48155);
    assert_eq!(part2(input), 40292);
}
