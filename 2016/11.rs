use advent::prelude::*;
use std::hash::Hash;

fn default_input() -> &'static str {
    include_input!(2016 / 11)
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ItemSet(u8);

impl ItemSet {
    const EMPTY: ItemSet = ItemSet(0);

    fn nth(i: u8) -> ItemSet {
        ItemSet(1 << i)
    }

    fn combine(self, other: ItemSet) -> ItemSet {
        ItemSet(self.0 | other.0)
    }

    fn remove(self, other: ItemSet) -> ItemSet {
        ItemSet(self.0 & !other.0)
    }

    fn is_empty(self) -> bool {
        self == Self::EMPTY
    }

    fn is_set(self, n: u8) -> bool {
        (self.0 & Self::nth(n).0) != 0
    }

    fn one(self) -> impl Iterator<Item = ItemSet> + Clone {
        (0..u8::BITS as u8)
            .filter(move |n| self.is_set(*n))
            .map(ItemSet::nth)
    }

    fn two(self) -> impl Iterator<Item = ItemSet> {
        iproduct!(0..u8::BITS as u8, 0..u8::BITS as u8)
            .filter(move |(a, b)| a > b && self.is_set(*a) && self.is_set(*b))
            .map(|(a, b)| ItemSet::nth(a).combine(ItemSet::nth(b)))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Floor {
    microchips: ItemSet,
    generators: ItemSet,
}

impl Floor {
    const EMPTY: Floor = Floor {
        microchips: ItemSet::EMPTY,
        generators: ItemSet::EMPTY,
    };

    fn new((microchips, generators): (ItemSet, ItemSet)) -> Floor {
        Floor {
            microchips,
            generators,
        }
    }

    fn from_chips(microchips: ItemSet) -> Floor {
        Floor {
            microchips,
            generators: ItemSet::EMPTY,
        }
    }

    fn from_gens(generators: ItemSet) -> Floor {
        Floor {
            microchips: ItemSet::EMPTY,
            generators,
        }
    }

    fn is_empty(self) -> bool {
        self.microchips.is_empty() && self.generators.is_empty()
    }

    fn combine(self, other: Floor) -> Floor {
        Floor {
            microchips: self.microchips.combine(other.microchips),
            generators: self.generators.combine(other.generators),
        }
    }

    fn remove(self, other: Floor) -> Floor {
        Floor {
            microchips: self.microchips.remove(other.microchips),
            generators: self.generators.remove(other.generators),
        }
    }

    fn conflicts(self) -> bool {
        let unmatched_chips = self.microchips.0 & !self.generators.0;
        unmatched_chips != 0 && self.generators.0 != 0
    }
}

#[derive(Clone, Copy, Eq)]
struct State {
    floors: [Floor; 4],
    floor_idx: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cannonical() == other.cannonical()
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cannonical().hash(state);
    }
}

impl State {
    const FLOOR_CNT: usize = 4;

    fn is_win(&self) -> bool {
        self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }

    fn cannonical(self) -> (u64, usize) {
        // We want to filter out states that aren't identical but *will* take an equivlent number
        // of steps. To do that we use the insight that any two microchip-generator pairs with 
        // identical positions are equivlent. We don't, however, store this representation becuase 
        // it makes it annoying to generate the adjacent states.
        // 
        // The idea for this optimization comes from this reddit comment: https://www.reddit.com/r/adventofcode/comments/5hoia9/comment/db1v1ws/
        // though I did not look at this until after I had solved the puzzle.

        let mut combos = [0; 8];

        for (fi, floor) in self.floors.iter().enumerate() {
            for i in 0..8 {
                if floor.microchips.is_set(i) {
                    combos[i as usize] |= fi as u8;
                }
                if floor.generators.is_set(i) {
                    combos[i as usize] |= (fi as u8) << 3;
                }
            }
        }

        // There are more zeros in our input so we want to avoid moving them a lot. That is why
        // we do a reverse sort here. There is actually a measurable speed boost.
        combos.sort_by_key(|b| Reverse(*b));
        (u64::from_le_bytes(combos), self.floor_idx)
    }

