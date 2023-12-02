use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 16)
}

fn parse(input: &str) -> (usize, Vec<Valve>) {
    let re = regex!("[A-Z]{2}");

    let names = input.non_empty().map(|l| &l[6..8]).collect_vec();
    let start = names.iter().position(|n| *n == "AA").unwrap();

    let valves = input
        .non_empty()
        .map(|l| {
            let rate = l.nums().nu();
            let connections = re
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

fn part1(input: &str) -> i64 {
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

fn part2(input: &str) -> i64 {
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

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
    ";
    assert_eq!(part1(input), 1651);
    assert_eq!(part2(input), 1707);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1724);
    assert_eq!(part2(input), 2283);
}
