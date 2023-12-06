use advent::prelude::*;
use std::fmt::Debug;

fn default_input() -> &'static str {
    include_input!(2016 / 11)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ItemSet(u8);

impl ItemSet {
    const EMPTY: ItemSet = ItemSet(0);
    const FULL: ItemSet = ItemSet(0b1111111);
    const CNT: u8 = 7;
    // const FULL: ItemSet = ItemSet(0b11);

    fn nth(i: u8) -> ItemSet {
        ItemSet(1 << i)
    }

    fn combo(self, other: ItemSet) -> ItemSet {
        ItemSet(self.0 | other.0)
    }

    fn remove(self, other: ItemSet) -> ItemSet {
        ItemSet(self.0 & !other.0)
    }

    fn is_full(self) -> bool {
        self == Self::FULL
    }

    fn conflicts(chips: ItemSet, gen: ItemSet) -> bool {
        let unmatched_chips = chips.0 & !gen.0;
        unmatched_chips != 0 && gen.0 != 0
    }

    fn is_set(self, n: u8) -> bool {
        (self.0 & Self::nth(n).0) != 0
    }

    fn one(self) -> impl Iterator<Item = ItemSet> + Clone {
        (0..Self::CNT).filter(move |n| self.is_set(*n)).map(ItemSet::nth)
    }

    fn two(self) -> impl Iterator<Item = ItemSet> {
        iproduct!(0..Self::CNT, 0..Self::CNT)
            .filter(move |(a, b)| a > b && self.is_set(*a) && self.is_set(*b))
            .map(|(a, b)| ItemSet::nth(a).combo(ItemSet::nth(b)))
    }
}

impl Debug for ItemSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemSet({:07b})", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Floor {
    microchips: ItemSet,
    generators: ItemSet,
}

impl Floor {
    fn is_full(self) -> bool {
        self.microchips.is_full() && self.generators.is_full()
    }

    fn combo(self, other: Floor) -> Floor {
        Floor {
            microchips: self.microchips.combo(other.microchips),
            generators: self.generators.combo(other.generators),
        }
    }

    fn remove(self, other: Floor) -> Floor {
        Floor {
            microchips: self.microchips.remove(other.microchips),
            generators: self.generators.remove(other.generators),
        }
    }

    fn bad(self) -> bool {
        ItemSet::conflicts(self.microchips, self.generators)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    floors: [Floor; 4],
    loc: usize,
}

impl State {
    fn is_win(&self) -> bool {
        self.floors[3].is_full()
    }

    fn elevators(self, floor: usize) -> impl Iterator<Item = Floor> {
        let floor = self.floors[floor];

        let gen = floor
            .generators
            .two()
            .chain(floor.generators.one())
            .map(|g| Floor {
                microchips: ItemSet::EMPTY,
                generators: g,
            });
        let m = floor
            .microchips
            .two()
            .chain(floor.microchips.one())
            .map(|m| Floor {
                microchips: m,
                generators: ItemSet::EMPTY,
            });
        let both = iproduct!(floor.generators.one(), floor.microchips.one()).map(|(g, m)| Floor {
            microchips: m,
            generators: g,
        });

        gen.chain(m).chain(both)
    }

    fn apply_move(self, next_loc: usize, elevator: Floor) -> Option<State> {
        let curr_floor = self.floors[self.loc];
        let next_floor = self.floors[next_loc];
        let new_curr = curr_floor.remove(elevator);
        let new_next = next_floor.combo(elevator);

        if elevator.bad() || new_curr.bad() || new_next.bad() {
            return None;
        }

        let mut new_state = self;
        new_state.floors[self.loc] = new_curr;
        new_state.floors[next_loc] = new_next;
        new_state.loc = next_loc;

        Some(new_state)
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "State(")?;
        for i in (0..4).rev() {
            writeln!(
                f,
                "    {} {:?} {:?}",
                if self.loc == i { '>' } else { ' ' },
                self.floors[i].microchips,
                self.floors[i].generators
            )?;
        }
        write!(f, ")")?;

        Ok(())
    }
}

fn solve(state: State) -> i64 {
    let mut steps = 0;
    let mut seen: HashSet<State> = HashSet::new();
    let mut edge = HashSet::new();
    edge.insert(state);
    seen.insert(state);

    loop {
        let mut new_edge = HashSet::new();

        if edge.is_empty() {
            panic!();
        }

        for state in edge {
            if state.is_win() {
                return steps;
            }

            if state.loc != 0 {
                // Go down
                let next_loc = state.loc - 1;
                for elevator in state.elevators(state.loc) {
                    if let Some(new_state) = state.apply_move(next_loc, elevator) {
                        if !seen.contains(&new_state) {
                            new_edge.insert(new_state);
                        }
                    }
                }
            }

            if state.loc != 3 {
                // Go up
                let next_loc = state.loc + 1;
                for elevator in state.elevators(state.loc) {
                    if let Some(new_state) = state.apply_move(next_loc, elevator) {
                        if !seen.contains(&new_state) {
                            new_edge.insert(new_state);
                        }
                    }
                }
            }
        }

        edge = new_edge;
        seen.extend(&edge);
        steps += 1;
    }
}

fn part1(_input: &str) -> i64 {
    // dbg!(ItemSet(0b0011).two().collect_vec());
    // return 0;

    // promethium = 0
    // cobalt     = 1
    // curium     = 2
    // ruthenium  = 3
    // plutonium  = 4

    let state = State {
        floors: [
            Floor {
                microchips: ItemSet::nth(0),
                generators: ItemSet::nth(0),
            },
            Floor {
                microchips: ItemSet::EMPTY,
                generators: ItemSet::nth(1)
                    .combo(ItemSet::nth(2))
                    .combo(ItemSet::nth(3))
                    .combo(ItemSet::nth(4)),
            },
            Floor {
                microchips: ItemSet::nth(1)
                    .combo(ItemSet::nth(2))
                    .combo(ItemSet::nth(3))
                    .combo(ItemSet::nth(4)),
                generators: ItemSet::EMPTY,
            },
            Floor {
                microchips: ItemSet::EMPTY,
                generators: ItemSet::EMPTY,
            },
        ],
        loc: 0,
    };

    solve(state)
}

fn part2(_input: &str) -> i64 {
    let state = State {
        floors: [
            Floor {
                microchips: ItemSet::nth(0).combo(ItemSet::nth(5)).combo(ItemSet::nth(6)),
                generators: ItemSet::nth(0).combo(ItemSet::nth(5)).combo(ItemSet::nth(6)),
            },
            Floor {
                microchips: ItemSet::EMPTY,
                generators: ItemSet::nth(1)
                    .combo(ItemSet::nth(2))
                    .combo(ItemSet::nth(3))
                    .combo(ItemSet::nth(4)),
            },
            Floor {
                microchips: ItemSet::nth(1)
                    .combo(ItemSet::nth(2))
                    .combo(ItemSet::nth(3))
                    .combo(ItemSet::nth(4)),
                generators: ItemSet::EMPTY,
            },
            Floor {
                microchips: ItemSet::EMPTY,
                generators: ItemSet::EMPTY,
            },
        ],
        loc: 0,
    };

    solve(state)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "";
    assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}
