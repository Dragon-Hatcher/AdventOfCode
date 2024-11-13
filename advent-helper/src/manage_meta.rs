use std::{fs, io::BufReader, path::PathBuf};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::{helpers::get_workspace_path, printers::print_message};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Puzzle {
    pub year: u32,
    pub day: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Metadata {
    pub active_puzzle: Option<Puzzle>,
}

fn get_meta_path() -> PathBuf {
    const FILE_NAME: &str = "meta.json";
    get_workspace_path().join(FILE_NAME)
}

impl Metadata {
    pub fn new_from_fs() -> Self {
        match fs::File::open(get_meta_path()) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap_or_default()
            }
            Err(_) => Default::default(),
        }
    }

    pub fn write_to_fs(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(get_meta_path(), json)?;

        Ok(())
    }

    pub fn resolve_selected_puzzle(&self, year: Option<u32>, day: Option<u32>) -> Result<Puzzle> {
        match (year, day, self.active_puzzle) {
            (Some(year), Some(day), _) => Ok(Puzzle { year, day }),
            (None, Some(day), Some(active)) => Ok(Puzzle {
                year: active.year,
                day,
            }),
            (None, None, Some(active)) => Ok(active),
            _ => bail!("You must specify a puzzle."),
        }
    }
}

pub fn set_active_puzzle(year: u32, day: u32) -> Result<()> {
    let mut metadata = Metadata::new_from_fs();
    let new = Some(Puzzle { year, day });

    if metadata.active_puzzle != new {
        metadata.active_puzzle = new;
        metadata.write_to_fs()?;

        print_message(
            "Updated",
            format!("set active puzzle to {year:04}-{day:02}"),
        );
    }

    Ok(())
}
