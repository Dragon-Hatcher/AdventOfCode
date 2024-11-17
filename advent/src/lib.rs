use interop::{Part, PartChoice, Puzzle};
use options::Options;
use printers::print_run;
use std::{fmt::Display, fs, panic::UnwindSafe, time::Instant};

mod options;
mod printers;

pub use prelude;

pub fn new<'a, F, I>(year: u32, day: u32, parse: F) -> Solution<'a, I>
where
    F: Fn() -> I + UnwindSafe + 'a,
{
    Solution {
        puzzle: Puzzle { year, day },
        parse: Box::new(parse),
        part1: None,
        part2: None,
    }
}

type ParseFn<'a, I> = Box<dyn Fn() -> I + 'a>;
type PartFn<'a, I> = Box<dyn Fn(I) -> Box<dyn Display + 'a> + UnwindSafe + 'a>;

pub struct Solution<'a, I> {
    puzzle: Puzzle,
    parse: ParseFn<'a, I>,
    part1: Option<PartFn<'a, I>>,
    part2: Option<PartFn<'a, I>>,
}

impl<'a, I> Solution<'a, I> {
    pub fn part1<D, F>(mut self, f: F) -> Self
    where
        D: Display + 'a,
        F: Fn(I) -> D + UnwindSafe + 'a,
    {
        self.part1 = Some(Box::new(move |i| Box::new(f(i))));
        self
    }

    pub fn part2<D, F>(mut self, f: F) -> Self
    where
        D: Display + 'a,
        F: Fn(I) -> D + UnwindSafe + 'a,
    {
        self.part2 = Some(Box::new(move |i| Box::new(f(i))));
        self
    }
}

impl<'a, I> Solution<'a, I>
where
    I: Clone + UnwindSafe,
{
    fn run(self, part_choice: PartChoice) {
        let input = (self.parse)();

        let mut which_parts = Vec::new();
        if part_choice.has_part_1() {
            which_parts.push((Part::One, "Part 1", self.part1));
        }
        if part_choice.has_part_2() {
            which_parts.push((Part::Two, "Part 2", self.part2));
        }

        for (part, name, part_fn) in which_parts {
            let Some(part_fn) = part_fn else { continue };

            let input = input.clone();

            println!();

            let start = Instant::now();
            let result = std::panic::catch_unwind(move || part_fn(input));
            let elapsed = start.elapsed();

            let output = match result {
                Ok(res) => res.to_string(),
                Err(_) => "<program panicked>".to_owned(),
            };

            print_run(name, &output, elapsed);
            save_output(self.puzzle, part, &output);
        }
    }

    pub fn cli(self) {
        let opts: Options = argh::from_env();
        self.run(opts.part);
    }
}

fn save_output(puzzle: Puzzle, part: Part, output: &str) {
    let output = output.trim();
    let path = puzzle.get_run_output_path(part);

    _ = fs::create_dir_all(path.parent().unwrap());
    _ = fs::write(&path, output);
}
