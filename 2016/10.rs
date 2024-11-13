use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 10)
}

#[derive(Debug, Clone, Copy)]
enum Destination {
    Bot(i64),
    Output(i64),
}

fn parse_destination(dest: &str) -> Destination {
    let idx = dest.nums().nu();
    if dest.starts_with("bot") {
        Destination::Bot(idx)
    } else {
        Destination::Output(idx)
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Const {
        value: i64,
        dest: Destination,
    },
    Transfer {
        source: i64,
        low_dest: Destination,
        high_dest: Destination,
    },
}

fn parse_instruction(line: &str) -> Instruction {
    if line.starts_with("value") {
        let (value, bot) = line.nums().tup();
        Instruction::Const {
            value,
            dest: Destination::Bot(bot),
        }
    } else {
        let (source, gives) = line.split_once(" gives low to ").unwrap();
        let (low, high) = gives.split_once(" and high to ").unwrap();
        let source = source.nums().nu();
        let low = parse_destination(low);
        let high = parse_destination(high);
        Instruction::Transfer {
            source,
            low_dest: low,
            high_dest: high,
        }
    }
}

#[derive(Debug, Default)]
struct Bot(HashSet<i64>);

impl Bot {
    fn get_low_high(&self) -> Option<(i64, i64)> {
        if self.0.len() != 2 {
            None
        } else {
            let (&a, &b) = self.0.iter().tup();
            let low = a.min(b);
            let high = a.max(b);
            Some((low, high))
        }
    }

    fn add(&mut self, val: i64) {
        self.0.insert(val);
    }
}

struct Factory {
    bots: HashMap<i64, Bot>,
    outputs: HashMap<i64, i64>,
    instructions: VecDeque<Instruction>,
}

impl Factory {
    fn bot(&mut self, id: i64) -> &mut Bot {
        self.bots.entry(id).or_default()
    }

    fn send(&mut self, to: Destination, value: i64) {
        match to {
            Destination::Bot(id) => self.bot(id).add(value),
            Destination::Output(id) => {
                self.outputs.insert(id, value);
            }
        }
    }

    fn finish(&mut self) {
        while let Some(i) = self.instructions.pop_front() {
            let completed = match i {
                Instruction::Const { value, dest } => {
                    self.send(dest, value);
                    true
                }
                Instruction::Transfer {
                    source,
                    low_dest,
                    high_dest,
                } => {
                    if let Some((low, high)) = self.bot(source).get_low_high() {
                        self.send(low_dest, low);
                        self.send(high_dest, high);
                        true
                    } else {
                        false
                    }
                }
            };

            if !completed {
                self.instructions.push_back(i);
            }
        }
    }
}

fn parse_factory(factory: &str) -> Factory {
    let instructions = factory.lines().map(parse_instruction).collect();
    Factory {
        bots: Default::default(),
        outputs: Default::default(),
        instructions,
    }
}

fn part1(input: &str) -> i64 {
    let mut factory = parse_factory(input);
    factory.finish();

    *factory
        .bots
        .iter()
        .find(|(_, bot)| bot.get_low_high() == Some((17, 61)))
        .unwrap()
        .0
}

fn part2(input: &str) -> i64 {
    let mut factory = parse_factory(input);
    factory.finish();
    
    factory.outputs[&0] * factory.outputs[&1] * factory.outputs[&2]
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 181);
    assert_eq!(part2(input), 12567);
}
