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
    fn print_open(open: u64) {
        for i in 0..64 {
            if open & (1 << i) != 0 {
                print!("{}", ('A' as usize + i) as u8 as char);
            }
        }
    }

    fn solve(
        min: usize,
        valves: &[Valve],
        rates: &[i64; 64],
        open: u64,
        loc: usize,
        // tot_so_far: i64,
        visited: u64,
        // best_known: &mut FxHashMap<usize, i64>,
        // loc_times: &mut Vec<(u64, usize, usize, i64)>,
    ) -> i64 {
        if min >= 30 {
            return 0;
        }

        // let upper_bound = upper_bound(30 - min, rates, open);
        // let this_best_known = *best_known.get(&min).unwrap_or(&-1);
        // if min <= 25 && upper_bound < this_best_known {
        //     if min <= 20 {
        //         println!("Skip at min {min}");
        //     }
        //     return 0;
        // }

        let released_now = calc_released(open, rates);
        let cur_valve = &valves[loc];

        // if loc_times.iter().any(|(o, l, m, s)| {
        //     *o == open && *l == loc && min >= *m && *s >= tot_so_far + released_now
        // }) {
        //     // println!("{open} @ {loc} but {min}");
        //     // println!("xxx @ {min}");
        //     return released_now * (30 - min) as i64;
        // }

        // loc_times.push((open, loc, min, tot_so_far + released_now));

        // let nothing_value = solve(min + 1, valves, rates, open, loc, best_known);

        let move_value = cur_valve
            .connections
            .iter()
            .flat_map(|new_loc| {
                if visited & (1 << new_loc) == 0 {
                    let new_visited = visited | (1 << new_loc);
                    Some(solve(min + 1, valves, &rates, open, *new_loc, new_visited))
                } else {
                    None
                }
            })
            .max()
            .unwrap_or_default();

        let open_value = if cur_valve.rate != 0 && (1 << loc) & open == 0 {
            let new_opened = open ^ (1 << loc);
            solve(min + 1, valves, rates, new_opened, loc, 0)
        } else {
            0
        };

        if min < 4 {
            print!("{}", "|".repeat(min));
            print_open(open);
            println!("     {min}: +{released_now} loc {loc} with = {move_value} & {open_value}");
        }

        released_now + (move_value).max(open_value)
    }

    let s = solve(
        // 4,
        0,
        &valves,
        &rates,
        // 0 | (1 << 3),
        0,
        // 1,
        names.iter().position(|n| *n == "AA").unwrap(),
        0,
        // &mut vec![],
    );

    // println!("{s}");
    // todo!()

    s
}

pub fn part2(input: &str) -> i64 {
    0
}

const PART1_EX_ANSWER: &str = "1651";
const PART1_ANSWER: &str = "0";
const PART2_EX_ANSWER: &str = "0";
const PART2_ANSWER: &str = "0";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
