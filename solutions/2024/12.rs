use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 12)
}

fn part1(input: &str) -> i64 {
    let mut visited = HashSet::default();
    let grid = Grid::new_by_char(input, |c| c);

    fn claim(
        p: Vec2,
        grid: &Grid<char>,
        visited: &mut HashSet<Vec2>,
        perim: &mut i64,
        area: &mut i64,
    ) {
        *area += 1;
        for n in p.neighbors4() {
            if !(grid.in_bounds(n) && grid[n] == grid[p]) {
                *perim += 1;
            }
        }
        visited.insert(p);

        for n in grid.neighbors4(p) {
            if !visited.contains(&n) && grid[n] == grid[p] {
                claim(n, grid, visited, perim, area);
            }
        }
    }

    let mut sum = 0;
    for p in grid.points() {
        if visited.contains(&p) {
            continue;
        }

        let mut area = 0;
        let mut perim = 0;
        claim(p, &grid, &mut visited, &mut perim, &mut area);
        sum += area * perim;
    }

    sum
}

fn part2(input: &str) -> i64 {
    let mut visited = HashSet::default();
    let grid = Grid::new_by_char(input, |c| c);

    fn claim(
        p: Vec2,
        grid: &Grid<char>,
        visited: &mut HashSet<Vec2>,
        perim: &mut i64,
        area: &mut i64,
    ) {
        *area += 1;

        for d in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            if grid.get(p + d.vector()) != Some(&grid[p])
                && (grid.get(p + d.turn(Turn::Left).vector()) != Some(&grid[p])
                    || grid.get(p + d.vector() + d.turn(Turn::Left).vector()) == Some(&grid[p]))
            {
                *perim += 1;
            }
        }

        visited.insert(p);

        for n in grid.neighbors4(p) {
            if !visited.contains(&n) && grid[n] == grid[p] {
                claim(n, grid, visited, perim, area);
            }
        }
    }

    let mut sum = 0;
    for p in grid.points() {
        if visited.contains(&p) {
            continue;
        }

        let mut area = 0;
        let mut perim = 0;
        claim(p, &grid, &mut visited, &mut perim, &mut area);
        sum += area * perim;
    }

    sum
}

fn main() {
    advent::new(2024, 12, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    assert_eq!(part1(input), 1930);
    assert_eq!(part2(input), 1206);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1400386);
    assert_eq!(part2(input), 851994);
}
