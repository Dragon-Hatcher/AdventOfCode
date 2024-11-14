use anyhow::{Context, Result};
use reqwest::{cookie::Jar, Url};
use std::{env, path::PathBuf};

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

pub fn get_run_output_path(year: u32, day: u32, part: u32) -> String {
    format!("input/{year:04}/{day:02}_{part}_output.txt")
}

pub fn get_cookie_jar(url: &Url) -> Result<Jar> {
    let cookie = format!(
        "session={}",
        env::var("AOC_SESSION_ID").context("`AOC_SESSION_ID` must be set")?
    );
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, url);

    Ok(jar)
}
