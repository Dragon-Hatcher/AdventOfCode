use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 24)
}

type Port = i64;

fn parse(input: &str) -> UnGraph<Port, i64> {
    let mut graph = UnGraph::new();
    for (a, b) in input.lines().map(|l| l.nums().tup()) {
        graph.add_edge(a, b, a + b);
    }
    graph
}

fn part1(input: &str) -> i64 {
    let graph = parse(input);

    fn max_dist(
        graph: &UnGraph<Port, i64>,
        at: Port,
        visited: &mut ExtraEdgeInfo<Port, bool>,
    ) -> i64 {
        let mut dist = 0;

        for (to, weight) in graph.outgoing(at) {
            if *visited.get(at, to) {
                continue;
            }

            *visited.get_mut(at, to) = true;
            let new_dist = weight + max_dist(graph, to, visited);
            *visited.get_mut(at, to) = false;

            dist = dist.max(new_dist);
        }

        dist
    }

    max_dist(&graph, 0, &mut ExtraEdgeInfo::new())
}

fn part2(input: &str) -> i64 {
    let graph = parse(input);

    fn max_dist(
        graph: &UnGraph<Port, i64>,
        at: Port,
        visited: &mut ExtraEdgeInfo<Port, bool>,
        so_far: (i64, i64),
    ) -> (i64, i64) {
        let mut dist = so_far;

        for (to, weight) in graph.outgoing(at) {
            if *visited.get(at, to) {
                continue;
            }

            *visited.get_mut(at, to) = true;
            let new_dist = max_dist(graph, to, visited, (so_far.0 + 1, so_far.1 + weight));
            *visited.get_mut(at, to) = false;

            dist = dist.max(new_dist);
        }

        dist
    }

    let (_length, strength) = max_dist(&graph, 0, &mut ExtraEdgeInfo::new(), (0, 0));

    strength
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
    // assert_eq!(part2(input), 19);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 1859);
    // assert_eq!(part2(input), 1799);
}
