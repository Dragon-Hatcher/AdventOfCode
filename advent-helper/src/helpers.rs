use std::path::PathBuf;

pub fn get_workspace_path() -> PathBuf {
    PathBuf::from(env!("CARGO_WORKSPACE_DIR"))
}

pub fn get_manifest_path() -> PathBuf {
    get_workspace_path().join("Cargo.toml")
}

pub fn get_bin_name(year: u32, day: u32) -> String {
    format!("{year:04}-{day:02}")
}

pub fn get_bin_path(year: u32, day: u32) -> String {
    format!("solutions/{year:04}/{day:02}.rs")
}
