use std::{env, path::PathBuf};

pub fn get_workspace_path() -> PathBuf {
    PathBuf::from(env!("CARGO_WORKSPACE_DIR"))
}

pub fn get_manifest_path() -> PathBuf {
    get_workspace_path().join("Cargo.toml")
}
