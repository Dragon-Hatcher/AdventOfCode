use advent::prelude::*;

fn parse(str: &str) -> Grid<i64> {
    Grid::new_by_char(str, |c| c.to_digit(10).unwrap() as i64)
}

fn default_input() -> Grid<i64> {
    parse(include_input!(2022 / 08))
}

fn part1(heights: Grid<i64>) -> i64 {
    fn is_visible(p: Vector2, delta: Vector2, heights: &Grid<i64>) -> bool {
        let height = heights[p];
        let mut search = p + delta;
        while heights.in_bounds(search) {
            if heights[search] >= height {
                return false;
            }
            search += delta;
        }

        true
    }

    heights
        .points()
        .map(|p| {
            is_visible(p, Vector2::new(1, 0), &heights)
                || is_visible(p, Vector2::new(-1, 0), &heights)
                || is_visible(p, Vector2::new(0, 1), &heights)
                || is_visible(p, Vector2::new(0, -1), &heights)
        })
        .filter(|b| *b)
        .count() as i64
}

fn part2(heights: Grid<i64>) -> i64 {
    fn calc_dist(p: Vector2, delta: Vector2, heights: &Grid<i64>) -> i64 {
        let height = heights[p];
        let mut dist = 0;
        let mut search = p + delta;
        while heights.in_bounds(search) {
            dist += 1;
            if heights[search] >= height {
                break;
            }
            search += delta;
        }
        dist
    }

    heights
        .points()
        .map(|p| {
            calc_dist(p, Vector2::new(1, 0), &heights)
                * calc_dist(p, Vector2::new(-1, 0), &heights)
                * calc_dist(p, Vector2::new(0, 1), &heights)
                * calc_dist(p, Vector2::new(0, -1), &heights)
        })
        .max()
        .unwrap_or_default()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = parse("30373
25512
65332
33549
35390");
    assert_eq!(part1(input.clone()), 21);
    assert_eq!(part2(input), 8);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1672);
    assert_eq!(part2(input), 327180);
}
