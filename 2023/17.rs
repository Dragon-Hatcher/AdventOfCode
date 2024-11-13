use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 17)
}

fn solve(input: &str, min: i64, max: i64) -> i64 {
    let grid = Grid::new_by_char(input, |c| c.to_digit(10).unwrap() as i64);
    let goal = Vector2::new(grid.width() - 1, grid.height() - 1);

    let mut distances = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((0, (Vector2::ZERO, Direction::Right)));
    queue.push((0, (Vector2::ZERO, Direction::Down)));

    while let Some((cost, (pos, dir))) = queue.pop() {
        if pos == goal {
            return -cost;
        }

        for next_dir in [dir.turn(Turn::Left), dir.turn(Turn::Right)] {
            let mut next_cost = -cost;
            for i in 1..=max {
                let next_pos = pos + next_dir.vector() * i;
                if !grid.in_bounds(next_pos) {
                    break;
                }
                next_cost += grid[next_pos];
                let next = (next_pos, next_dir.normalize());
                if min <= i && next_cost < *distances.get(&next).unwrap_or(&i64::MAX) {
                    distances.insert(next, next_cost);
                    queue.push((-next_cost, next));
                }
            }
        }
    }

    unreachable!();
}

fn part1(input: &str) -> i64 {
    solve(input, 1, 3)
}

fn part2(input: &str) -> i64 {
    solve(input, 4, 10)
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
    assert_eq!(part2(input), 94);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 674);
    assert_eq!(part2(input), 773);
}
