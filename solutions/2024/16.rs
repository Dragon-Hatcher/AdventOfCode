use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 16)
}

fn parse(input: &str) -> (Grid<bool>, Vec2, Vec2) {
    let grid = Grid::new_by_char(input, |c| c == '#');
    let start = Vec2::new(1, grid.height() - 2);
    let end = Vec2::new(grid.width() - 2, 1);

    (grid, start, end)
}

fn dijkstra(
    grid: &Grid<bool>,
    mut unvisited: MinBinaryHeap<(i64, Vec2, Direction)>,
) -> HashMap<(Vec2, Direction), i64> {
    let mut visited = HashMap::default();

    while let Some((dist, p, d)) = unvisited.pop() {
        if visited.contains_key(&(p, d)) || grid[p] {
            continue;
        }

        visited.insert((p, d), dist);

        unvisited.push((dist + 1, p + d.vector(), d));
        unvisited.push((dist + 1000, p, d.turn_left()));
        unvisited.push((dist + 1000, p, d.turn_right()));
    }

    visited
}

fn part1(input: &str) -> i64 {
    let (grid, start, end) = parse(input);

    let mut unvisited = MinBinaryHeap::new_min();
    unvisited.push((0, start, Direction::Right));
    let distances = dijkstra(&grid, unvisited);

    Direction::ALL
        .into_iter()
        .map(|d| distances[&(end, d)])
        .min()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let (grid, start, end) = parse(input);

    let mut unvisited = MinBinaryHeap::new_min();
    unvisited.push((0, start, Direction::Right));
    let distances_start = dijkstra(&grid, unvisited);

    let mut unvisited = MinBinaryHeap::new_min();
    unvisited.push((0, end, Direction::Up));
    unvisited.push((0, end, Direction::Right));
    unvisited.push((0, end, Direction::Down));
    unvisited.push((0, end, Direction::Left));
    let distances_end = dijkstra(&grid, unvisited);

    let best_dist = part1(input);

    grid.points()
        .filter(|&p| !grid[p])
        .filter(|&p| {
            Direction::ALL
                .into_iter()
                .any(|d| distances_start[&(p, d)] + distances_end[&(p, d.reverse())] <= best_dist)
        })
        .count() as i64
}

fn main() {
    advent::new(2024, 16, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    assert_eq!(part1(input), 7036);
    assert_eq!(part2(input), 45);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 99460);
    assert_eq!(part2(input), 500);
}
