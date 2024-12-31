use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 25)
}

fn part1(input: &str) -> i64 {
    let mut locks = vec![];
    let mut keys = vec![];

    let mut height = 0;
    for thing in input.sections() {
        let grid = Grid::new_by_char(thing, |c| c == '#');
        height = grid.height();

        let heights = grid
            .cols()
            .map(|col| col.points().filter(|&p| grid[p]).count() as i64 - 1)
            .collect_vec();

        if grid.row_range(0).points().all(|p| grid[p]) {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut sum = 0;
    for (lock, key) in locks.into_iter().cartesian_product(keys.into_iter()) {
        let tot = lock
            .into_iter()
            .zip(key.into_iter())
            .map(|(a, b)| a + b)
            .all(|h| h < height - 1);

        if tot {
            sum += 1;
        }
    }

    sum
}

fn main() {
    advent::new(2024, 25, default_input).part1(part1).cli();
}

#[test]
fn example() {
    let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    assert_eq!(part1(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 3344);
}
