use argh::{FromArgValue, FromArgs};

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
    New(NewOptions),
    Submit(SubmitOptions),
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
    #[argh(option, short = 'p', default = "Part::Both")]
    pub part: Part,

    #[argh(positional, greedy)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
    Both,
}

impl FromArgValue for Part {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "1" | "one" => Ok(Part::One),
            "2" | "two" => Ok(Part::Two),
            "b" | "both" => Ok(Part::Both),
            _ => Err("invalid part".into()),
        }
    }
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
