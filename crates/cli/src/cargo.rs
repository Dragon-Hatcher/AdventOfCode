use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

pub fn get_bin_name(year: u32, day: u32) -> String {
    format!("{year:04}{day:02}")
}

pub fn get_workspace_path() -> PathBuf {
    PathBuf::from(env!("CARGO_WORKSPACE_DIR"))
}

pub fn get_manifest_path() -> PathBuf {
    get_workspace_path().join("Cargo.toml")
}

pub fn get_bin_path(year: u32, day: u32) -> PathBuf {
    get_workspace_path().join(format!("{year:04}/{day:02}.rs"))
}

pub fn display_bin_path(year: u32, day: u32) -> String {
    let workspace_path = get_manifest_path();
    let bin_path = get_bin_path(year, day);
    bin_path
        .strip_prefix(&workspace_path)
        .unwrap_or(&bin_path)
        .display()
        .to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Binary {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Binaries {
    pub bin: Vec<Binary>,
}

impl Binaries {
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
}

pub fn get_binaries() -> Result<Binaries> {
    let manifest = fs::read_to_string(&get_manifest_path())?;
    let index = manifest.find("[[bin]]").unwrap();
    let (_, binaries) = manifest.split_at(index);
    let binaries = toml::from_str(binaries)?;
    Ok(binaries)
}

pub fn write_binaries(bins: Binaries) -> Result<()> {
    let binaries = toml::to_string(&bins)?;
    let manifest = fs::read_to_string(&get_manifest_path())?;
    let index = manifest.find("[[bin]]").unwrap();
    let (main, _) = manifest.split_at(index);
    fs::write(&get_manifest_path(), main.to_owned() + &binaries)?;

    Ok(())
}
