use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 23)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
    Dir(Direction),
}

fn parse(cell: char) -> Cell {
    match cell {
        '#' => Cell::Wall,
        '.' => Cell::Empty,
        '^' => Cell::Dir(Direction::Up),
        '>' => Cell::Dir(Direction::Right),
        '<' => Cell::Dir(Direction::Left),
        'v' => Cell::Dir(Direction::Down),
        _ => panic!(),
    }
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, parse);
    let start = Vector2::new(1, 0);
    let end = Vector2::new(grid.width() - 2, grid.height() - 1);

    fn dfs(grid: &Grid<Cell>, seen: &mut HashSet<Vector2>, loc: Vector2, goal: Vector2) -> i64 {
        if !grid.in_bounds(loc) {
            return -100000;
        }
        if loc == goal {
            return 0;
        }

        let cell = grid[loc];
        let next = match cell {
            Cell::Wall => Vec::new(),
            Cell::Empty => loc.neighbors4().collect_vec(),
            Cell::Dir(d) => vec![loc + d.vector()],
        };

        let mut best = -10000;

        for n in next {
            if seen.contains(&n) {
                continue;
            }

            seen.insert(n);
            let dist = dfs(grid, seen, n, goal);
            best = best.max(dist + 1);
            seen.remove(&n);
        }

        best
    }

    let mut seen = HashSet::new();
    dfs(&grid, &mut seen, start, end)
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, parse);
    let start = Vector2::new(1, 0);
    let end = Vector2::new(grid.width() - 2, grid.height() - 1);

    fn find_group(
        grid: &Grid<Cell>,
        empties: &HashMap<Vector2, usize>,
        start: Vector2,
        end: Vector2,
    ) -> HashSet<(Vector2, usize, i64)> {
        let mut ends = HashSet::new();

        let mut edge = HashSet::new();
        let mut seen: HashSet<Vector2> = HashSet::new();
        let mut dist = 0;
        edge.insert(start);
        seen.insert(start);

        while !edge.is_empty() {
            let mut next = HashSet::new();

            for e in &edge {
                for n in e.neighbors4() {
                    if seen.contains(&n) || !grid.in_bounds(n) || n == start {
                        continue;
                    }

                    if n == end {
                        let empt = empties.get(&e).unwrap();
                        ends.insert((n, *empt, dist + 1));
                    }

                    let cell = grid[n];
                    match cell {
                        Cell::Wall => {}
                        Cell::Empty => {
                            next.insert(n);
                        }
                        Cell::Dir(_) => {
                            let empt = empties.get(&e).unwrap();
                            ends.insert((n, *empt, dist + 1));
                        }
                    }
                }
            }

            edge = next;
            seen.extend(&edge);
            dist += 1;
        }

        ends
    }

    fn find_empty(grid: &Grid<Cell>, seed: Vector2) -> HashSet<Vector2> {
        let mut edge = HashSet::new();
        let mut seen = HashSet::new();

        edge.insert(seed);
        seen.insert(seed);

        while !edge.is_empty() {
            let mut next = HashSet::new();

            for e in edge {
                for n in e.neighbors4() {
                    if grid.in_bounds(n) && !seen.contains(&n) && grid[n] == Cell::Empty {
                        next.insert(n);
                    }
                }
            }

            edge = next;
            seen.extend(&edge);
        }

        seen
    }

    let mut next_id = 0;
    let mut empties: HashMap<Vector2, usize> = HashMap::new();
    for p in grid.points() {
        if !empties.contains_key(&p) && matches!(grid[p], Cell::Empty) {
            for c in find_empty(&grid, p) {
                empties.insert(c, next_id);
            }
            next_id += 1;
        }
    }

    let mut groups: HashMap<Vector2, HashSet<(Vector2, usize, i64)>> = HashMap::new();

    for p in grid.points() {
        if p == start || matches!(grid[p], Cell::Dir(_)) {
            groups.insert(p, find_group(&grid, &empties, p, end));
        }
    }

    fn dfs(
        con: &HashMap<Vector2, HashSet<(Vector2, usize, i64)>>,
        seen: &mut HashSet<usize>,
        loc: Vector2,
        goal: Vector2,
    ) -> i64 {
        if loc == goal {
            return 0;
        }

        let mut best = -100000;

        for (n, e, d) in con.get(&loc).unwrap() {
            if seen.contains(e) {
                continue;
            }

            seen.insert(*e);
            let dist = dfs(con, seen, *n, goal);
            best = best.max(dist + d);
            seen.remove(e);
        }

        best
    }

    let mut seen = HashSet::new();
    dfs(&groups, &mut seen, start, end)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    assert_eq!(part1(input), 94);
    assert_eq!(part2(input), 154);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
