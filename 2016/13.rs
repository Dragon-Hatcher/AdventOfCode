use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!("31,39 " / 2016 / 13)
}

fn is_open(p: Vector2, key: i64) -> bool {
    p.x >= 0
        && p.y >= 0
        && (p.x * p.x + 3 * p.x + 2 * p.x * p.y + p.y + p.y * p.y + key).count_ones() % 2 == 0
}

fn part1(input: &str) -> i64 {
    fn a_star(start: Vector2, goal: Vector2, key: i64) -> i64 {
        #[derive(Debug, Clone, Copy)]
        struct Node {
            g: i64,
            h: i64,
        }

        fn cmp_node(a: Node, b: Node) -> Ordering {
            (a.g + a.h).cmp(&(b.g + b.h)).then(a.g.cmp(&b.g))
        }

        let mut to_search = HashMap::new();
        to_search.insert(start, Node { g: 0, h: 0 });
        let mut proccesed: HashMap<Vector2, Node> = HashMap::new();

        loop {
            let (&pos, &node) = to_search
                .iter()
                .min_by(|(_, a), (_, b)| cmp_node(**a, **b))
                .unwrap();

            to_search.remove(&pos);
            proccesed.insert(pos, node);

            if pos == goal {
                return node.g;
            }

            for n in pos
                .neighbors4()
                .filter(|&p| is_open(p, key) && !proccesed.contains_key(&p))
            {
                let dist_to_n = node.g + 1;
                to_search
                    .entry(n)
                    .and_modify(|e| e.g = e.g.min(dist_to_n))
                    .or_insert(Node {
                        g: dist_to_n,
                        h: pos.manhattan_dist(n),
                    });
            }
        }
    }

    let (x, y, key) = input.nums().tup();
    let start = Vector2::new(1, 1);
    let goal = Vector2::new(x, y);
    a_star(start, goal, key)
}

fn part2(input: &str) -> i64 {
    let (_, _, key) = input.nums().tup();
    let start = Vector2::new(1, 1);

    let mut edge = HashSet::new();
    let mut seen = HashSet::new();

    edge.insert(start);
    seen.insert(start);

    for _ in 0..50 {
        edge = edge
            .iter()
            .flat_map(|p| p.neighbors4().filter(|&p| is_open(p, key)))
            .collect();

        seen.extend(&edge);
    }

    seen.len() as i64
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "7,4 10";
    assert_eq!(part1(input), 11);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 96);
    assert_eq!(part2(input), 141);
}
