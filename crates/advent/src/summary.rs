use std::time::Duration;

use yansi::Paint;

pub enum Summary {
    Run(Vec<RunSummary>),
}

impl Summary {
    pub fn print(&self) {
        match self {
            Self::Run(runs) => print_run_summary(runs),
        }
    }
}

pub struct RunSummary {
    pub name: String,
    pub result: String,
    pub time: Duration,
}

fn print_run_summary(runs: &[RunSummary]) {
    for (i, run) in runs.iter().enumerate() {
        let RunSummary { name, result, time } = run;
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
