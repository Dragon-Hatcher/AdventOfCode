use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 20)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum ModType<'a> {
    Broadcast,
    Empty,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Pulse>),
}

impl<'a> ModType<'a> {
    fn pulse(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        match self {
            ModType::Broadcast => Some(pulse),
            ModType::Empty => None,
            ModType::FlipFlop(on) => {
                if pulse == Pulse::Low {
                    *on = !*on;
                    Some(if *on { Pulse::High } else { Pulse::Low })
                } else {
                    None
                }
            }
            ModType::Conjunction(mem) => {
                *mem.get_mut(from).unwrap() = pulse;
                Some(if mem.values().all(|&p| p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
        }
    }

    fn is_conj(&self) -> bool {
        matches!(self, ModType::Conjunction(_))
    }
}

#[derive(Debug)]
struct Module<'a> {
    ty: ModType<'a>,
    outputs: Vec<&'a str>,
}

fn parse_mod(m: &str) -> (&str, Module<'_>) {
    let (name, outputs) = m.split_once(" -> ").unwrap();
    let outputs = outputs.split(", ").collect_vec();

    if let Some(name) = name.strip_prefix('%') {
        (
            name,
            Module {
                ty: ModType::FlipFlop(false),
                outputs,
            },
        )
    } else if let Some(name) = name.strip_prefix('&') {
        (
            name,
            Module {
                ty: ModType::Conjunction(HashMap::new()),
                outputs,
            },
        )
    } else {
        (
            name,
            Module {
                ty: ModType::Broadcast,
                outputs,
            },
        )
    }
}

fn part1(input: &str) -> i64 {
    let mut mods: HashMap<_, _> = input.lines().map(parse_mod).collect();

    for (name, m) in input.lines().map(parse_mod) {
        for out in m.outputs {
            if let ModType::Conjunction(mem) = &mut mods
                .entry(out)
                .or_insert_with(|| Module {
                    ty: ModType::Empty,
                    outputs: Vec::new(),
                })
                .ty
            {
                mem.insert(name, Pulse::Low);
            }
        }
    }

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(("broadcaster", Pulse::Low, "button"));

        while let Some((to, pulse, from)) = pulses.pop_front() {
            match pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            }

            let m = mods.get_mut(to).unwrap();
            if let Some(new_pulse) = m.ty.pulse(pulse, from) {
                for out in &m.outputs {
                    pulses.push_back((out, new_pulse, to));
                }
            }
        }
    }

    low * high
}

fn part2(input: &str) -> i64 {
    let mods: HashMap<_, _> = input.lines().map(parse_mod).collect();

    mods.get("broadcaster")
        .unwrap()
        .outputs
        .iter()
        .map(|&sec| {
            let mut cur = sec;
            let mut tot = 0;
            let mut pow = 1;

            loop {
                let outputs = &mods.get(cur).unwrap().outputs;
                let is_one = outputs.iter().any(|o| mods.get(o).unwrap().ty.is_conj());
                let next = outputs.iter().find(|&o| !mods.get(o).unwrap().ty.is_conj());

                if is_one { tot += pow; }

                match next {
                    Some(next) => cur = next,
                    None => break tot,
                }

                pow *= 2;
            }
        })
        .reduce(lcm)
        .unwrap_or_default()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    assert_eq!(part1(input), 32000000);
    let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    assert_eq!(part1(input), 11687500);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 818723272);
    assert_eq!(part2(input), 243902373381257);
}
