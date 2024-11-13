use options::{Options, Part};
use std::{fmt::Display, panic::UnwindSafe, time::Instant};

mod options;

pub use prelude;

pub fn new<'a, F, I>(parse: F) -> Solution<'a, I>
where
    F: Fn() -> I + UnwindSafe + 'a,
{
    Solution {
        parse: Box::new(parse),
        part1: None,
        part2: None,
    }
}

type ParseFn<'a, I> = Box<dyn Fn() -> I + 'a>;
type PartFn<'a, I> = Box<dyn Fn(I) -> Box<dyn Display + 'a> + UnwindSafe + 'a>;

pub struct Solution<'a, I> {
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
    fn run(self, parts: Part) {
        let input = (self.parse)();

        let mut which_parts = Vec::new();
        if matches!(parts, Part::One | Part::Both) {
            which_parts.push(self.part1);
        }
        if matches!(parts, Part::Two | Part::Both) {
            which_parts.push(self.part2);
        }

        for part in which_parts.into_iter().flatten() {
            let input = input.clone();

            let start = Instant::now();
            let result = std::panic::catch_unwind(move || part(input));
            let elapsed = start.elapsed();

            let str = match result {
                Ok(res) => res.to_string(),
                Err(_) => "<program panicked>".to_owned(),
            };

            println!("{elapsed:?}");
            println!("{str}");
        }
    }

    pub fn cli(self) {
        let opts: Options = argh::from_env();
        self.run(opts.part);
    }
}
