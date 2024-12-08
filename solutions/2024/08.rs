use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 08)
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);
    let loc_pairs = grid.points().map(|p| (p, grid[p])).filter(|(_, c)| *c != '.').collect_vec();
    let mut locs: HashMap<char, HashSet<Vec2>> = HashMap::default();

    for (p, c) in loc_pairs {
        locs.entry(c).or_default().insert(p);
    }

    let mut sum = 0;
    'outer: for p in grid.points() {
        for (_, ps) in locs.iter() {
            for (p1, p2) in ps.iter().tuple_combinations() {
                let diff = p2 - p1;
                if p == p2 + diff || p == p1 - diff {
                    sum += 1;
                    continue 'outer;
                }
            }
        }

    }   
    sum
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);
    let loc_pairs = grid.points().map(|p| (p, grid[p])).filter(|(_, c)| *c != '.').collect_vec();
    let mut locs: HashMap<char, HashSet<Vec2>> = HashMap::default();

    for (p, c) in loc_pairs {
        locs.entry(c).or_default().insert(p);
    }

    let mut sum = 0;
    'outer: for p in grid.points() {
        for (_, ps) in locs.iter() {
            for (p1, p2) in ps.iter().tuple_combinations() {
                let diff = p2 - p1;
                let diff_i = p - p1;
                if diff.is_scaling(diff_i) || diff_i.is_scaling(diff) {
                    sum += 1;
                    continue 'outer;
                }
            }
        }

    }   
    sum
}

fn main() {
    advent::new(2024, 8, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(part1(input), 398);
    assert_eq!(part2(input), 1333);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 398);
    assert_eq!(part2(input), 1333);
}
