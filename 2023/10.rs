use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 10)
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Cell {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl Cell {
    fn from_char(c: char) -> Option<Cell> {
        match c {
            '|' => Some(Cell {
                north: true,
                south: true,
                ..Default::default()
            }),
            '-' => Some(Cell {
                east: true,
                west: true,
                ..Default::default()
            }),
            'L' => Some(Cell {
                north: true,
                east: true,
                ..Default::default()
            }),
            'J' => Some(Cell {
                north: true,
                west: true,
                ..Default::default()
            }),
            '7' => Some(Cell {
                south: true,
                west: true,
                ..Default::default()
            }),
            'F' => Some(Cell {
                south: true,
                east: true,
                ..Default::default()
            }),
            '.' => Some(Default::default()),
            _ => None,
        }
    }

    fn from_neighbors(
        north: Option<Cell>,
        south: Option<Cell>,
        east: Option<Cell>,
        west: Option<Cell>,
    ) -> Cell {
        Cell {
            north: north.map(|c| c.south).unwrap_or(false),
            south: south.map(|c| c.north).unwrap_or(false),
            east: east.map(|c| c.west).unwrap_or(false),
            west: west.map(|c| c.east).unwrap_or(false),
        }
    }

    fn is_empty(&self) -> bool {
        !self.north && !self.south && !self.east && !self.west
    }
}

fn parse(input: &str) -> (Grid<Cell>, Vector2) {
    let mut grid = Grid::new_by_char(input, Cell::from_char);
    let start = grid.points().find(|p| grid[*p].is_none()).unwrap();
    let start_cell = Cell::from_neighbors(
        grid.get(start - Vector2::E2).map(|c| c.unwrap()),
        grid.get(start + Vector2::E2).map(|c| c.unwrap()),
        grid.get(start + Vector2::E1).map(|c| c.unwrap()),
        grid.get(start - Vector2::E1).map(|c| c.unwrap()),
    );
    grid[start] = Some(start_cell);
    let grid = grid.map(|c| c.unwrap());

    (grid, start)
}

fn walk(grid: &Grid<Cell>, start: Vector2) -> HashSet<Vector2> {
    let mut edge = HashSet::new();
    let mut seen = HashSet::new();

    edge.insert(start);
    seen.insert(start);

    while !edge.is_empty() {
        let mut new_edge = HashSet::new();

        for p in edge {
            let cell = grid[p];
            if cell.north {
                new_edge.insert(p - Vector2::E2);
            }
            if cell.south {
                new_edge.insert(p + Vector2::E2);
            }
            if cell.east {
                new_edge.insert(p + Vector2::E1);
            }
            if cell.west {
                new_edge.insert(p - Vector2::E1);
            }
        }

        edge = new_edge;
        edge.retain(|p| !seen.contains(p));
        seen.extend(&edge);
    }

    seen
}

fn part1(input: &str) -> i64 {
    let (grid, start) = parse(input);
    walk(&grid, start).len() as i64 / 2
}

fn part2(input: &str) -> i64 {
    let (mut grid, start) = parse(input);
    let walk = walk(&grid, start);

    for p in grid.points().filter(|p| !walk.contains(p)) {
        grid[p] = Cell::from_char('.').unwrap();
    }

    let mut found = 0;

    for y in 0..grid.height() {
        let mut inside = false;

        let mut north = false;
        let mut south = false;

        for x in 0..grid.width() {
            let cell = grid[Vector2::new(x, y)];

            if cell.is_empty() && inside {
                found += 1;
            }

            if cell.north { north = !north; }
            if cell.south {
                south = !south;
            }

            if north && south {
                north = false;
                south = false;
                inside = !inside;
            }
        }
    }

    found
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(
        part1(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
        ),
        8
    );
    assert_eq!(
        part2(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
        ),
        10
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 6875);
    assert_eq!(part2(input), 471);
}
