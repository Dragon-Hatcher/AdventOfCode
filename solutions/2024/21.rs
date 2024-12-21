use std::{i64, sync::OnceLock};

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 21)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    dir1: Vec2,
    dir2: Vec2,
    num: Vec2,
}

#[derive(Debug, Clone, Copy)]
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

fn dir_pad() -> &'static HashMap<Vec2, Action> {
    static DIR_PAD: OnceLock<HashMap<Vec2, Action>> = OnceLock::new();
    DIR_PAD.get_or_init(|| {
        hashmap!(
            v2(1, 0) => Action::Move(Direction::Up),
            v2(0, 1) => Action::Move(Direction::Left),
            v2(1, 1) => Action::Move(Direction::Down),
            v2(2, 1) => Action::Move(Direction::Right),
            v2(2, 0) => Action::Press,
        )
    })
}

fn num_pad() -> &'static HashMap<Vec2, char> {
    static DIR_PAD: OnceLock<HashMap<Vec2, char>> = OnceLock::new();
    DIR_PAD.get_or_init(|| {
        hashmap!(
            v2(0, 0) => '7',
            v2(1, 0) => '8',
            v2(2, 0) => '9',
            v2(0, 1) => '4',
            v2(1, 1) => '5',
            v2(2, 1) => '6',
            v2(0, 2) => '1',
            v2(1, 2) => '2',
            v2(2, 2) => '3',
            v2(1, 3) => '0',
            v2(2, 3) => 'A',
        )
    })
}

impl State {
    fn start() -> Self {
        Self {
            dir1: v2(2, 0),
            dir2: v2(2, 0),
            num: v2(2, 3),
        }
    }

    fn tick_dir_1(mut self, action: Action) -> Option<(Self, Option<char>)> {
        match action {
            Action::Press => {
                let act = dir_pad()[&self.dir1];
                self.tick_dir_2(act)
            }
            Action::Move(d) => {
                self.dir1 += d.vector();
                dir_pad().contains_key(&self.dir1).then_some((self, None))
            }
        }
    }

    fn tick_dir_2(mut self, action: Action) -> Option<(Self, Option<char>)> {
        match action {
            Action::Press => {
                let act = dir_pad()[&self.dir2];
                self.tick_num(act)
            }
            Action::Move(d) => {
                self.dir2 += d.vector();
                dir_pad().contains_key(&self.dir2).then_some((self, None))
            }
        }
    }

    fn tick_num(mut self, action: Action) -> Option<(Self, Option<char>)> {
        match action {
            Action::Press => Some((self, Some(num_pad()[&self.num]))),
            Action::Move(d) => {
                self.num += d.vector();
                num_pad().contains_key(&self.num).then_some((self, None))
            }
        }
    }
}

fn search(
    state: State,
    make: String,
    seen: &mut HashSet<(State, String)>,
    memo: &mut HashMap<(State, String), i64>,
    level: i64,
) -> i64 {
    if make.is_empty() {
        return 0;
    }

    if let Some(mem) = memo.get(&(state, make.clone())) {
        return *mem;
    }

    if seen.contains(&(state, make.clone())) {
        return i64::MAX / 2;
    }

    seen.insert((state, make.clone()));
    let mut best = i64::MAX / 2;
    
    for act in Action::ALL {
        // let act_char = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
        //     .chars()
        //     .nth(level as usize);
        // let act = if level < 10 { match act_char.unwrap() {
        //     '^' => Action::Move(Direction::Up),
        //     '<' => Action::Move(Direction::Left),
        //     '>' => Action::Move(Direction::Right),
        //     'v' => Action::Move(Direction::Down),
        //     _ => Action::Press
        // } } else {*act};

        let Some((new, c)) = state.tick_dir_1(*act) else {
            continue;
        };

        let Some(make) = (match c {
            Some(c) => make.strip_prefix(c),
            None => Some(make.as_str()),
        }) else {
            continue;
        };
        let make = make.to_owned();

        let s = search(new, make, seen, memo, level + 1);
        best = best.min(1 + s);

        if level == 0 {
            dbg!(act, s);
        }
    }

    for act in Action::ALL {
        let act_char = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
            .chars()
            .nth(level as usize);
        let act = if level < 68 { match act_char.unwrap() {
            '^' => Action::Move(Direction::Up),
            '<' => Action::Move(Direction::Left),
            '>' => Action::Move(Direction::Right),
            'v' => Action::Move(Direction::Down),
            _ => Action::Press
        } } else {*act};

        let Some((new, c)) = state.tick_dir_1(act) else {
            continue;
        };

        let Some(make) = (match c {
            Some(c) => make.strip_prefix(c),
            None => Some(make.as_str()),
        }) else {
            continue;
        };
        let make = make.to_owned();

        let s = search(new, make, seen, memo, level + 1);
        best = best.min(1 + s);
    }
    seen.remove(&(state, make.clone()));

    memo.insert((state, make), best);
    // if level == 67 {
    //     println!("{best}");
    // }
    best
}

fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for l in input.lines() {
        let num = l.nums().nu();
        let mut seen = HashSet::default();
        let mut memo = HashMap::default();
        let len = search(State::start(), l.trim().to_owned(), &mut seen, &mut memo, 0);
        dbg!(len, num);
        dbg!(seen);
        sum += len * num;
    }
    sum
}

fn part2(input: &str) -> i64 {
    _ = input;
    todo!("Part 2")
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
