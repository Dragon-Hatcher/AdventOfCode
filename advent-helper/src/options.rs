use argh::FromArgs;
use interop::PartChoice;

/// 🎄 Advent of Code solution management modified from rossmacarthur/advent 🎄
#[derive(Debug, FromArgs)]
pub struct Options {
    /// the action to perform
    #[argh(subcommand)]
    pub nested: SubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Run(RunOptions),
    Test(TestOptions),
    New(NewOptions),
    Submit(SubmitOptions),
    Switch(SwitchOptions),
}

/// run the solution for a specific day
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
pub struct RunOptions {
    /// the year of the solution to run
    #[argh(option, short = 'y')]
    pub year: Option<u32>,

    /// the day of the solution to run
    #[argh(option, short = 'd')]
    pub day: Option<u32>,

    /// which solution part to run
    #[argh(option, short = 'p', default = "PartChoice::Both")]
    pub part: PartChoice,

    /// which solution part to run
    #[argh(switch)]
    pub all: bool,

    #[argh(positional, greedy)]
    pub args: Vec<String>,
}

/// test the solution for a specific day
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "test")]
pub struct TestOptions {
    /// the year of the solution to test
    #[argh(option, short = 'y')]
    pub year: Option<u32>,

    /// the day of the solution to test
    #[argh(option, short = 'd')]
    pub day: Option<u32>,

    #[argh(positional, greedy)]
    pub args: Vec<String>,
}

/// create a new solution file
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "new")]
pub struct NewOptions {
    /// the year of the solution to run
    #[argh(option, short = 'y')]
    pub year: Option<u32>,

    /// the day of the solution to run
    #[argh(option, short = 'd')]
    pub day: Option<u32>,
}

/// submit a solution
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "submit")]
pub struct SubmitOptions {
    /// the year of the solution to run
    #[argh(option, short = 'y')]
    pub year: Option<u32>,

    /// the day of the solution to run
    #[argh(option, short = 'd')]
    pub day: Option<u32>,

    /// the value to submit
    #[argh(positional)]
    pub answer: Option<String>,
}

/// set the active day
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "switch")]
pub struct SwitchOptions {
    /// the year to switch to
    #[argh(option, short = 'y')]
    pub year: Option<u32>,

    /// the day to switch to
    #[argh(option, short = 'd')]
    pub day: Option<u32>,
}
