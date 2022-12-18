use crate::{helpers::IterExtension, standard_parsers::AocParsed};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::Debug;

///
/// --- Day 16: Proboscidea Volcanium ---
///
/// The sensors have led you to the origin of the distress signal: yet another handheld
/// device, just like the one the Elves gave you. However, you don't see any Elves
/// around; instead, the device is surrounded by elephants! They must have gotten
/// lost in these tunnels, and one of the elephants apparently figured out how to
/// turn on the distress signal.
///
/// The ground rumbles again, much stronger this time. What kind of cave is this,
/// exactly? You scan the cave with your handheld device; it reports mostly igneous
/// rock, some ash, pockets of pressurized gas, magma... this isn't just a cave,
/// it's a volcano!
///
/// You need to get the elephants out of here, quickly. Your device estimates that
/// you have *30 minutes* before the volcano erupts, so you don't have time to go
/// back out the way you came in.
///
/// You scan the cave for other options and discover a network of pipes and pressure-release
/// *valves*. You aren't sure how such a system got into a volcano, but you don't
/// have time to complain; your device produces a report (your puzzle input) of each
/// valve's *flow rate* if it were opened (in pressure per minute) and the tunnels
/// you could use to move between the valves.
///
/// There's even a valve in the room you and the elephants are currently standing
/// in labeled `AA`. You estimate it will take you one minute to open a single valve
/// and one minute to follow any tunnel from one valve to another. What is the most
/// pressure you could release?
///
/// For example, suppose you had the following scan output:
///
/// ```
/// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
/// Valve BB has flow rate=13; tunnels lead to valves CC, AA
/// Valve CC has flow rate=2; tunnels lead to valves DD, BB
/// Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
/// Valve EE has flow rate=3; tunnels lead to valves FF, DD
/// Valve FF has flow rate=0; tunnels lead to valves EE, GG
/// Valve GG has flow rate=0; tunnels lead to valves FF, HH
/// Valve HH has flow rate=22; tunnel leads to valve GG
/// Valve II has flow rate=0; tunnels lead to valves AA, JJ
/// Valve JJ has flow rate=21; tunnel leads to valve II
///
/// ```
///
/// All of the valves begin *closed*. You start at valve `AA`, but it must be damaged
/// or jammed or something: its flow rate is `0`, so there's no point in opening
/// it. However, you could spend one minute moving to valve `BB` and another minute
/// opening it; doing so would release pressure during the remaining *28 minutes*
/// at a flow rate of `13`, a total eventual pressure release of `28 * 13 = *364*`.
/// Then, you could spend your third minute moving to valve `CC` and your fourth
/// minute opening it, providing an additional *26 minutes* of eventual pressure
/// release at a flow rate of `2`, or `*52*` total pressure released by valve `CC`.
///
/// Making your way through the tunnels like this, you could probably open many or
/// all of the valves by the time 30 minutes have elapsed. However, you need to release
/// as much pressure as possible, so you'll need to be methodical. Instead, consider
/// this approach:
///
/// ```
/// == Minute 1 ==
/// No valves are open.
/// You move to valve DD.
///
/// == Minute 2 ==
/// No valves are open.
/// You open valve DD.
///
/// == Minute 3 ==
/// Valve DD is open, releasing 20 pressure.
/// You move to valve CC.
///
/// == Minute 4 ==
/// Valve DD is open, releasing 20 pressure.
/// You move to valve BB.
///
/// == Minute 5 ==
/// Valve DD is open, releasing 20 pressure.
/// You open valve BB.
///
/// == Minute 6 ==
/// Valves BB and DD are open, releasing 33 pressure.
/// You move to valve AA.
///
/// == Minute 7 ==
/// Valves BB and DD are open, releasing 33 pressure.
/// You move to valve II.
///
/// == Minute 8 ==
/// Valves BB and DD are open, releasing 33 pressure.
/// You move to valve JJ.
///
/// == Minute 9 ==
/// Valves BB and DD are open, releasing 33 pressure.
/// You open valve JJ.
///
/// == Minute 10 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve II.
///
/// == Minute 11 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve AA.
///
/// == Minute 12 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve DD.
///
/// == Minute 13 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve EE.
///
/// == Minute 14 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve FF.
///
/// == Minute 15 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve GG.
///
/// == Minute 16 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve HH.
///
/// == Minute 17 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You open valve HH.
///
/// == Minute 18 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You move to valve GG.
///
/// == Minute 19 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You move to valve FF.
///
/// == Minute 20 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You move to valve EE.
///
/// == Minute 21 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You open valve EE.
///
/// == Minute 22 ==
/// Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
/// You move to valve DD.
///
/// == Minute 23 ==
/// Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
/// You move to valve CC.
///
/// == Minute 24 ==
/// Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
/// You open valve CC.
///
/// == Minute 25 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// == Minute 26 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// == Minute 27 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// == Minute 28 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// == Minute 29 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// == Minute 30 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// ```
///
/// This approach lets you release the most pressure possible in 30 minutes with
/// this valve layout, `*1651*`.
///
/// Work out the steps to release the most pressure in 30 minutes. *What is the most
/// pressure you can release?*
///
pub fn part1(input: &str) -> i64 {
    let names = input.non_empty().map(|l| &l[6..8]).collect_vec();

    #[derive(Debug)]
    struct Valve {
        rate: i64,
        connections: Vec<usize>,
    }

    lazy_static! {
        static ref RE: Regex = Regex::new("[A-Z]{2}").unwrap();
    }

    let valves = input
        .non_empty()
        .map(|l| {
            let rate = l.nums().nu();
            let connections = RE
                .captures_iter(l)
                .skip(1)
                .map(|c| names.iter().position(|i| *i == &c[0]).unwrap())
                .collect_vec();
            Valve { rate, connections }
        })
        .collect_vec();

    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    struct ValveSet(u64);

    impl Debug for ValveSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;

            for i in 0..64 {
                if self.is_set(i) {
                    write!(f, "{}", ('A' as usize + i) as u8 as char)?;
                }
            }

            write!(f, "]")
        }
    }

    impl ValveSet {
        fn is_set(self, valve: usize) -> bool {
            self.0 & (1 << valve) != 0
        }

        fn toggle(&mut self, valve: usize) {
            self.0 ^= 1 << valve;
        }
    }

    fn calc_distance(from: usize, res: &mut FxHashMap<(usize, usize), usize>, valves: &[Valve]) {
        let mut dist = 0;
        let mut visited = FxHashSet::default();
        visited.insert(from);
        let mut frontier = visited.clone();

        loop {
            dist += 1;

            let mut new_frontier = FxHashSet::default();

            for v in frontier.iter() {
                for n in valves[*v].connections.iter() {
                    if !visited.contains(&n) {
                        res.insert((from, *n), dist);
                        new_frontier.insert(*n);
                    }
                }
            }

            visited.extend(new_frontier.iter());
            frontier.clear();
            frontier.extend(new_frontier);

            if visited.len() == valves.len() {
                break;
            }
        }
    }

    fn calc_distances(
        closed_valves: &ValveSet,
        valves: &[Valve],
    ) -> FxHashMap<(usize, usize), usize> {
        let mut ret = FxHashMap::default();
        for from in 0..64 {
            if !closed_valves.is_set(from) {
                continue;
            }
            calc_distance(from, &mut ret, valves);
        }
        ret
    }

    fn solve(
        mins_left: usize,
        loc: usize,
        cur_flow_rate: i64,
        closed_valves: &mut ValveSet,
        distances: &FxHashMap<(usize, usize), usize>,
        valves: &[Valve],
    ) -> i64 {
        let mut max = cur_flow_rate * mins_left as i64;

        for dest in 0..64 {
            if !closed_valves.is_set(dest) {
                continue;
            }

            let dist = distances.get(&(loc, dest)).unwrap_or(&usize::MAX);
            let time = dist + 1;
            if time < mins_left {
                let new_flow_rate = cur_flow_rate + valves[dest].rate;
                closed_valves.toggle(dest);
                let total_released = time as i64 * cur_flow_rate
                    + solve(
                        mins_left - time,
                        dest,
                        new_flow_rate,
                        closed_valves,
                        distances,
                        valves,
                    );
                closed_valves.toggle(dest);
                max = total_released.max(max);
            }
        }

        max
    }

    let start = names.iter().position(|n| *n == "AA").unwrap();
    let mut closed_valves = ValveSet(0);
    for i in 0..valves.len() {
        if valves[i].rate != 0 {
            closed_valves.toggle(i);
        }
    }

    closed_valves.toggle(start);
    let distances = calc_distances(&closed_valves, &valves);
    closed_valves.toggle(start);

    solve(30, start, 0, &mut closed_valves, &distances, &valves)
}

pub fn part2(input: &str) -> i64 {
    0
}

const PART1_EX_ANSWER: &str = "1651";
const PART1_ANSWER: &str = "1724";
const PART2_EX_ANSWER: &str = "0";
const PART2_ANSWER: &str = "0";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
