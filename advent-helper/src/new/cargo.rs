use crate::helpers::{get_bin_path, get_bin_name, get_manifest_path};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Binary {
    name: String,
    path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Binaries {
    bin: Vec<Binary>,
}

impl Binaries {
    pub fn new_from_fs() -> Result<Self> {
        let manifest = fs::read_to_string(&get_manifest_path())?;
        let index = manifest.find("[[bin]]").unwrap();
        let (_, binaries) = manifest.split_at(index);
        let binaries = toml::from_str(binaries)?;
        Ok(binaries)
    }

    pub fn ensure_has(&mut self, year: u32, day: u32) -> bool {
        let binary = Binary {
            name: get_bin_name(year, day),
            path: get_bin_path(year, day),
        };

        let added = !self.bin.contains(&binary);
        self.bin.push(binary);
        self.bin.sort();
        self.bin.dedup();
        added
    }

    pub fn write_to_fs(&self) -> Result<()> {
        let binaries = toml::to_string(&self)?;
        let manifest = fs::read_to_string(&get_manifest_path())?;
        let index = manifest.find("[[bin]]").unwrap();
        let (main, _) = manifest.split_at(index);
        fs::write(&get_manifest_path(), main.to_owned() + &binaries)?;

        Ok(())
    }
}
