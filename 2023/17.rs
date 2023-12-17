use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 17)
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c.to_digit(10).unwrap() as i64);
    let goal = Vector2::new(grid.width() - 1, grid.height() - 1);

    #[derive(Debug, Clone, Copy)]
    struct Node {
        g: i64,
        h: i64,
    }

    fn cmp_node(a: Node, b: Node) -> Ordering {
        (a.g + a.h).cmp(&(b.g + b.h)).then(a.g.cmp(&b.g))
    }

    let mut to_search = FxHashMap::default();
    to_search.insert((Vector2::ZERO, Direction::Up), Node { g: 0, h: 0 });
    to_search.insert((Vector2::ZERO, Direction::Left), Node { g: 0, h: 0 });
    let mut processed = FxHashMap::default();

    loop {
        let (&cur, &node) = to_search
            .iter()
            .min_by(|(_, a), (_, b)| cmp_node(**a, **b))
            .unwrap();

        to_search.remove(&cur);
        processed.insert(cur, node);

        if cur.0 == goal {
            return node.g;
        }

        for dir in [
            Direction::Right,
            Direction::Down,
            Direction::Up,
            Direction::Left,
        ] {
            if dir == cur.1 || dir.reverse() == cur.1 {
                continue;
            }

            let mut cost = 0;
            for len in 1..=3 {
                let new_p = cur.0 + dir.vector() * len;
                if !grid.in_bounds(new_p) {
                    break;
                }

                cost += grid[new_p];

                let dist_to_n = node.g + cost;
                to_search
                    .entry((new_p, dir))
                    .and_modify(|e| e.g = e.g.min(dist_to_n))
                    .or_insert(Node {
                        g: dist_to_n,
                        h: goal.manhattan_dist(new_p),
                    });
            }
        }
    }
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c.to_digit(10).unwrap() as i64);
    let goal = Vector2::new(grid.width() - 1, grid.height() - 1);

    #[derive(Debug, Clone, Copy)]
    struct Node {
        g: i64,
        h: i64,
    }

    fn cmp_node(a: Node, b: Node) -> Ordering {
        (a.g + a.h).cmp(&(b.g + b.h)).then(a.g.cmp(&b.g))
    }

    let mut to_search = FxHashMap::default();
    to_search.insert((Vector2::ZERO, Direction::Up), Node { g: 0, h: 0 });
    to_search.insert((Vector2::ZERO, Direction::Left), Node { g: 0, h: 0 });
    let mut processed = FxHashMap::default();

    loop {
        let (&cur, &node) = to_search
            .iter()
            .min_by(|(_, a), (_, b)| cmp_node(**a, **b))
            .unwrap();

        to_search.remove(&cur);
        processed.insert(cur, node);

        if cur.0 == goal {
            return node.g;
        }

        for dir in [
            Direction::Right,
            Direction::Down,
            Direction::Up,
            Direction::Left,
        ] {
            if dir == cur.1 || dir.reverse() == cur.1 {
                continue;
            }

            let mut cost = 0;
            for len in 1..=10 {
                let new_p = cur.0 + dir.vector() * len;
                if !grid.in_bounds(new_p) {
                    break;
                }

                cost += grid[new_p];

                if len < 4 {
                    continue;
                }

                let dist_to_n = node.g + cost;
                to_search
                    .entry((new_p, dir))
                    .and_modify(|e| e.g = e.g.min(dist_to_n))
                    .or_insert(Node {
                        g: dist_to_n,
                        h: goal.manhattan_dist(new_p),
                    });
            }
        }
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    assert_eq!(part1(input), 102);
    assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 674);
    assert_eq!(part2(input), 773);
}
