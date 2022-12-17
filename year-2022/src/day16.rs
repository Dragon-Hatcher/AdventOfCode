use std::{cmp::Ordering, fmt::Debug};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{helpers::IterExtension, standard_parsers::AocParsed};

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

    let rates = {
        let mut rates = [0; 64];
        for i in 0..64 {
            rates[i] = valves.get(i).map(|v| v.rate).unwrap_or_default();
        }
        rates
    };

    fn calc_released(open: u64, rates: &[i64; 64]) -> i64 {
        let mut released_now = 0;
        for i in 0..64 {
            released_now += rates[i] * ((1 << i) & open != 0) as i64;
        }
        released_now
    }

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

        fn set(self, valve: usize) -> Self {
            Self(self.0 | (1 << valve))
        }

        fn is_superset_of(self, other: ValveSet) -> bool {
            (self.0 | other.0) == self.0
        }

        fn has_disjunction(self, other: ValveSet) -> bool {
            (self.0 ^ other.0) != 0
        }

        fn calc_released(self, released: &[i64; 64]) -> i64 {
            (0..64)
                .map(|valve| released[valve] * self.is_set(valve) as i64)
                .sum()
        }
    }

    #[derive(Clone, Copy)]
    struct State {
        loc: usize,
        open: ValveSet,
        visited_since_open: ValveSet,
        released: i64,
    }

    impl State {
        fn next_options(
            &self,
            res: &mut FxHashMap<usize, Vec<State>>,
            valves: &[Valve],
            rates: &[i64; 64],
            full_valve_set: ValveSet,
        ) {
            let released = self.released + self.open.calc_released(rates);

            if self.open == full_valve_set {
                let new_state = State { released, ..*self };
                res.entry(new_state.loc)
                    .or_insert_with(|| Vec::default())
                    .push(new_state);
                return;
            }

            if !self.open.is_set(self.loc) && valves[self.loc].rate != 0 {
                let new_state = State {
                    open: self.open.set(self.loc),
                    released,
                    visited_since_open: ValveSet::default(),
                    ..*self
                };
                res.entry(new_state.loc)
                    .or_insert_with(|| Vec::default())
                    .push(new_state)
            }

            for connection in &valves[self.loc].connections {
                if self.visited_since_open.is_set(*connection) {
                    continue;
                }

                let visited_since_open = self.visited_since_open.set(self.loc);
                let new_state = State {
                    loc: *connection,
                    open: self.open,
                    visited_since_open,
                    released,
                };
                res.entry(new_state.loc)
                    .or_insert_with(|| Vec::default())
                    .push(new_state)
            }
        }
    }

    impl Debug for State {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "State {{ loc: {:?}, open: {:?}, visited_since_open: {:?}, released: {:?} }}",
                ('A' as u8 + self.loc as u8) as char,
                self.open,
                self.visited_since_open,
                self.released
            )
        }
    }

    impl PartialEq for State {
        fn eq(&self, other: &Self) -> bool {
            // really means one isn't strictly worse than the other
            self.cmp(other).is_eq()
        }
    }

    impl Eq for State {}

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.loc != other.loc {
                Ordering::Equal
            } else if self.open == other.open && self.released == other.released {
                Ordering::Equal
            } else if self.open.is_superset_of(other.open) && self.released >= other.released {
                Ordering::Greater
            } else if other.open.is_superset_of(self.open) && other.released > self.released {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    }

    let mut current_states: FxHashMap<usize, Vec<State>> = FxHashMap::default();
    let start = names.iter().position(|n| *n == "AA").unwrap();
    current_states.insert(
        start,
        vec![State {
            loc: start,
            open: ValveSet::default(),
            visited_since_open: ValveSet::default(),
            released: 0,
        }],
    );

    let full_valve_set = {
        let mut set = ValveSet::default();
        for i in 0..valves.len() {
            set = set.set(i);
        }
        set
    };

    // println!("{valves:?}");

    for min in 0..30 {
        let mut frontier = FxHashMap::default();

        // get next steps
        current_states
            .values()
            .flatten()
            .for_each(|s| s.next_options(&mut frontier, &valves, &rates, full_valve_set));

        // println!("min {min} -> {}.", min + 1);
        // println!("  pre  {}", current_states.values().flatten().count());

        // reduce next steps
        // if min != 13 {
        for (loc, items) in frontier.iter_mut() {
            // let debug = *loc == 5 && min == 13 && false;

            let mut items_iter = items.iter();
            let mut diff = vec![];
            if let Some(i) = items_iter.next() {
                diff.push(*i);
                for i in items_iter {
                    // if debug {
                    //     println!("{diff:?}");
                    // }
                    let mut greater = false;
                    let mut eq = false;
                    let mut exact_eq = false;
                    for d in &diff {
                        if i.loc == d.loc
                            && i.open == d.open
                            && i.visited_since_open == d.visited_since_open
                            && i.released == d.released
                        {
                            exact_eq = true;
                            break;
                        }
                        match i.cmp(&d) {
                            Ordering::Less => {}
                            Ordering::Equal => {
                                // let pos = diff.binary_search(i).unwrap_or_else(|e| e);
                                // diff.insert(pos, *i);
                                eq = true;
                                break;
                            }
                            Ordering::Greater => {
                                greater = true;
                            }
                        }
                    }
                    if exact_eq {
                    } else if eq {
                        diff.push(*i);
                    } else if greater {
                        diff.clear();
                        diff.push(*i);
                    }

                    // match i.cmp(diff.first().unwrap()) {
                    //     Ordering::Less => {}
                    //     Ordering::Equal => {
                    //         let pos = diff.binary_search(i).unwrap_or_else(|e| e);
                    //         diff.insert(pos, *i);
                    //         // diff.push(*i)
                    //     }
                    //     Ordering::Greater => {
                    //         diff.clear();
                    //         diff.push(*i);
                    //     }
                }
            }
            _ = std::mem::replace(items, diff);
        }
        // }
        current_states = frontier;

        let count = current_states.values().flatten().count();
        let correct = [
            0, 0, 20, 40, 60, 93, 126, 159, 192, 246, 300, 354, 408, 462, 516, 570, 624, 700, 776,
            852, 928, 1007, 1086, 1165, 1246, 1327, 1408, 1489, 1570, 1651,
        ];

        // println!("  post {}", count);
        // current_states
        //     .values()
        //     .flatten()
        //     .filter(|v| v.released == correct[min])
        //     // .sorted_by_key(|v| -v.released)
        //     // .take(10)
        //     .for_each(|s| println!("    {s:?}"))
    }

    current_states
        .values()
        .flatten()
        .map(|s| s.released)
        .max()
        .unwrap_or_default()

    // fn upper_bound(mins_remaining: usize, rates: &[i64; 64], open: u64) -> i64 {
    //     let mut already = 0;

    //     let mut rates = *rates;
    //     for i in 0..64 {
    //         if (1 << i) & open != 0 {
    //             already += rates[i] * mins_remaining as i64;
    //             rates[i] = 0;
    //         }
    //     }

    //     rates
    //         .iter()
    //         .sorted()
    //         .rev()
    //         .enumerate()
    //         .take((mins_remaining + 1) / 2)
    //         .map(|(i, r)| *r * (mins_remaining as i64 - i as i64 * 2))
    //         .sum::<i64>()
    //         + already
    // }

    // fn solve(
    //     min: usize,
    //     valves: &[Valve],
    //     rates: &[i64; 64],
    //     open: ValveSet,
    //     loc: usize,
    //     visited: u64,
    // ) -> i64 {
    //     if min >= 30 {
    //         return 0;
    //     }

    //     let released_now = open.calc_released(rates);
    //     let cur_valve = &valves[loc];

    //     // if loc_times.iter().any(|(o, l, m, s)| {
    //     //     *o == open && *l == loc && min >= *m && *s >= tot_so_far + released_now
    //     // }) {
    //     //     // println!("{open} @ {loc} but {min}");
    //     //     // println!("xxx @ {min}");
    //     //     return released_now * (30 - min) as i64;
    //     // }

    //     // loc_times.push((open, loc, min, tot_so_far + released_now));

    //     // let nothing_value = solve(min + 1, valves, rates, open, loc, best_known);

    //     let move_value = cur_valve
    //         .connections
    //         .iter()
    //         .flat_map(|new_loc| {
    //             if visited & (1 << new_loc) == 0 {
    //                 let new_visited = visited | (1 << new_loc);
    //                 Some(solve(min + 1, valves, &rates, open, *new_loc, new_visited))
    //             } else {
    //                 None
    //             }
    //         })
    //         .max()
    //         .unwrap_or_default();

    //     let open_value = if cur_valve.rate != 0 && (1 << loc) & open == 0 {
    //         let new_opened = open ^ (1 << loc);
    //         solve(min + 1, valves, rates, new_opened, loc, 0)
    //     } else {
    //         0
    //     };

    //     if min < 4 {
    //         print!("{}", "|".repeat(min));
    //         print_open(open);
    //         println!("     {min}: +{released_now} loc {loc} with = {move_value} & {open_value}");
    //     }

    //     released_now + (move_value).max(open_value)
    // }

    // let s = solve(
    //     // 4,
    //     0,
    //     &valves,
    //     &rates,
    //     // 0 | (1 << 3),
    //     0,
    //     // 1,
    //     names.iter().position(|n| *n == "AA").unwrap(),
    //     0,
    //     // &mut vec![],
    // );

    // println!("{s}");
    // todo!()
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
