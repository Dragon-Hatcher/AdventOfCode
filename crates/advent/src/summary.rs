use crate::{human, stats::Stats};
use std::time::Duration;
use yansi::Paint;

#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub enum Summary {
    Run(Vec<RunSummary>),
    Bench(Vec<BenchSummary>),
}

impl Summary {
    pub fn print(&self) {
        match self {
            Self::Run(runs) => print_run_summary(runs),
            Self::Bench(benches) => print_bench_summary(benches),
        }
    }

    #[cfg(feature = "json")]
    pub fn print_json(&self) -> serde_json::Result<()> {
        serde_json::to_writer(std::io::BufWriter::new(std::io::stdout()), self)
    }
}

#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct RunSummary {
    pub name: String,
    pub result: String,
    pub time: Duration,
}

fn print_run_summary(parts: &[RunSummary]) {
    for (i, part) in parts.iter().enumerate() {
        let RunSummary { name, result, time } = part;
        if i != 0 {
            println!();
        }
        let width = 46_usize.saturating_sub(name.chars().count() + 2);
        println!(
            "{}: {:>width$}\n{}",
            Paint::cyan(&name).bold(),
            Paint::fixed(245, format!("({time:?})")),
            Paint::new(result).bold(),
            width = width
        )
    }
}

#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct BenchSummary {
    pub name: String,
    pub stats: Stats,
}

fn print_bench_summary(parts: &[BenchSummary]) {
    for (i, part) in parts.iter().enumerate() {
        let BenchSummary { name, stats } = part;
        if i != 0 {
            println!();
        }
        println!(
            "{}{:>width$}",
            Paint::new(name).bold(),
            Paint::fixed(245, &human::Samples::new(stats.samples)),
            width = 46 - name.chars().count(),
        );
        let mean = human::Time::new(stats.mean.as_secs_f64());
        let std_dev = human::Time::with_scale(stats.std_dev.as_secs_f64(), mean.scale());
        let min = human::Time::with_scale(stats.min.as_secs_f64(), mean.scale());
        let max = human::Time::with_scale(stats.max.as_secs_f64(), mean.scale());
        println!(
            "  Time ({} ± {}):        {:>9} ± {:>8}",
            Paint::green("mean").bold(),
            Paint::green("σ"),
            Paint::green(&mean).bold(),
            Paint::green(&std_dev),
        );
        println!(
            "  Range ({} … {}):      {:>9} … {:>8}",
            Paint::cyan("min"),
            Paint::magenta("max"),
            Paint::cyan(&min),
            Paint::magenta(&max),
        );
    }
}
