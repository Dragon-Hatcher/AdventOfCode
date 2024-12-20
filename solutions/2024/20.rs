use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 20)
}

fn parse(input: &str) -> (Grid<bool>, Vec2, Vec2) {
    let grid = Grid::new_by_char(input, |c| c == '#');

    let cs = Grid::new_by_char(input, |c| c);
    let start = cs.points().find(|&p| cs[p] == 'S').unwrap();
    let end = cs.points().find(|&p| cs[p] == 'E').unwrap();

    (grid, start, end)
}

fn do_bfs(grid: &Grid<bool>, start: Vec2) -> HashMap<Vec2, i64> {
    bfs()
        .start(start)
        .no_goal()
        .next(|p| grid.neighbors4(*p).filter(|p| !grid[*p]))
        .find_all()
        .visited
}

fn solve(input: &str, cheat_time: i64) -> i64 {
    let (grid, start, end) = parse(input);

    let dist_from_start = do_bfs(&grid, start);
    let dist_from_end = do_bfs(&grid, end);

    let normal_time = dist_from_start[&end];
    let target_time = normal_time - 100;

    let search_area = cheat_time + 1;

    let mut cnt = 0;

    for p1 in grid.points() {
        if grid[p1] {
            continue;
        }

        for p2 in Range::new_tl(
            p1 - v2(search_area, search_area),
            search_area * 2,
            search_area * 2,
        )
        .points()
        {
            if grid.get(p2) != Some(&false) {
                continue;
            }

            let between = p1.manhattan_dist(p2);
            if between > cheat_time {
                continue;
            }

            if dist_from_start[&p1] + between + dist_from_end[&p2] <= target_time {
                cnt += 1;
            }
        }
    }

    cnt
}

fn part1(input: &str) -> i64 {
    solve(input, 2)
}

fn part2(input: &str) -> i64 {
    solve(input, 20)
}

fn main() {
    advent::new(2024, 20, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1438);
    assert_eq!(part2(input), 1026446);
}
