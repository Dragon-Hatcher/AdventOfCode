use std::{fs, io::BufReader, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::helpers::get_workspace_path;

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

}
