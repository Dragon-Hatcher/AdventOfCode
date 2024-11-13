use argh::{FromArgValue, FromArgs};

/// 🎄 Run this Advent of Code solution 🎄
#[derive(Debug, FromArgs)]
pub struct Options {
    /// which solution part to run
    #[argh(option, short = 'p', default = "Part::Both")]
    pub part: Part,
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
