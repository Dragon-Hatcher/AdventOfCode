use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2025 / 04)
}

fn parse(input: &str) -> (Grid<bool>, Grid<usize>) {
    let has_roll = Grid::new_by_char(input, |c| c == '@');
    let neighbors = Grid::new_with(has_roll.width(), has_roll.height(), |p| {
        has_roll.neighbors8(p).filter(|&n| has_roll[n]).count()
    });
    (has_roll, neighbors)
}

fn part1(input: &str) -> i64 {
    let (has_roll, neighbors) = parse(input);
    has_roll
        .points()
        .filter(|&p| has_roll[p] && neighbors[p] < 4)
        .count() as i64
}

fn part2(input: &str) -> i64 {
    let (mut has_roll, mut neighbors) = parse(input);

    let mut removed = 0;
    let mut stack: Vec<Vec2> = has_roll
        .points()
        .filter(|&p| has_roll[p] && neighbors[p] < 4)
        .collect();

    while let Some(next) = stack.pop() {
        if !has_roll[next] {
            continue;
        }

        removed += 1;
        has_roll[next] = false;
        for neighbor in has_roll.neighbors8(next) {
            neighbors[neighbor] -= 1;
            if neighbors[neighbor] < 4 {
                stack.push(neighbor);
            }
        }
    }

    removed
}

fn main() {
    advent::new(2025, 4, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 43);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1460);
    assert_eq!(part2(input), 9243);
}
