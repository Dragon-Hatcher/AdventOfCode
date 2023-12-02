pub use prelude;
use std::{fmt::Display, panic::UnwindSafe, time::Instant};
use summary::{RunSummary, Summary};

mod summary;

pub fn new<'a, F, I>(parse: F) -> Builder<'a, I>
where
    F: Fn() -> I + UnwindSafe + 'a,
{
    Builder {
        parse: Box::new(parse),
        parts: Vec::new(),
    }
}

type FnParse<'a, I> = Box<dyn Fn() -> I + 'a>;
type FnPart<'a, I> = Box<dyn Fn(I) -> Box<dyn Display + 'a> + UnwindSafe + 'a>;

pub struct Builder<'a, I> {
    parse: FnParse<'a, I>,
    parts: Vec<(String, FnPart<'a, I>)>,
}

impl<'a, I> Builder<'a, I>
where
    I: Clone + UnwindSafe,
{
    pub fn part<F, R>(self, f: F) -> Self
    where
        R: Display + 'a,
        F: Fn(I) -> R + UnwindSafe + 'a,
    {
        let parse = self.parse;
        let mut parts = self.parts;
        parts.push((
            format!("Part {}", parts.len() + 1),
            Box::new(move |i| Box::new(f(i))),
        ));
        Self { parse, parts }
    }

    pub fn build(self) -> Solution<'a, I> {
        Solution {
            parse: self.parse,
            parts: self.parts,
        }
    }
}

pub struct Solution<'a, I> {
    parse: FnParse<'a, I>,
    parts: Vec<(String, FnPart<'a, I>)>,
}

impl<'a, I> Solution<'a, I>
where
    I: Clone + UnwindSafe,
{
    fn run(self) -> Summary {
        let Self { parse, parts } = self;
        let mut runs = Vec::new();

        let input = (parse)();
        for (name, f) in parts {
            let input = input.clone();

            let (result, time) = {
                let start = Instant::now();
                let result = std::panic::catch_unwind(move || f(input));
                let elapsed = start.elapsed();
                let result = match result {
                    Ok(result) => result.to_string(),
                    Err(_) => "🚨👻🚨".to_owned(),
                };
                (result, elapsed)
            };

            runs.push(RunSummary { name, result, time });
        }

        Summary::Run(runs)
    }

    pub fn cli(self) {
        self.run().print()
    }
}
