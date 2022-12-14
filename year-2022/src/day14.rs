use crate::{grid::Point, standard_parsers::AocParsed};
use itertools::Itertools;
use rustc_hash::FxHashSet;

struct SandHeap {
    occupied: FxHashSet<Point>,
    floor: bool,
    max_y: i64,
    locations: Vec<Point>,
}

impl SandHeap {
    const SAND_START: Point = Point::new(500, 0);

    fn new(occupied: FxHashSet<Point>) -> SandHeap {
        let max_y = occupied.iter().map(|p| p.y).max().unwrap_or_default();
        SandHeap {
            occupied,
            floor: false,
            max_y: max_y + 2,
            locations: vec![Self::SAND_START],
        }
    }

    fn new_with_floor(occupied: FxHashSet<Point>) -> SandHeap {
        let max_y = occupied.iter().map(|p| p.y).max().unwrap_or_default();
        SandHeap {
            occupied,
            floor: true,
            max_y: max_y + 2,
            locations: vec![Self::SAND_START],
        }
    }

    fn can_hold_sand(&self, p: Point) -> bool {
        !self.occupied.contains(&p) && (!self.floor || p.y != self.max_y)
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand_p = *self.locations.last().unwrap_or(&Self::SAND_START);

        loop {
            if sand_p.y > self.max_y + 2 {
                break false;
            }

            let down = Point::new(sand_p.x, sand_p.y + 1);
            let left = Point::new(sand_p.x - 1, sand_p.y + 1);
            let right = Point::new(sand_p.x + 1, sand_p.y + 1);

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

fn connect_line(occupied: &mut FxHashSet<Point>, a: Point, b: Point) {
    let from_x = a.x.min(b.x);
    let to_x = a.x.max(b.x);
    let from_y = a.y.min(b.y);
    let to_y = a.y.max(b.y);

    for x in from_x..=to_x {
        for y in from_y..=to_y {
            occupied.insert(Point::new(x, y));
        }
    }
}

fn parse(input: &str, floor: bool) -> SandHeap {
    let mut occupied = FxHashSet::default();

    input.non_empty().for_each(|l| {
        l.nums_pos()
            .tuples()
            .into_iter()
            .map(|(x, y)| Point::new(x, y))
            .tuple_windows()
            .for_each(|(a, b)| connect_line(&mut occupied, a, b));
    });

    if floor {
        SandHeap::new_with_floor(occupied)
    } else {
        SandHeap::new(occupied)
    }
}

///
/// --- Day 14: Regolith Reservoir ---
///
/// The distress signal leads you to a giant waterfall! Actually, hang on - the signal
/// seems like it's coming from the waterfall itself, and that doesn't make any sense.
/// However, you do notice a little path that leads *behind* the waterfall.
///
/// Correction: the distress signal leads you behind a giant waterfall! There seems
/// to be a large cave system here, and the signal definitely leads further inside.
///
/// As you begin to make your way deeper underground, you feel the ground rumble
/// for a moment. Sand begins pouring into the cave! If you don't quickly figure
/// out where the sand is going, you could quickly become trapped!
///
/// Fortunately, your [familiarity](/2018/day/17) with analyzing the path of falling
/// material will come in handy here. You scan a two-dimensional vertical slice of
/// the cave above you (your puzzle input) and discover that it is mostly *air* with
/// structures made of *rock*.
///
/// Your scan traces the path of each solid rock structure and reports the `x,y`
/// coordinates that form the shape of the path, where `x` represents distance to
/// the right and `y` represents distance down. Each path appears as a single line
/// of text in your scan. After the first point of each path, each point indicates
/// the end of a straight horizontal or vertical line to be drawn from the previous
/// point. For example:
///
/// ```
/// 498,4 -> 498,6 -> 496,6
/// 503,4 -> 502,4 -> 502,9 -> 494,9
///
/// ```
///
/// This scan means that there are two paths of rock; the first path consists of
/// two straight lines, and the second path consists of three straight lines. (Specifically,
/// the first path consists of a line of rock from `498,4` through `498,6` and another
/// line of rock from `498,6` through `496,6`.)
///
/// The sand is pouring into the cave from point `500,0`.
///
/// Drawing rock as `#`, air as `.`, and the source of the sand as `+`, this becomes:
///
/// ```
///
///   4     5  5
///   9     0  0
///   4     0  3
/// 0 ......+...
/// 1 ..........
/// 2 ..........
/// 3 ..........
/// 4 ....#...##
/// 5 ....#...#.
/// 6 ..###...#.
/// 7 ........#.
/// 8 ........#.
/// 9 #########.
///
/// ```
///
/// Sand is produced *one unit at a time*, and the next unit of sand is not produced
/// until the previous unit of sand *comes to rest*. A unit of sand is large enough
/// to fill one tile of air in your scan.
///
/// A unit of sand always falls *down one step* if possible. If the tile immediately
/// below is blocked (by rock or sand), the unit of sand attempts to instead move
/// diagonally *one step down and to the left*. If that tile is blocked, the unit
/// of sand attempts to instead move diagonally *one step down and to the right*.
/// Sand keeps moving as long as it is able to do so, at each step trying to move
/// down, then down-left, then down-right. If all three possible destinations are
/// blocked, the unit of sand *comes to rest* and no longer moves, at which point
/// the next unit of sand is created back at the source.
///
/// So, drawing sand that has come to rest as `o`, the first unit of sand simply
/// falls straight down and then stops:
///
/// ```
/// ......+...
/// ..........
/// ..........
/// ..........
/// ....#...##
/// ....#...#.
/// ..###...#.
/// ........#.
/// ......o.#.
/// #########.
///
/// ```
///
/// The second unit of sand then falls straight down, lands on the first one, and
/// then comes to rest to its left:
///
/// ```
/// ......+...
/// ..........
/// ..........
/// ..........
/// ....#...##
/// ....#...#.
/// ..###...#.
/// ........#.
/// .....oo.#.
/// #########.
///
/// ```
///
/// After a total of five units of sand have come to rest, they form this pattern:
///
/// ```
/// ......+...
/// ..........
/// ..........
/// ..........
/// ....#...##
/// ....#...#.
/// ..###...#.
/// ......o.#.
/// ....oooo#.
/// #########.
///
/// ```
///
/// After a total of 22 units of sand:
///
/// ```
/// ......+...
/// ..........
/// ......o...
/// .....ooo..
/// ....#ooo##
/// ....#ooo#.
/// ..###ooo#.
/// ....oooo#.
/// ...ooooo#.
/// #########.
///
/// ```
///
/// Finally, only two more units of sand can possibly come to rest:
///
/// ```
/// ......+...
/// ..........
/// ......o...
/// .....ooo..
/// ....#ooo##
/// ...o#ooo#.
/// ..###ooo#.
/// ....oooo#.
/// .o.ooooo#.
/// #########.
///
/// ```
///
/// Once all `*24*` units of sand shown above have come to rest, all further sand
/// flows out the bottom, falling into the endless void. Just for fun, the path any
/// new sand takes before falling forever is shown here with `~`:
///
/// ```
/// .......+...
/// .......~...
/// ......~o...
/// .....~ooo..
/// ....~#ooo##
/// ...~o#ooo#.
/// ..~###ooo#.
/// ..~..oooo#.
/// .~o.ooooo#.
/// ~#########.
/// ~..........
/// ~..........
/// ~..........
///
/// ```
///
/// Using your scan, simulate the falling sand. *How many units of sand come to rest
/// before sand starts flowing into the abyss below?*
///
pub fn part1(input: &str) -> i64 {
    parse(input, false).fill()
}

///
/// --- Part Two ---
///
/// You realize you misread the scan. There isn't an endless void at the bottom of
/// the scan - there's floor, and you're standing on it!
///
/// You don't have time to scan the floor, so assume the floor is an infinite horizontal
/// line with a `y` coordinate equal to *two plus the highest `y` coordinate* of
/// any point in your scan.
///
/// In the example above, the highest `y` coordinate of any point is `9`, and so
/// the floor is at `y=11`. (This is as if your scan contained one extra rock path
/// like `-infinity,11 -> infinity,11`.) With the added floor, the example above
/// now looks like this:
///
/// ```
///         ...........+........
///         ....................
///         ....................
///         ....................
///         .........#...##.....
///         .........#...#......
///         .......###...#......
///         .............#......
///         .............#......
///         .....#########......
///         ....................
/// <-- etc #################### etc -->
///
/// ```
///
/// To find somewhere safe to stand, you'll need to simulate falling sand until a
/// unit of sand comes to rest at `500,0`, blocking the source entirely and stopping
/// the flow of sand into the cave. In the example above, the situation finally looks
/// like this after `*93*` units of sand come to rest:
///
/// ```
/// ............o............
/// ...........ooo...........
/// ..........ooooo..........
/// .........ooooooo.........
/// ........oo#ooo##o........
/// .......ooo#ooo#ooo.......
/// ......oo###ooo#oooo......
/// .....oooo.oooo#ooooo.....
/// ....oooooooooo#oooooo....
/// ...ooo#########ooooooo...
/// ..ooooo.......ooooooooo..
/// #########################
///
/// ```
///
/// Using your scan, simulate the falling sand until the source of the sand becomes
/// blocked. *How many units of sand come to rest?*
///
pub fn part2(input: &str) -> i64 {
    parse(input, true).fill()
}

const PART1_EX_ANSWER: &str = "24";
const PART1_ANSWER: &str = "715";
const PART2_EX_ANSWER: &str = "93";
const PART2_ANSWER: &str = "25248";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
