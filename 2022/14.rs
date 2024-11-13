use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 14)
}

struct SandHeap {
    occupied: FxHashSet<Vector2>,
    floor: bool,
    max_y: i64,
    locations: Vec<Vector2>,
}

impl SandHeap {
    const SAND_START: Vector2 = Vector2::new(500, 0);

    fn new(occupied: FxHashSet<Vector2>) -> SandHeap {
        let max_y = occupied.iter().map(|p| p.y).max().unwrap_or_default();
        SandHeap {
            occupied,
            floor: false,
            max_y: max_y + 2,
            locations: vec![Self::SAND_START],
        }
    }

    fn new_with_floor(occupied: FxHashSet<Vector2>) -> SandHeap {
        let max_y = occupied.iter().map(|p| p.y).max().unwrap_or_default();
        SandHeap {
            occupied,
            floor: true,
            max_y: max_y + 2,
            locations: vec![Self::SAND_START],
        }
    }

    fn can_hold_sand(&self, p: Vector2) -> bool {
        !self.occupied.contains(&p) && (!self.floor || p.y != self.max_y)
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand_p = *self.locations.last().unwrap_or(&Self::SAND_START);

        loop {
            if sand_p.y > self.max_y + 2 {
                break false;
            }

            let down = Vector2::new(sand_p.x, sand_p.y + 1);
            let left = Vector2::new(sand_p.x - 1, sand_p.y + 1);
            let right = Vector2::new(sand_p.x + 1, sand_p.y + 1);

            if self.can_hold_sand(down) {
                sand_p = down;
            } else if self.can_hold_sand(left) {
                sand_p = left;
            } else if self.can_hold_sand(right) {
                sand_p = right
            } else if self.can_hold_sand(sand_p) {
                self.locations.pop();
                self.occupied.insert(sand_p);
                break true;
            } else {
                break false;
            }

            self.locations.push(sand_p);
        }
    }

    fn fill(&mut self) -> i64 {
        let mut dropped = 0;
        loop {
            if self.drop_sand() {
                dropped += 1;
            } else {
                break dropped;
            }
        }
    }
}

fn connect_line(occupied: &mut FxHashSet<Vector2>, a: Vector2, b: Vector2) {
    let from_x = a.x.min(b.x);
    let to_x = a.x.max(b.x);
    let from_y = a.y.min(b.y);
    let to_y = a.y.max(b.y);

    for x in from_x..=to_x {
        for y in from_y..=to_y {
            occupied.insert(Vector2::new(x, y));
        }
    }
}

fn parse(input: &str, floor: bool) -> SandHeap {
    let mut occupied = FxHashSet::default();

    input.non_empty().for_each(|l| {
        l.nums_pos()
            .tuples()
            .into_iter()
            .map(|(x, y)| Vector2::new(x, y))
            .tuple_windows()
            .for_each(|(a, b)| connect_line(&mut occupied, a, b));
    });

    if floor {
        SandHeap::new_with_floor(occupied)
    } else {
        SandHeap::new(occupied)
    }
}

fn part1(input: &str) -> i64 {
    parse(input, false).fill()
}

fn part2(input: &str) -> i64 {
    parse(input, true).fill()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(part1(input), 24);
    assert_eq!(part2(input), 93);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 715);
    assert_eq!(part2(input), 25248);
}
