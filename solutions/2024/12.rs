use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 12)
}

fn part1(input: &str) -> i64 {
    let mut visited = HashSet::default();
    let grid = Grid::new_by_char(input, |c| c);

    fn claim(
        p: Vec2,
        group: char,
        grid: &Grid<char>,
        visited: &mut HashSet<Vec2>,
        perimeter: &mut i64,
        area: &mut i64,
    ) {
        if visited.contains(&p) || grid[p] != group {
            return;
        }

        visited.insert(p);
        *area += 1;
        *perimeter += p
            .neighbors4()
            .filter(|n| grid.get(*n) != Some(&group))
            .count() as i64;

        for n in grid.neighbors4(p) {
            claim(n, group, grid, visited, perimeter, area);
        }
    }

    grid.points()
        .map(|p| {
            let mut area = 0;
            let mut perimeter = 0;
            claim(p, grid[p], &grid, &mut visited, &mut perimeter, &mut area);
            area * perimeter
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut visited = HashSet::default();
    let grid = Grid::new_by_char(input, |c| c);

    fn claim(
        p: Vec2,
        group: char,
        grid: &Grid<char>,
        visited: &mut HashSet<Vec2>,
        sides: &mut i64,
        area: &mut i64,
    ) {
        if visited.contains(&p) || grid[p] != group {
            return;
        }

        visited.insert(p);
        *area += 1;
        *sides += Direction::ALL
            .into_iter()
            // If the grid has a member of the same group in the direction we
            // are checking then this isn't an edge at all let alone a unique side.
            .filter(|d| grid.get(p + d.vector()) != Some(&group))
            // If this is an edge we want to count each edge only once. We check
            // if this is the left-most square on this edge. This is the case if
            // either there is no square in the same group to the left, or there
            // is such a square but it has another square above it so the edge
            // still ends here.
            .filter(|d| {
                grid.get(p + d.turn_left().vector()) != Some(&group)
                    || grid.get(p + d.vector() + d.turn_left().vector()) == Some(&group)
            })
            .count() as i64;

        visited.insert(p);

        for n in grid.neighbors4(p) {
            claim(n, group, grid, visited, sides, area);
        }
    }

    grid.points()
        .map(|p| {
            let mut area = 0;
            let mut perimeter = 0;
            claim(p, grid[p], &grid, &mut visited, &mut perimeter, &mut area);
            area * perimeter
        })
        .sum()
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
