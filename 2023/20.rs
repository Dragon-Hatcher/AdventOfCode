use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 20)
}

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn toggle(&mut self) {
        *self = match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        };
    }
}

#[derive(Debug, Clone)]
enum ModType<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Pulse>),
}

impl<'a> ModType<'a> {
    fn pulse(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        match (self, pulse) {
            (ModType::Broadcast, _) => Some(pulse),
            (ModType::FlipFlop(_), Pulse::High) => None,
            (ModType::FlipFlop(on), Pulse::Low) => {
                *on = !*on;
                Some(if *on { Pulse::High } else { Pulse::Low })
            }
            (ModType::Conjunction(mem), _) => {
                mem.entry(from).or_insert(Pulse::Low).toggle();
            },
        }
    }
}

struct Module<'a> {
    outputs: Vec<&'a str>,
}

fn part1(input: &str) -> i64 {
    todo!("Part 1")
}

fn part2(input: &str) -> i64 {
    todo!("Part 2")
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
