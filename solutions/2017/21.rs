use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 21)
}

fn parse_rule(rule: &str) -> (Grid<bool>, Grid<bool>) {
    let (from, to) = rule.split_once(" => ").unwrap();
    (
        Grid::new_by_char(&from.replace('/', "\n"), |c| c == '#'),
        Grid::new_by_char(&to.replace('/', "\n"), |c| c == '#'),
    )
}

fn gen_all_rules(input: &str) -> Vec<(Grid<bool>, Grid<bool>)> {
    let mut rules = vec![];

    for (from, to) in input.lines().map(parse_rule) {
        let r1 = from;
        let r2 = r1.rotate_90();

        rules.push((r1.mirror_horizontal(), to.clone()));
        rules.push((r1.mirror_vertical(), to.clone()));
        rules.push((r1, to.clone()));

        rules.push((r2.mirror_horizontal(), to.clone()));
        rules.push((r2.mirror_vertical(), to.clone()));
        rules.push((r2, to.clone()));
    }

    rules
}

fn iterate(grid: Grid<bool>, rules: &[(Grid<bool>, Grid<bool>)]) -> Grid<bool> {
    let og_part_size = if grid.width() % 2 == 0 { 2 } else { 3 };
    let new_part_size = og_part_size + 1;

    let part_width = grid.width() / og_part_size;
    let new_size = part_width * new_part_size;
    let mut new_grid = Grid::new_homogenous(new_size, new_size, false);

    for p in Range::new_tl(Vec2::ZERO, part_width, part_width).points() {
        let og_p = p * og_part_size;
        let new_p = p * new_part_size;

        let part = grid.sub_grid(Range::new_tl(og_p, og_part_size, og_part_size));
        let (_, replace) = &rules.iter().find(|(f, _)| f == &part).unwrap();

        for r_p in replace.points() {
            new_grid[new_p + r_p] = replace[r_p];
        }
    }

    new_grid
}

const START: &str = ".#.
..#
###";

fn solve(input: &str, times: i64) -> i64 {
    let mut grid = Grid::new_by_char(START, |c| c == '#');
    let rules = gen_all_rules(input);

    for _ in 0..times {
        grid = iterate(grid, &rules);
    }

    grid.count_true()
}

fn part1(input: &str) -> i64 {
    solve(input, 5)
}

fn part2(input: &str) -> i64 {
    solve(input, 18)
}

fn main() {
    advent::new(2017, 21, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
    assert_eq!(solve(input, 2), 12);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 203);
    assert_eq!(part2(input), 3342470);
}
