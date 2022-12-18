use crate::{helpers::IterExtension, standard_parsers::AocParsed};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::Debug;

fn parse(input: &str) -> (usize, Vec<Valve>) {
    lazy_static! {
        static ref RE: Regex = Regex::new("[A-Z]{2}").unwrap();
    }

    let names = input.non_empty().map(|l| &l[6..8]).collect_vec();
    let start = names.iter().position(|n| *n == "AA").unwrap();

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
        .collect();

    (start, valves)
}

#[derive(Debug)]
struct Valve {
    rate: i64,
    connections: Vec<usize>,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct ValveSet(u64);

impl ValveSet {
    fn is_set(self, valve: usize) -> bool {
        self.0 & (1 << valve) != 0
    }

    fn toggle(self, valve: usize) -> Self {
        Self(self.0 ^ (1 << valve))
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

fn calc_distances(closed_valves: ValveSet, valves: &[Valve]) -> FxHashMap<(usize, usize), usize> {
    let mut ret = FxHashMap::default();
    for from in 0..64 {
        if !closed_valves.is_set(from) {
            continue;
        }
        calc_distance(from, &mut ret, valves);
    }
    ret
}

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
    fn solve(
        mins_left: usize,
        loc: usize,
        cur_flow_rate: i64,
        closed_valves: ValveSet,
        distances: &FxHashMap<(usize, usize), usize>,
        valves: &[Valve],
    ) -> i64 {
        let mut max = cur_flow_rate * mins_left as i64;

        for dest in 0..64 {
            if !closed_valves.is_set(dest) {
                continue;
            }

            let time = distances
                .get(&(loc, dest))
                .unwrap_or(&usize::MAX)
                .saturating_add(1);
            if time < mins_left {
                let new_flow_rate = cur_flow_rate + valves[dest].rate;
                let total_released = time as i64 * cur_flow_rate
                    + solve(
                        mins_left - time,
                        dest,
                        new_flow_rate,
                        closed_valves.toggle(dest),
                        distances,
                        valves,
                    );
                max = total_released.max(max);
            }
        }

        max
    }

    let (start, valves) = parse(input);
    let mut closed_valves = ValveSet(0);
    for i in 0..valves.len() {
        if valves[i].rate != 0 {
            closed_valves = closed_valves.toggle(i);
        }
    }

    let distances = calc_distances(closed_valves.toggle(start), &valves);

    solve(30, start, 0, closed_valves, &distances, &valves)
}

///
/// --- Part Two ---
///
/// You're worried that even with an optimal approach, the pressure released won't
/// be enough. What if you got one of the elephants to help you?
///
/// It would take you 4 minutes to teach an elephant how to open the right valves
/// in the right order, leaving you with only *26 minutes* to actually execute your
/// plan. Would having two of you working together be better, even if it means having
/// less time? (Assume that you teach the elephant before opening any valves yourself,
/// giving you both the same full 26 minutes.)
///
/// In the example above, you could teach the elephant to help you as follows:
///
/// ```
/// == Minute 1 ==
/// No valves are open.
/// You move to valve II.
/// The elephant moves to valve DD.
///
/// == Minute 2 ==
/// No valves are open.
/// You move to valve JJ.
/// The elephant opens valve DD.
///
/// == Minute 3 ==
/// Valve DD is open, releasing 20 pressure.
/// You open valve JJ.
/// The elephant moves to valve EE.
///
/// == Minute 4 ==
/// Valves DD and JJ are open, releasing 41 pressure.
/// You move to valve II.
/// The elephant moves to valve FF.
///
/// == Minute 5 ==
/// Valves DD and JJ are open, releasing 41 pressure.
/// You move to valve AA.
/// The elephant moves to valve GG.
///
/// == Minute 6 ==
/// Valves DD and JJ are open, releasing 41 pressure.
/// You move to valve BB.
/// The elephant moves to valve HH.
///
/// == Minute 7 ==
/// Valves DD and JJ are open, releasing 41 pressure.
/// You open valve BB.
/// The elephant opens valve HH.
///
/// == Minute 8 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You move to valve CC.
/// The elephant moves to valve GG.
///
/// == Minute 9 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You open valve CC.
/// The elephant moves to valve FF.
///
/// == Minute 10 ==
/// Valves BB, CC, DD, HH, and JJ are open, releasing 78 pressure.
/// The elephant moves to valve EE.
///
/// == Minute 11 ==
/// Valves BB, CC, DD, HH, and JJ are open, releasing 78 pressure.
/// The elephant opens valve EE.
///
/// (At this point, all valves are open.)
///
/// == Minute 12 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// ...
///
/// == Minute 20 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// ...
///
/// == Minute 26 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// ```
///
/// With the elephant helping, after 26 minutes, the best you could do would release
/// a total of `*1707*` pressure.
///
/// *With you and an elephant working together for 26 minutes, what is the most pressure
/// you could release?*
///
pub fn part2(input: &str) -> i64 {
    fn solve(
        mins_left: usize,
        loc1: usize,
        cooldown1: usize,
        loc2: usize,
        cooldown2: usize,
        cur_flow_rate: i64,
        closed_valves: ValveSet,
        distances: &FxHashMap<(usize, usize), usize>,
        valves: &[Valve],
    ) -> i64 {
        if cooldown1 == 0 {
            // Pick a new destination for agent 1

            let new_flow_rate = cur_flow_rate + valves[loc1].rate;
            let mut max = new_flow_rate * mins_left as i64
                + (mins_left - cooldown2) as i64 * valves[loc2].rate;

            for dest in 0..64 {
                if !closed_valves.is_set(dest) {
                    continue;
                }

                let time = distances
                    .get(&(loc1, dest))
                    .unwrap_or(&usize::MAX)
                    .saturating_add(1);

                // TODO what if agent1 can't but agent 2 can?
                if time < mins_left {
                    let to_elapse = time.min(cooldown2);
                    let total_released = to_elapse as i64 * new_flow_rate
                        + solve(
                            mins_left - to_elapse,
                            dest,
                            time - to_elapse,
                            loc2,
                            cooldown2 - to_elapse,
                            new_flow_rate,
                            closed_valves.toggle(dest),
                            distances,
                            valves,
                        );

                    max = total_released.max(max);
                }
            }

            max
        } else if cooldown2 == 0 {
            // Swap agents

            solve(
                mins_left,
                loc2,
                cooldown2,
                loc1,
                cooldown1,
                cur_flow_rate,
                closed_valves,
                distances,
                valves,
            )
        } else {
            unreachable!()
        }
    }

    let (start, valves) = parse(input);
    let mut closed_valves = ValveSet(0);
    for i in 0..valves.len() {
        if valves[i].rate != 0 {
            closed_valves = closed_valves.toggle(i);
        }
    }

    let distances = calc_distances(closed_valves.toggle(start), &valves);

    solve(
        26,
        start,
        0,
        start,
        0,
        0,
        closed_valves,
        &distances,
        &valves,
    )
}

const PART1_EX_ANSWER: &str = "1651";
const PART1_ANSWER: &str = "1724";
const PART2_EX_ANSWER: &str = "1707";
const PART2_ANSWER: &str = "2283";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
