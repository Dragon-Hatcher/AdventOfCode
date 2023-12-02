use argh::FromArgs;
use std::{
    fmt::Display,
    hint,
    panic::UnwindSafe,
    time::{Duration, Instant},
};
use summary::{RunSummary, Summary};
use yansi::Paint;
use crate::{stats::Stats, summary::BenchSummary};

pub use prelude;

mod human;
mod stats;
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

    fn bench(self) -> Summary {
        let Self { parse, parts } = self;
        let mut benches = Vec::new();

        fn bench_with_input<F, I, O>(input: I, f: F) -> Stats
        where
            I: Clone,
            F: Fn(I) -> O,
        {
            const FIVE_SECS: Duration = Duration::from_secs(5);
            const THREE_SECS: Duration = Duration::from_secs(3);

            // warm up for 3 secs
            let start = Instant::now();
            while start.elapsed() < THREE_SECS {
                hint::black_box(f(input.clone()));
            }

            // now time for 5 secs, but with at least 25 samples
            let mut times = Vec::new();
            let start = Instant::now();
            while times.len() < 25 || (start.elapsed() < FIVE_SECS && times.len() < 123_456) {
                let input = input.clone();
                let start = Instant::now();
                hint::black_box(f(input));
                times.push(start.elapsed());
            }

            stats::basics(&times)
        }

        let input = (parse)();

        let stats = bench_with_input((), move |_| parse());
        benches.push(BenchSummary { name: "Parse".into(), stats });

        for (name, f) in parts {
            let stats = bench_with_input(input.clone(), &f);
            benches.push(BenchSummary { name, stats });
        }

        Summary::Bench(benches)
    }

    pub fn cli(self) {
        let Opt { bench } = argh::from_env();

        let summary = if bench {
            if cfg!(not(profile = "release")) {
                eprintln!(
                    "{}\n",
                    Paint::yellow("Note: using --bench without --release").bold()
                );
            }
            self.bench()
        } else {
            self.run()
        };

        summary.print()
    }
}

/// Run the program.
#[derive(Debug, FromArgs)]
struct Opt {
    /// whether to benchmark
    #[argh(switch)]
    bench: bool,
}
