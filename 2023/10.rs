use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 10)
}

fn loop_length(
    grid: &mut Grid<(bool, bool, bool, bool, bool)>,
    start: Vector2,
    shape: (bool, bool, bool, bool, bool),
) -> Option<(HashSet<Vector2>, HashSet<Vector2>)> {
    grid[start] = shape;

    let mut prev = start;
    let mut curr = start;
    let mut visited = HashSet::new();
    let mut doubled = HashSet::new();

    loop {
        visited.insert(curr);
        doubled.insert(curr * 2);

        if grid[curr].0 && prev != curr - Vector2::E2 && grid.in_bounds(curr - Vector2::E2) {
            let next = curr - Vector2::E2;
            doubled.insert(curr * 2 - Vector2::E2);
            prev = curr;
            curr = next;
            if next == start {
                return Some((visited, doubled));
            }
        } else if grid[curr].1 && prev != curr + Vector2::E2 && grid.in_bounds(curr + Vector2::E2) {
            let next = curr + Vector2::E2;
            doubled.insert(curr * 2 + Vector2::E2);
            prev = curr;
            curr = next;
            if next == start {
                return Some((visited, doubled));
            }
        } else if grid[curr].2 && prev != curr + Vector2::E1 && grid.in_bounds(curr + Vector2::E1) {
            let next = curr + Vector2::E1;
            doubled.insert(curr * 2 + Vector2::E1);
            prev = curr;
            curr = next;
            if next == start {
                return Some((visited, doubled));
            }
        } else if grid[curr].3 && prev != curr - Vector2::E1 && grid.in_bounds(curr - Vector2::E1) {
            let next = curr - Vector2::E1;
            doubled.insert(curr * 2 - Vector2::E1);
            prev = curr;
            curr = next;
            if next == start {
                return Some((visited, doubled));
            }
        } else {
            return None;
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut grid = Grid::new_by_char(input, |c| match c {
        '|' => (true, true, false, false, false),
        '-' => (false, false, true, true, false),
        'L' => (true, false, true, false, false),
        'J' => (true, false, false, true, false),
        '7' => (false, true, false, true, false),
        'F' => (false, true, true, false, false),
        '.' => (false, false, false, false, false),
        'S' => (true, true, true, true, true),
        _ => panic!(),
    });

    let start = grid.points().find(|p| grid[*p].4).unwrap();

    for t in [
        (true, true, false, false, false),
        (false, false, true, true, false),
        (true, false, true, false, false),
        (true, false, false, true, false),
        (false, true, false, true, false),
        (false, true, true, false, false),
        (false, false, false, false, false),
    ] {
        if let Some((visited, _)) = loop_length(&mut grid, start, t) {
            return visited.len() as i64 / 2;
        }
    }

    unreachable!()
}

fn part2(input: &str) -> i64 {
    let mut grid = Grid::new_by_char(input, |c| match c {
        '|' => (true, true, false, false, false),
        '-' => (false, false, true, true, false),
        'L' => (true, false, true, false, false),
        'J' => (true, false, false, true, false),
        '7' => (false, true, false, true, false),
        'F' => (false, true, true, false, false),
        '.' => (false, false, false, false, false),
        'S' => (true, true, true, true, true),
        _ => panic!("{c}"),
    });

    let start = grid.points().find(|p| grid[*p].4).unwrap();

    for t in [
        (true, true, false, false, false),
        (false, false, true, true, false),
        (true, false, true, false, false),
        (true, false, false, true, false),
        (false, true, false, true, false),
        (false, true, true, false, false),
        (false, false, false, false, false),
    ] {
        if let Some((_, doubled)) = loop_length(&mut grid, start, t) {
            let mut all_adjacent: HashSet<_> = doubled
                .iter()
                .flat_map(|p| p.neighbors4())
                .filter(|p| !doubled.contains(p))
                .collect();

            let min_x = doubled.iter().map(|p| p.x).min().unwrap_or_default() - 1;
            let min_y = doubled.iter().map(|p| p.y).min().unwrap_or_default() - 1;
            let max_x = doubled.iter().map(|p| p.x).max().unwrap_or_default() + 1;
            let max_y = doubled.iter().map(|p| p.y).max().unwrap_or_default() + 1;

            dbg!(min_x, min_y, max_x, max_y);

            loop {
                let next: HashSet<_> = all_adjacent
                    .iter()
                    .flat_map(|p| p.neighbors4())
                    .filter(|p| {
                        !doubled.contains(p)
                            && p.x >= min_x
                            && p.x <= max_x
                            && p.y >= min_y
                            && p.y <= max_y
                    })
                    .collect();

                let pre_len = all_adjacent.len();
                all_adjacent.extend(next);

                if pre_len == all_adjacent.len() {
                    let mut sum = 0;

                    while !all_adjacent.is_empty() {
                        let group_seed = all_adjacent.iter().nu();
                        let mut group = HashSet::new();
                        group.insert(*group_seed);

                        loop {
                            let next_group: HashSet<Vector2> = group
                                .iter()
                                .flat_map(|p| p.neighbors4())
                                .filter(|p| all_adjacent.contains(p))
                                .collect();

                            let pre_len = group.len();
                            group.extend(next_group);

                            if group.len() == pre_len {
                                break;
                            }
                        }

                        let edge: HashSet<Vector2> = group
                            .iter()
                            .copied()
                            .filter(|p| p.neighbors4().any(|p2| !group.contains(&p2)))
                            .collect();

                        if edge
                            .iter()
                            .all(|p| p.neighbors4().any(|p2| doubled.contains(&p2)))
                        {
                            let adding: HashSet<_> = group
                                .iter()
                                .filter(|p| p.x % 2 == 0 && p.y % 2 == 0)
                                .collect();
                                sum += adding.len();
                        }

                        for p in group {
                            all_adjacent.remove(&p);
                        }
                    }

                    return sum as i64;
                }
            }
        }
    }

    unreachable!()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("..F7.
.FJ|.
SJ.L7
|F--J
LJ..."), 8);
    assert_eq!(part2("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"), 10);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 6875);
    assert_eq!(part2(input), 471);
}
