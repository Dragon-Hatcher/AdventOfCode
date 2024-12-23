use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 25)
}

#[derive(Debug, Clone, Copy)]
struct Action {
    write: bool,
    delta: i64,
    new_state: char,
}

#[derive(Debug, Clone, Copy)]
struct State {
    zero_action: Action,
    one_action: Action,
}

fn parse_action(w: &str, m: &str, s: &str) -> Action {
    Action {
        write: w.nums().nu() == 1,
        delta: if m.contains("left") { -1 } else { 1 },
        new_state: s.strip_suffix('.').unwrap().chars().nbu(),
    }
}

fn parse_state(lines: &str) -> (char, State) {
    let (on_state, actions) = lines.split_once(':').unwrap();

    let char = on_state.chars().nbu();

    let (_, w0, m0, s0, _, w1, m1, s1) = actions.trim().lines().tup();
    let zero_action = parse_action(w0, m0, s0);
    let one_action = parse_action(w1, m1, s1);

    (
        char,
        State {
            zero_action,
            one_action,
        },
    )
}

fn part1(input: &str) -> i64 {
    let (start, states) = input.split_once("\n\n").unwrap();

    let (start, checksum) = start.lines().tup();
    let mut state_char = start.strip_suffix('.').unwrap().chars().nbu();
    let mut pos = 0;
    let steps = checksum.nums().nu();

    let states: HashMap<char, State> = states.sections().map(parse_state).collect();

    let mut active = HashSet::default();

    for _ in 0..steps {
        let state = states[&state_char];
        let action = if active.contains(&pos) {
            state.one_action
        } else {
            state.zero_action
        };

        if action.write {
            active.insert(pos);
        } else {
            active.remove(&pos);
        }
        pos += action.delta;
        state_char = action.new_state;
    }

    active.len() as i64
}

fn main() {
    advent::new(2017, 25, default_input).part1(part1).cli();
}

#[test]
fn example() {
    let input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";
    assert_eq!(part1(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 2725);
}
