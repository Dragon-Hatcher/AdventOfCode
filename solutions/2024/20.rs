use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 20)
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c == '#');
    let cs = Grid::new_by_char(input, |c| c);
    let start = cs.points().find(|p| cs[*p] == 'S').unwrap();
    let end = cs.points().find(|p| cs[*p] == 'E').unwrap();

    let time = bfs()
        .start(start)
        .goal(end)
        .next(|p| grid.neighbors4(*p).filter(|p| !grid[*p]))
        .shortest()
        .steps;

    fn solve(
        grid: &Grid<bool>,
        at: Vec2,
        cheat: Option<(Vec2, Vec2)>,
        goal: Vec2,
        steps_saved: i64,
        visited: &mut HashSet<Vec2>,
        found_cheats: &mut HashSet<(Vec2, Vec2)>, // memo: &mut HashMap<(Vec2, bool), i64>,
    ) {
        if at == goal {
            if steps_saved > 0 {
                dbg!(steps_saved);
            }
            if steps_saved >= 100 && cheat.is_some() {
                found_cheats.insert(cheat.unwrap());
            }
            return;
            // return if steps_saved >= 100 { 1 } else { 0 };
        }

        // if let Some(cnt) = memo.get(&(at, cheat_left)) {
        //     return *cnt;
        // }

        let mut sum = 0;

        for n in grid.neighbors4(at) {
            if visited.contains(&n) {
                continue;
            }

            visited.insert(n);
            if !grid[n] {
                solve(grid, n, cheat, goal, steps_saved - 1, visited, found_cheats);
            } else if cheat.is_none() {
                let cheat = Some((at, n));
                solve(grid, n, cheat, goal, steps_saved - 1, visited, found_cheats);
            }
            visited.remove(&n);
        }

        // memo.insert((at, cheat_left), sum);
    }

    dbg!(time);

    let mut found = HashSet::default();
    solve(
        &grid,
        start,
        None,
        end,
        time,
        &mut HashSet::default(),
        &mut found, // &mut HashMap::default(),
    );

    found.len() as i64
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c == '#');
    let cs = Grid::new_by_char(input, |c| c);
    let start = cs.points().find(|p| cs[*p] == 'S').unwrap();
    let end = cs.points().find(|p| cs[*p] == 'E').unwrap();

    let time = bfs()
        .start(start)
        .goal(end)
        .next(|p| grid.neighbors4(*p).filter(|p| !grid[*p]))
        .shortest()
        .steps;

    let dist_from_start = bfs()
        .start(start)
        .no_goal()
        .next(|p| grid.neighbors4(*p).filter(|p| !grid[*p]))
        .find_all()
        .visited;

    let dist_from_end = bfs()
        .start(end)
        .no_goal()
        .next(|p| grid.neighbors4(*p).filter(|p| !grid[*p]))
        .find_all()
        .visited;

    let mut cnt = 0;
    let points = grid.points().collect_vec();
    for (p1, p2) in points.into_iter().tuple_combinations() {
        if grid[p1] { continue; }
        if grid[p2] { continue; }

        let between = p1.manhattan_dist(p2);
        {
            let from_start = dist_from_start[&p1];
            let to_end = dist_from_end[&p2];
            if between <= 20 && from_start + between + to_end + 100 <= time {
                cnt += 1;
                continue;
            }
        }

        {
            let from_start = dist_from_start[&p2];
            let to_end = dist_from_end[&p1];
            if between <= 20 && from_start + between + to_end + 100 <= time {
                cnt += 1;
                continue;
            }
        }
    }

    cnt
}

fn main() {
    advent::new(2024, 20, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    assert_eq!(part1(input), 1);
    // assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
