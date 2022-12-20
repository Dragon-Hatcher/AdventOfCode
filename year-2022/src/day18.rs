use crate::standard_parsers::{AocParsed, IntoTup};
use rustc_hash::FxHashSet;

const DELTAS: [(i64, i64, i64); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

const DELTAS_FULL: [(i64, i64, i64); 26] = [
    (-1, -1, -1),
    (0, -1, -1),
    (1, -1, -1),
    (-1, 0, -1),
    (0, 0, -1),
    (1, 0, -1),
    (-1, 1, -1),
    (0, 1, -1),
    (1, 1, -1),
    (-1, -1, 0),
    (0, -1, 0),
    (1, -1, 0),
    (-1, 0, 0),
    (1, 0, 0),
    (-1, 1, 0),
    (0, 1, 0),
    (1, 1, 0),
    (-1, -1, 1),
    (0, -1, 1),
    (1, -1, 1),
    (-1, 0, 1),
    (0, 0, 1),
    (1, 0, 1),
    (-1, 1, 1),
    (0, 1, 1),
    (1, 1, 1),
];

///
/// --- Day 18: Boiling Boulders ---
///
/// You and the elephants finally reach fresh air. You've emerged near the base of
/// a large volcano that seems to be actively erupting! Fortunately, the lava seems
/// to be flowing away from you and toward the ocean.
///
/// Bits of lava are still being ejected toward you, so you're sheltering in the
/// cavern exit a little longer. Outside the cave, you can see the lava landing in
/// a pond and hear it loudly hissing as it solidifies.
///
/// Depending on the specific compounds in the lava and speed at which it cools,
/// it might be forming [obsidian](https://en.wikipedia.org/wiki/Obsidian)! The cooling
/// rate should be based on the surface area of the lava droplets, so you take a
/// quick scan of a droplet as it flies past you (your puzzle input).
///
/// Because of how quickly the lava is moving, the scan isn't very good; its resolution
/// is quite low and, as a result, it approximates the shape of the lava droplet
/// with *1x1x1 cubes on a 3D grid*, each given as its `x,y,z` position.
///
/// To approximate the surface area, count the number of sides of each cube that
/// are not immediately connected to another cube. So, if your scan were only two
/// adjacent cubes like `1,1,1` and `2,1,1`, each cube would have a single side covered
/// and five sides exposed, a total surface area of `*10*` sides.
///
/// Here's a larger example:
///
/// ```
/// 2,2,2
/// 1,2,2
/// 3,2,2
/// 2,1,2
/// 2,3,2
/// 2,2,1
/// 2,2,3
/// 2,2,4
/// 2,2,6
/// 1,2,5
/// 3,2,5
/// 2,1,5
/// 2,3,5
///
/// ```
///
/// In the above example, after counting up all the sides that aren't connected to
/// another cube, the total surface area is `*64*`.
///
/// *What is the surface area of your scanned lava droplet?*
///
pub fn part1(input: &str) -> i64 {
    let points: FxHashSet<(i64, i64, i64)> = input.non_empty().map(|l| l.nums().tup()).collect();

    points
        .iter()
        .flat_map(|p| DELTAS.map(|d| (p.0 + d.0, p.1 + d.1, p.2 + d.2)))
        .filter(|p| !points.contains(p))
        .count() as i64
}

///
/// --- Part Two ---
///
/// Something seems off about your calculation. The cooling rate depends on exterior
/// surface area, but your calculation also included the surface area of air pockets
/// trapped in the lava droplet.
///
/// Instead, consider only cube sides that could be reached by the water and steam
/// as the lava droplet tumbles into the pond. The steam will expand to reach as
/// much as possible, completely displacing any air on the outside of the lava droplet
/// but never expanding diagonally.
///
/// In the larger example above, exactly one cube of air is trapped within the lava
/// droplet (at `2,2,5`), so the exterior surface area of the lava droplet is `*58*`.
///
/// *What is the exterior surface area of your scanned lava droplet?*
///
pub fn part2(input: &str) -> i64 {
    let points: FxHashSet<(i64, i64, i64)> = input.non_empty().map(|l| l.nums().tup()).collect();

    let top_point = *points.iter().max_by_key(|(x, _, _)| x).unwrap();
    let top_point = (top_point.0 + 1, top_point.1, top_point.2);

    let mut steam = FxHashSet::default();
    steam.insert(top_point);
    let mut steam_frontier = FxHashSet::default();
    steam_frontier.insert(top_point);

    loop {
        let mut new_frontier = FxHashSet::default();

        for s in steam_frontier.iter() {
            for n in DELTAS.map(|d| (d.0 + s.0, d.1 + s.1, d.2 + s.2)) {
                if !steam.contains(&n)
                    && !points.contains(&n)
                    && DELTAS_FULL
                        .iter()
                        .map(|d| (d.0 + n.0, d.1 + n.1, d.2 + n.2))
                        .any(|p| points.contains(&p))
                {
                    new_frontier.insert(n);
                }
            }
        }

        if new_frontier.is_empty() {
            break;
        }

        steam.extend(new_frontier.iter());
        steam_frontier.clear();
        steam_frontier.extend(new_frontier);
    }

    points
        .iter()
        .flat_map(|p| DELTAS.map(|d| (p.0 + d.0, p.1 + d.1, p.2 + d.2)))
        .filter(|p| steam.contains(p))
        .count() as i64
}

const PART1_EX_ANSWER: &str = "64";
const PART1_ANSWER: &str = "4390";
const PART2_EX_ANSWER: &str = "58";
const PART2_ANSWER: &str = "2534";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
