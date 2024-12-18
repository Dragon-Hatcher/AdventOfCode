use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!("width=71, take=1024\n\n" / 2024 / 18)
}

fn parse(input: &str) -> (Vec<Vec2>, i64, i64) {
    let (config, bytes) = input.sections().tup();
    let (width, take) = config.nums().tup();
    let bytes = bytes.lines().map(|l| {
        let (x, y) = l.nums().tup();
        v2(x, y)
    }).collect_vec();

    (bytes, width, take)
}

fn find_path(grid: &Grid<bool>) -> Option<BFSResult<Vec2>> {
    bfs()
        .start(Vec2::ZERO)
        .goal(Vec2::new(grid.width() - 1, grid.height() - 1))
        .next(|&p| grid.neighbors4(p).filter(|&p| !grid[p]))
        .try_shortest()
        .ok()
}

fn part1(input: &str) -> i64 {
    let (bytes, width, take) = parse(input);

    let mut grid = Grid::new_homogenous(width, width, false);
    for p in bytes.into_iter().take(take as usize) {
        grid[p] = true;
    }

    find_path(&grid).unwrap().steps
}

fn part2(input: &str) -> Vec2 {
    let (bytes, width, _) = parse(input);

    let mut grid = Grid::new_homogenous(width, width, false);
    let mut last_path = find_path(&grid).unwrap();
    for p in bytes {
        grid[p] = true;

        // This didn't disturb our last path so no need to check again.
        if !last_path.visited.contains_key(&p) {
            continue;
        }

        if let Some(path) = find_path(&grid) {
            last_path = path;
        } else {
            return p;
        }
    }

    unreachable!()
}

fn main() {
    advent::new(2024, 18, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "width=7, take=12

5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    assert_eq!(part1(input), 22);
    assert_eq!(part2(input), Vec2::new(6, 1));
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 282);
    assert_eq!(part2(input), Vec2::new(64, 29));
}
