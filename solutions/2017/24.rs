use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 24)
}

fn part1(input: &str) -> i64 {
    let mut graph = UnGraph::new();
    for (a, b) in input.lines().map(|l| l.nums().tup()) {
        let ai = graph.ni_or_insert(a);
        let bi = graph.ni_or_insert(b);
        graph.new_edge(ai, bi, a + b);
    }

    fn max_dist(graph: &UnGraph<i64, i64>, at: NodeIndex, used: &mut HashSet<EdgeIndex>) -> i64 {
        let mut d = 0;

        for edge in graph
            .outgoing(at)
            .filter(|e| !used.contains(&e.index()))
            .collect_vec()
        {
            used.insert(edge.index());
            d = d.max(edge.weight() + max_dist(graph, edge.other_end(at), used));
            used.remove(&edge.index());
        }

        d
    }

    let start = graph.node_index(&0);
    let mut visited = HashSet::default();
    max_dist(&graph, start, &mut visited)
}

fn part2(input: &str) -> i64 {
    let mut graph = UnGraph::new();
    for (a, b) in input.lines().map(|l| l.nums().tup()) {
        let ai = graph.ni_or_insert(a);
        let bi = graph.ni_or_insert(b);
        graph.new_edge(ai, bi, a + b);
    }

    fn max_path_length(
        graph: &UnGraph<i64, i64>,
        at: NodeIndex,
        used: &mut HashSet<EdgeIndex>,
    ) -> i64 {
        let mut l = 0;

        for edge in graph
            .outgoing(at)
            .filter(|e| !used.contains(&e.index()))
            .collect_vec()
        {
            used.insert(edge.index());
            l = l.max(1 + max_path_length(graph, edge.other_end(at), used));
            used.remove(&edge.index());
        }

        l
    }

    fn max_dist(
        graph: &UnGraph<i64, i64>,
        at: NodeIndex,
        used: &mut HashSet<EdgeIndex>,
        so_far: i64,
        length: i64,
    ) -> i64 {
        let mut d = 0;

        for edge in graph
            .outgoing(at)
            .filter(|e| !used.contains(&e.index()))
            .collect_vec()
        {
            used.insert(edge.index());
            d = d.max(max_dist(graph, edge.other_end(at), used, so_far + edge.weight(), length - 1));
            used.remove(&edge.index());
        }

        if d == 0 && length == 0 {
            return so_far
        }

        d
    }

    let start = graph.node_index(&0);
    let mut visited = HashSet::default();
    let length = max_path_length(&graph, start, &mut visited);
    visited.clear();
    max_dist(&graph, start, &mut visited, 0, length)
}

fn main() {
    advent::new(2017, 24, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
    assert_eq!(part1(input), 31);
    assert_eq!(part2(input), 19);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1859);
    assert_eq!(part2(input), 1799);
}
