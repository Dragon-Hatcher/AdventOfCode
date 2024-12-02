pub mod files;

use argh::FromArgValue;
use files::get_workspace_path;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf, str::FromStr};

pub const FIRST_YEAR: u32 = 2015;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Puzzle {
    pub year: u32,
    pub day: u32,
}

impl Puzzle {
    pub fn get_bin_name(self) -> String {
        format!("{:04}-{:02}", self.year, self.day)
    }

    pub fn get_bin_rel_path(self) -> String {
        format!("solutions/{:04}/{:02}.rs", self.year, self.day)
    }

    pub fn get_bin_path(self) -> PathBuf {
        let workspace = get_workspace_path();
        let rel = self.get_bin_rel_path();
        workspace.join(rel)
    }

    pub fn get_run_output_rel_path(self, part: Part) -> String {
        format!("input/{:04}/{:02}_{part}_output.txt", self.year, self.day)
    }

    pub fn get_run_output_path(self, part: Part) -> PathBuf {
        let workspace = get_workspace_path();
        let rel = self.get_run_output_rel_path(part);
        workspace.join(rel)
    }

    pub fn get_input_rel_path(self) -> String {
        format!("input/{:04}/{:02}.txt", self.year, self.day)
    }

    pub fn get_input_path(self) -> PathBuf {
        let workspace = get_workspace_path();
        let rel = self.get_input_rel_path();
        workspace.join(rel)
    }

    pub fn no_part_2(self) -> bool {
        self.day == 25
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.day)
    }
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (year, day) = s
            .split_once('-')
            .ok_or(anyhow::Error::msg("Must contain year and day."))?;
        let year = year.parse()?;
        let day = day.parse()?;
        Ok(Puzzle { year, day })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Part {
    One,
    Two,
}

impl Part {
    pub fn to_flag(&self) -> &'static str {
        match self {
            Part::One => "one",
            Part::Two => "two",
        }
    }

    pub fn to_num_str(&self) -> &'static str {
        match self {
            Part::One => "1",
            Part::Two => "2",
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One => write!(f, "one"),
            Part::Two => write!(f, "two"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum PartChoice {
    Single(Part),

    #[default]
    Both,
}

impl PartChoice {
    pub fn has_part_1(self) -> bool {
        matches!(self, Self::Both | Self::Single(Part::One))
    }

    pub fn has_part_2(self) -> bool {
        matches!(self, Self::Both | Self::Single(Part::Two))
    }

    pub fn to_flag(self) -> &'static str {
        match self {
            PartChoice::Single(p) => p.to_flag(),
            PartChoice::Both => "both",
        }
    }
}

impl FromArgValue for PartChoice {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "1" | "one" => Ok(PartChoice::Single(Part::One)),
            "2" | "two" => Ok(PartChoice::Single(Part::Two)),
            "b" | "both" => Ok(PartChoice::Both),
            _ => Err("invalid part".into()),
        }
    }
}

impl Display for PartChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PartChoice::Single(Part::One) => "part 1",
            PartChoice::Single(Part::Two) => "part 2",
            PartChoice::Both => "both parts",
        };
        write!(f, "{s}")
    }
}
