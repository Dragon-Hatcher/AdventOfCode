use crate::standard_parsers::AocParsed;
use itertools::Itertools;
use std::collections::HashSet;

struct Hill {
    start: (i64, i64),
    end: (i64, i64),
    heights: Vec<Vec<u8>>,
}

impl Hill {
    fn in_bounds(&self, loc: (i64, i64)) -> bool {
        loc.0 >= 0
            && loc.1 >= 0
            && loc.0 < self.heights[0].len() as i64
            && loc.1 < self.heights.len() as i64
    }

    fn neighbors(&self, loc: (i64, i64)) -> impl Iterator<Item = (i64, i64)> + '_ {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(move |(dx, dy)| (loc.0 + dx, loc.1 + dy))
            .filter(|loc| self.in_bounds(*loc))
    }

    fn height(&self, loc: (i64, i64)) -> u8 {
        self.heights[loc.1 as usize][loc.0 as usize]
    }

    fn search(
        &self,
        start: (i64, i64),
        can_move: impl Fn(u8, u8) -> bool,
        end_condition: impl Fn((i64, i64)) -> bool,
    ) -> i64 {
        if end_condition(start) {
            return 0;
        }

        let mut dist = 0;
        let mut visited = HashSet::new();
        visited.insert(start);
        let mut frontier = visited.clone();

        loop {
            dist += 1;

            let mut new_frontier = HashSet::new();

            for p in frontier.iter() {
                let start_height = self.height(*p);

                for p in self.neighbors(*p) {
                    if can_move(self.height(p), start_height) && !visited.contains(&p) {
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

fn parse_hill(input: &str) -> Hill {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let heights: Vec<Vec<u8>> = input
        .non_empty()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        'S' => start = (x as i64, y as i64),
                        'E' => end = (x as i64, y as i64),
                        _ => {}
                    }

                    let num = match c {
                        'S' => 'a',
                        'E' => 'z',
                        _ => c,
                    } as usize
                        - 'a' as usize;
                    num as u8
                })
                .collect_vec()
        })
        .collect_vec();

    Hill {
        start,
        end,
        heights,
    }
}

///
/// --- Day 12: Hill Climbing Algorithm ---
///
/// You try contacting the Elves using your handheld device, but the river you're
/// following must be too low to get a decent signal.
///
/// You ask the device for a heightmap of the surrounding area (your puzzle input).
/// The heightmap shows the local area from above broken into a grid; the elevation
/// of each square of the grid is given by a single lowercase letter, where `a` is
/// the lowest elevation, `b` is the next-lowest, and so on up to the highest elevation,
/// `z`.
///
/// Also included on the heightmap are marks for your current position (`S`) and
/// the location that should get the best signal (`E`). Your current position (`S`)
/// has elevation `a`, and the location that should get the best signal (`E`) has
/// elevation `z`.
///
/// You'd like to reach `E`, but to save energy, you should do it in *as few steps
/// as possible*. During each step, you can move exactly one square up, down, left,
/// or right. To avoid needing to get out your climbing gear, the elevation of the
/// destination square can be *at most one higher* than the elevation of your current
/// square; that is, if your current elevation is `m`, you could step to elevation
/// `n`, but not to elevation `o`. (This also means that the elevation of the destination
/// square can be much lower than the elevation of your current square.)
///
/// For example:
///
/// ```
/// Sabqponm
/// abcryxxl
/// accszExk
/// acctuvwj
/// abdefghi
///
/// ```
///
/// Here, you start in the top-left corner; your goal is near the middle. You could
/// start by moving down or right, but eventually you'll need to head toward the
/// `e` at the bottom. From there, you can spiral around to the goal:
///
/// ```
/// v..v<<<<
/// >v.vv<<^
/// .>vv>E^^
/// ..v>>>^^
/// ..>>>>>^
///
/// ```
///
/// In the above diagram, the symbols indicate whether the path exits each square
/// moving up (`^`), down (`v`), left (`<`), or right (`>`). The location that should
/// get the best signal is still `E`, and `.` marks unvisited squares.
///
/// This path reaches the goal in `*31*` steps, the fewest possible.
///
/// *What is the fewest steps required to move from your current position to the
/// location that should get the best signal?*
///
pub fn part1(input: &str) -> i64 {
    let hill = parse_hill(input);
    hill.search(hill.start, |from, to| from <= to + 1, |p| hill.end == p)
}

///
/// --- Part Two ---
///
/// As you walk up the hill, you suspect that the Elves will want to turn this into
/// a hiking trail. The beginning isn't very scenic, though; perhaps you can find
/// a better starting point.
///
/// To maximize exercise while hiking, the trail should start as low as possible:
/// elevation `a`. The goal is still the square marked `E`. However, the trail should
/// still be direct, taking the fewest steps to reach its goal. So, you'll need to
/// find the shortest path from *any square at elevation `a`* to the square marked
/// `E`.
///
/// Again consider the example from above:
///
/// ```
/// Sabqponm
/// abcryxxl
/// accszExk
/// acctuvwj
/// abdefghi
///
/// ```
///
/// Now, there are six choices for starting position (five marked `a`, plus the square
/// marked `S` that counts as being at elevation `a`). If you start at the bottom-left
/// square, you can reach the goal most quickly:
///
/// ```
/// ...v<<<<
/// ...vv<<^
/// ...v>E^^
/// .>v>>>^^
/// >^>>>>>^
///
/// ```
///
/// This path reaches the goal in only `*29*` steps, the fewest possible.
///
/// *What is the fewest steps required to move starting from any square with elevation
/// `a` to the location that should get the best signal?*
///
pub fn part2(input: &str) -> i64 {
    let hill = parse_hill(input);
    hill.search(hill.end, |from, to| from >= to - 1, |p| hill.height(p) == 0)
}

const PART1_EX_ANSWER: &str = "31";
const PART1_ANSWER: &str = "350";
const PART2_EX_ANSWER: &str = "29";
const PART2_ANSWER: &str = "349";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
