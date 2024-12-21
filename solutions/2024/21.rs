use std::{
    i64,
    iter::{empty, once},
    sync::OnceLock,
};

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 21)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    dirs: Vec<Vec2>,
    num: Vec2,
    left: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Action {
    Press,
    Move(Direction),
}

impl Action {
    const ALL: &'static [Action] = &[
        Action::Press,
        Action::Move(Direction::Up),
        Action::Move(Direction::Left),
        Action::Move(Direction::Down),
        Action::Move(Direction::Right),
    ];
}

fn dir_pad() -> &'static HashMap<Action, Vec2> {
    static DIR_PAD: OnceLock<HashMap<Action, Vec2>> = OnceLock::new();
    DIR_PAD.get_or_init(|| {
        hashmap!(
            Action::Move(Direction::Up) => v2(1, 0),
            Action::Move(Direction::Left) => v2(0, 1),
            Action::Move(Direction::Down) => v2(1, 1),
            Action::Move(Direction::Right) => v2(2, 1),
            Action::Press => v2(2, 0),
        )
    })
}

fn num_pad() -> &'static HashMap<char, Vec2> {
    static DIR_PAD: OnceLock<HashMap<char, Vec2>> = OnceLock::new();
    DIR_PAD.get_or_init(|| {
        hashmap!(
            '7' => v2(0, 0),
            '8' => v2(1, 0),
            '9' => v2(2, 0),
            '4' => v2(0, 1),
            '5' => v2(1, 1),
            '6' => v2(2, 1),
            '1' => v2(0, 2),
            '2' => v2(1, 2),
            '3' => v2(2, 2),
            '0' => v2(1, 3),
            'A' => v2(2, 3),
        )
    })
}

fn solve_one(goal: &str, n: usize) -> i64 {
    let mut transition_costs: HashMap<(Action, Action), i64> = HashMap::default();

    for &p1 in dir_pad().keys() {
        for &p2 in dir_pad().keys() {
            transition_costs.insert((p1, p2), 1);
        }
    }

    for _ in 0..n {
        let mut new_trans_costs = HashMap::default();

        for &(from, to) in transition_costs.keys() {
            let from_p = dir_pad()[&from];
            let to_p = dir_pad()[&to];

            let res = dijkstra()
                .start((from_p, Action::Press))
                .goal((to_p, Action::Press))
                .next(|&(at, last_act)| {
                    let mut res = vec![];

                    for &act in Action::ALL {
                        let cost = transition_costs[&(last_act, act)];
                        match act {
                            Action::Press => {
                                if at == to_p {
                                    res.push(((at, act), cost));
                                }
                            }
                            Action::Move(d) => {
                                let next = at + d.vector();
                                if dir_pad().values().contains(&next) {
                                    res.push(((next, act), cost));
                                }
                            }
                        }
                    }

                    res.into_iter()
                })
                .search();
            let dist = res.dists[&res.stop_node.unwrap()].max(1);
            // println!("{from:?} ---({dist})--> {to:?}");

            new_trans_costs.insert((from, to), dist);
        }

        transition_costs = new_trans_costs;
    }

    let mut cost = 0;
    for (from, to) in once('A').chain(goal.chars()).tuple_windows() {
        let from = num_pad()[&from];
        let to = num_pad()[&to];

        let res = dijkstra()
            .start((from, Action::Press))
            .goal((to, Action::Press))
            .next(|&(at, last_act)| {
                let mut res = vec![];

                for &act in Action::ALL {
                    let cost = transition_costs[&(last_act, act)];
                    match act {
                        Action::Press => {
                            if at == to {
                                res.push(((at, act), cost));
                            }
                        }
                        Action::Move(d) => {
                            let next = at + d.vector();
                            if num_pad().values().contains(&next) {
                                res.push(((next, act), cost));
                            }
                        }
                    }
                }

                res.into_iter()
            })
            .search();
        let dist = res.dists[&res.stop_node.unwrap()];
        cost += dist;
        // println!("{from} ---({dist})--> {to}");
    }

    cost

    // let mut open = MinBinaryHeap::new_min();
    // let mut seen = HashSet::default();
    // let mut g_score = HashMap::default();

    // let start = State::start(goal.to_owned(), n);
    // open.push((0, start.clone()));
    // g_score.insert(start.clone(), 0);
    // seen.insert(start.clone());

    // let mut lowest_seen = start.left.len();

    // while let Some((_, curr)) = open.pop() {
    //     let my_dist = g_score[&curr];

    //     if curr.left.is_empty() {
    //         return my_dist;
    //     }

    //     for n in Action::ALL
    //         .into_iter()
    //         .filter_map(move |act| curr.clone().tick_dir(*act, 0))
    //     {
    //         if !seen.contains(&n) {
    //             seen.insert(n.clone());
    //             g_score.insert(n.clone(), my_dist + 1);
    //             let h = (n.left.len() as i64);
    //             open.push((my_dist + 1 + h, n));
    //         }
    //     }
    // }

    // unreachable!()
}

fn solve(input: &str, n: usize) -> i64 {
    let mut sum = 0;
    for l in input.lines() {
        let num = l.nums().nu();
        let len = solve_one(l.trim(), n);
        sum += len * num;
    }
    sum
}

fn part1(input: &str) -> i64 {
    solve(input, 2)
}

fn part2(input: &str) -> i64 {
    solve(input, 25)
}

fn main() {
    advent::new(2024, 21, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "029A";
    assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
