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

        if grid.row_range(0).points().all(|p| grid[p]) {
            let mut heights = vec![];
            for x in 0..grid.width() {
                let height = grid.col_range(x).points().find(|p| !grid[*p]).unwrap();
                heights.push(height.y - 1);
            }
            println!("a {heights:?}");
            locks.push(heights);
        } else {
            let mut heights = vec![];
            for x in 0..grid.width() {
                let height = grid.height() - 1 - grid.col_range(x).points().find(|p| grid[*p]).unwrap().y;
                heights.push(height);
            }
            println!("b {heights:?}");
            keys.push(heights);
        }

    }

    dbg!(height);
    let mut sum = 0;
    for (lock, key) in locks.into_iter().cartesian_product(keys.into_iter()) {
        let tot = lock.into_iter().zip(key.into_iter()).map(|(a, b)| a + b).all(|h| h < height - 1);

        if tot {
            sum += 1;
        }


    }

    // dbg!(locks, keys);

    sum
}

fn main() {
    advent::new(2024, 25, default_input)
        .part1(part1)
        .cli();
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
    // assert_eq!(part1(input), 0);
}
