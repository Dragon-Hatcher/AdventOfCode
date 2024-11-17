use argh::FromArgs;
use interop::PartChoice;

/// 🎄 Run this Advent of Code solution 🎄
#[derive(Debug, FromArgs)]
pub struct Options {
    /// which solution part to run
    #[argh(option, short = 'p')]
    pub part: PartChoice,
}
