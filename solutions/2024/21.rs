use advent::prelude::*;
use std::{i64, iter::once, sync::OnceLock};

fn default_input() -> &'static str {
    include_input!(2024 / 21)
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

fn find_cost_from_to(
    from: Vec2,
    to: Vec2,
    costs: &HashMap<(Action, Action), i64>,
    valid_squares: &HashSet<Vec2>,
) -> i64 {
    let res = dijkstra()
        .start((from, Action::Press))
        .goal((to, Action::Press))
        .next(|&(at, last_act)| {
            Action::ALL.into_iter().filter_map(move |&act| {
                let cost = costs[&(last_act, act)];
                match act {
                    Action::Press => Some(((at, act), cost)),
                    Action::Move(d) => {
                        let next = at + d.vector();
                        valid_squares.contains(&next).then_some(((next, act), cost))
                    }
                }
            })
        })
        .search();

    res.dists[&res.stop_node.unwrap()]
}

fn solve_one(goal: &str, n: usize) -> i64 {
    let mut transition_costs: HashMap<(Action, Action), i64> = HashMap::default();

    // Initially the cost of any action is one button press on the top most
    // direction pad.
    for &p1 in dir_pad().keys() {
        for &p2 in dir_pad().keys() {
            transition_costs.insert((p1, p2), 1);
        }
    }

    let valid_dir_squares = dir_pad().values().copied().collect();
    let valid_num_squares = num_pad().values().copied().collect();

    // But now we see how much each transition costs for each additional
    // intermediate direction pad.
    for _ in 0..n {
        let mut new_trans_costs = HashMap::default();

        for &(from, to) in transition_costs.keys() {
            if from == to {
                // We just press the press button again.
                new_trans_costs.insert((from, to), 1);
                continue;
            }

            new_trans_costs.insert(
                (from, to),
                find_cost_from_to(
                    dir_pad()[&from],
                    dir_pad()[&to],
                    &transition_costs,
                    &valid_dir_squares,
                ),
            );
        }

        transition_costs = new_trans_costs;
    }

    // And finally we get the cost to do the required sequence of moves on the
    // number pad.
    once('A')
        .chain(goal.chars())
        .tuple_windows()
        .map(|(from, to)| {
            find_cost_from_to(
                num_pad()[&from],
                num_pad()[&to],
                &transition_costs,
                &valid_num_squares,
            )
        })
        .sum()
}

fn solve(input: &str, n: usize) -> i64 {
    input
        .lines()
        .map(|code| code.nums().nu() * solve_one(code, n))
        .sum()
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
    let input = "029A
980A
179A
456A
379A";
    assert_eq!(part1(input), 126384);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 231564);
    assert_eq!(part2(input), 281212077733592);
}
