use advent::prelude::*;
use std::convert::identity;

fn parse_hill(input: &str) -> Hill {
    let mut start = Vector2::new(0, 0);
    let mut end = Vector2::new(0, 0);
    let mut chars: Grid<char> = Grid::new_by_char(input, identity);
    chars.points().for_each(|p| match chars[p] {
        'S' => {
            start = p;
            chars[p] = 'a';
        }
        'E' => {
            end = p;
            chars[p] = 'z';
        }
        _ => {}
    });
    let heights = chars.map(|c| (*c as i64 - 'a' as i64) as u8);

    Hill {
        start,
        end,
        heights,
    }
}

fn default_input() -> Hill {
    parse_hill(include_input!(2022 / 12))
}

#[derive(Clone)]
struct Hill {
    start: Vector2,
    end: Vector2,
    heights: Grid<u8>,
}

impl Hill {
    fn search(
        &self,
        start: Vector2,
        can_move: impl Fn(u8, u8) -> bool,
        end_condition: impl Fn(Vector2) -> bool,
    ) -> i64 {
        if end_condition(start) {
            return 0;
        }

        let mut dist = 0;
        let mut visited = FxHashSet::default();
        visited.insert(start);
        let mut frontier = visited.clone();

        loop {
            dist += 1;

            let mut new_frontier = FxHashSet::default();

            for p in frontier.iter() {
                let start_height = self.heights[*p];

                for p in self.heights.neighbors4(*p) {
                    if can_move(self.heights[p], start_height) && !visited.contains(&p) {
                        if end_condition(p) {
                            return dist;
                        }
                        new_frontier.insert(p);
                    }
                }
            }

            visited.extend(new_frontier.iter());
            frontier.clear();
            frontier.extend(new_frontier);
        }
    }
}

fn part1(hill: Hill) -> i64 {
    hill.search(hill.start, |from, to| from <= to + 1, |p| hill.end == p)
}

fn part2(hill: Hill) -> i64 {
    hill.search(
        hill.end,
        |from, to| from >= to - 1,
        |p| hill.heights[p] == 0,
    )
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = parse_hill("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi");
    assert_eq!(part1(input.clone()), 31);
    assert_eq!(part2(input), 29);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 350);
    assert_eq!(part2(input), 349);
}
