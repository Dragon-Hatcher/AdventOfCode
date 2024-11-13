use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 05)
}

#[derive(Debug, Clone)]
struct Position {
    stacks: Vec<Vec<char>>,
}

fn position_from_lines(str: &str) -> Position {
    let first_line_len = str.split('\n').nu().len();
    let columns = (first_line_len + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); columns];

    let mut lines = str.lines();
    _ = lines.next_back().unwrap();
    for line in lines {
        for (i, char) in line.chars().skip(1).step_by(4).enumerate() {
            if char != ' ' {
                stacks[i].insert(0, char);
            }
        }
    }

    Position { stacks }
}

impl Position {
    fn apply(&mut self, m: Move) {
        for _ in 0..m.amount {
            let item = self.stacks[m.from - 1].pop().unwrap();
            self.stacks[m.to - 1].push(item)
        }
    }

    fn apply_at_once(&mut self, m: Move) {
        let len = self.stacks[m.from - 1].len() - m.amount;
        let items = &self.stacks[m.from - 1][len..].to_vec();
        for i in items {
            self.stacks[m.to - 1].push(*i);
        }
        self.stacks[m.from - 1].truncate(len);
    }

    fn res(&self) -> String {
        let mut out = "".to_owned();
        for stack in self.stacks.iter() {
            out.push(stack.last().copied().unwrap_or('.'));
        }
        out
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

fn move_from_line(line: &str) -> Move {
    let (amount, from, to) = line.nums().tup();

    Move {
        from: from as usize,
        to: to as usize,
        amount: amount as usize,
    }
}

fn part1(input: &str) -> String {
    let mut sections = input.sections();
    let stacks = sections.nu();
    let moves = sections.nu();
    let mut stacks = position_from_lines(stacks);
    let moves = moves.lines().map(move_from_line);

    for m in moves {
        stacks.apply(m);
    }

    stacks.res()
}

fn part2(input: &str) -> String {
    let mut sections = input.sections();
    let stacks = sections.nu();
    let moves = sections.nu();
    let mut stacks = position_from_lines(stacks);
    let moves = moves.lines().map(move_from_line);

    for m in moves {
        stacks.apply_at_once(m);
    }

    stacks.res()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(part1(input), "CMZ");
    assert_eq!(part2(input), "MCD");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "RTGWZTHLD");
    assert_eq!(part2(input), "STHGRZZFR");
}
