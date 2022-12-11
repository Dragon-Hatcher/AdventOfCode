use crate::{standard_parsers::{AocParsed, IntoTup}, helpers::IterExtension};

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

///
/// --- Day 5: Supply Stacks ---
///
/// The expedition can depart as soon as the final supplies have been unloaded from
/// the ships. Supplies are stored in stacks of marked *crates*, but because the
/// needed supplies are buried under many other crates, the crates need to be rearranged.
///
/// The ship has a *giant cargo crane* capable of moving crates between stacks. To
/// ensure none of the crates get crushed or fall over, the crane operator will rearrange
/// them in a series of carefully-planned steps. After the crates are rearranged,
/// the desired crates will be at the top of each stack.
///
/// The Elves don't want to interrupt the crane operator during this delicate procedure,
/// but they forgot to ask her *which* crate will end up where, and they want to
/// be ready to unload them as soon as possible so they can embark.
///
/// They do, however, have a drawing of the starting stacks of crates *and* the rearrangement
/// procedure (your puzzle input). For example:
///
/// ```
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2
///
/// ```
///
/// In this example, there are three stacks of crates. Stack 1 contains two crates:
/// crate `Z` is on the bottom, and crate `N` is on top. Stack 2 contains three crates;
/// from bottom to top, they are crates `M`, `C`, and `D`. Finally, stack 3 contains
/// a single crate, `P`.
///
/// Then, the rearrangement procedure is given. In each step of the procedure, a
/// quantity of crates is moved from one stack to a different stack. In the first
/// step of the above rearrangement procedure, one crate is moved from stack 2 to
/// stack 1, resulting in this configuration:
///
/// ```
/// [D]        
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// ```
///
/// In the second step, three crates are moved from stack 1 to stack 3. Crates are
/// moved *one at a time*, so the first crate to be moved (`D`) ends up below the
/// second and third crates:
///
/// ```
///         [Z]
///         [N]
///     [C] [D]
///     [M] [P]
///  1   2   3
///
/// ```
///
/// Then, both crates are moved from stack 2 to stack 1. Again, because crates are
/// moved *one at a time*, crate `C` ends up below crate `M`:
///
/// ```
///         [Z]
///         [N]
/// [M]     [D]
/// [C]     [P]
///  1   2   3
///
/// ```
///
/// Finally, one crate is moved from stack 1 to stack 2:
///
/// ```
///         [Z]
///         [N]
///         [D]
/// [C] [M] [P]
///  1   2   3
///
/// ```
///
/// The Elves just need to know *which crate will end up on top of each stack*; in
/// this example, the top crates are `C` in stack 1, `M` in stack 2, and `Z` in stack
/// 3, so you should combine these together and give the Elves the message `*CMZ*`.
///
/// *After the rearrangement procedure completes, what crate ends up on top of each
/// stack?*
///
pub fn part1(input: &str) -> String {
    let mut sections = input.sections();
    let stacks = sections.nu();
    let moves = sections.nu();
    let mut stacks = position_from_lines(stacks);
    let moves = moves.non_empty().map(move_from_line);

    for m in moves {
        stacks.apply(m);
    }

    stacks.res()
}

///
/// --- Part Two ---
///
/// As you watch the crane operator expertly rearrange the crates, you notice the
/// process isn't following your prediction.
///
/// Some mud was covering the writing on the side of the crane, and you quickly wipe
/// it away. The crane isn't a CrateMover 9000 - it's a *CrateMover 9001*.
///
/// The CrateMover 9001 is notable for many new and exciting features: air conditioning,
/// leather seats, an extra cup holder, and *the ability to pick up and move multiple
/// crates at once*.
///
/// Again considering the example above, the crates begin in the same configuration:
///
/// ```
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// ```
///
/// Moving a single crate from stack 2 to stack 1 behaves the same as before:
///
/// ```
/// [D]        
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// ```
///
/// However, the action of moving three crates from stack 1 to stack 3 means that
/// those three moved crates *stay in the same order*, resulting in this new configuration:
///
/// ```
///         [D]
///         [N]
///     [C] [Z]
///     [M] [P]
///  1   2   3
///
/// ```
///
/// Next, as both crates are moved from stack 2 to stack 1, they *retain their order*
/// as well:
///
/// ```
///         [D]
///         [N]
/// [C]     [Z]
/// [M]     [P]
///  1   2   3
///
/// ```
///
/// Finally, a single crate is still moved from stack 1 to stack 2, but now it's
/// crate `C` that gets moved:
///
/// ```
///         [D]
///         [N]
///         [Z]
/// [M] [C] [P]
///  1   2   3
///
/// ```
///
/// In this example, the CrateMover 9001 has put the crates in a totally different
/// order: `*MCD*`.
///
/// Before the rearrangement process finishes, update your simulation so that the
/// Elves know where they should stand to be ready to unload the final supplies.
/// *After the rearrangement procedure completes, what crate ends up on top of each
/// stack?*
///
pub fn part2(input: &str) -> String {
    let mut sections = input.sections();
    let stacks = sections.nu();
    let moves = sections.nu();
    let mut stacks = position_from_lines(stacks);
    let moves = moves.non_empty().map(move_from_line);

    for m in moves {
        stacks.apply_at_once(m);
    }

    stacks.res()
}

const PART1_EX_ANSWER: &str = "CMZ";
const PART1_ANSWER: &str = "RTGWZTHLD";
const PART2_EX_ANSWER: &str = "MCD";
const PART2_ANSWER: &str = "STHGRZZFR";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