    fn elevators(self) -> impl Iterator<Item = Floor> {
        let floor = self.floors[self.floor_idx];

        chain!(
            floor.generators.one().map(Floor::from_gens),
            floor.generators.two().map(Floor::from_gens),
            floor.microchips.one().map(Floor::from_chips),
            floor.microchips.two().map(Floor::from_chips),
            iproduct!(floor.microchips.one(), floor.generators.one()).map(Floor::new),
        )
    }

    fn apply_move(self, next_floor_idx: usize, elevator: Floor) -> Option<State> {
        let curr_floor = self.floors[self.floor_idx];
        let next_floor = self.floors[next_floor_idx];
        let new_curr_floor = curr_floor.remove(elevator);
        let new_next_floor = next_floor.combine(elevator);

        if elevator.conflicts() || new_curr_floor.conflicts() || new_next_floor.conflicts() {
            return None;
        }

        let mut new_state = self;
        new_state.floors[self.floor_idx] = new_curr_floor;
        new_state.floors[next_floor_idx] = new_next_floor;
        new_state.floor_idx = next_floor_idx;

        Some(new_state)
    }

    fn next_floors(self) -> impl Iterator<Item = usize> {
        let below = self.floor_idx.saturating_sub(1)..self.floor_idx;
        let above = (self.floor_idx + 1)..(self.floor_idx + 2).min(Self::FLOOR_CNT);
        below.chain(above)
    }

    fn next_states(self) -> impl Iterator<Item = State> {
        self.next_floors().flat_map(move |next_loc| {
            self.elevators()
                .filter_map(move |el| self.apply_move(next_loc, el))
        })
    }
}

fn parse_floor(line: &str) -> Floor {
    const NAMES: &[&str] = &[
        "promethium",
        "cobalt",
        "curium",
        "ruthenium",
        "plutonium",
        "elerium",
        "dilithium",
    ];

    let mut floor = Floor::EMPTY;

    for (i, name) in NAMES.iter().enumerate() {
        let iset = ItemSet::nth(i as u8);
        let chip = format!("{name}-compatible microchip");
        let gen = format!("{name} generator");

        if line.contains(&chip) {
            floor.microchips = floor.microchips.combine(iset);
        }
        if line.contains(&gen) {
            floor.generators = floor.generators.combine(iset);
        }
    }

    floor
}

fn parse_state(input: &str) -> State {
    let mut state = State {
        floors: [Floor::EMPTY; State::FLOOR_CNT],
        floor_idx: 0,
    };

    for (i, line) in input.lines().enumerate().take(State::FLOOR_CNT) {
        state.floors[i] = parse_floor(line);
    }

    state
}

fn solve(state: State) -> i64 {
    let mut steps = 0;
    let mut seen: HashSet<State> = HashSet::new();
    let mut edge = HashSet::new();
    edge.insert(state);
    seen.insert(state);

    loop {
        if edge.iter().any(State::is_win) {
            return steps;
        }

        edge = edge
            .iter()
            .flat_map(|state| state.next_states().filter(|s| !seen.contains(s)))
            .collect();

        seen.extend(&edge);
        steps += 1;
    }
}

fn part1(input: &str) -> i64 {
    let state = parse_state(input);
    solve(state)
}

fn part2(input: &str) -> i64 {
    let mut state = parse_state(input);
    let extra = parse_floor("elerium generator, elerium-compatible microchip, dilithium generator, dilithium-compatible microchip.");
    state.floors[0] = state.floors[0].combine(extra);
    solve(state)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "The first floor contains a promethium-compatible microchip and a cobalt-compatible microchip.
    The second floor contains a promethium generator.
    The third floor contains a cobalt generator.
    The fourth floor contains nothing relevant.";
    assert_eq!(part1(input), 11);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 33);
    assert_eq!(part2(input), 57);
}
