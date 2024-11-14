use anyhow::{bail, Result};
use reqwest::Url;
use std::{fs, sync::Arc};

use crate::{
    helpers::{get_cookie_jar, get_workspace_path},
    printers::print_message,
};

fn download(url: &str) -> Result<String> {
    let url: Url = url.parse()?;

    let jar = get_cookie_jar(&url)?;

    Ok(reqwest::blocking::ClientBuilder::new()
        .cookie_provider(Arc::new(jar))
        .user_agent("https://github.com/Dragon-Hatcher/AdventOfCode danieldragonhatcher@gmail.com")
        .build()?
        .get(url)
        .send()?
        .text()?)
}

fn get_input_url(year: u32, day: u32) -> String {
    format!("https://adventofcode.com/{year}/day/{day}/input")
}

fn get_input_path(year: u32, day: u32) -> String {
    format!("input/{year:04}/{day:02}.txt")
}

pub fn ensure_input_fetched(year: u32, day: u32) -> Result<()> {
    let workspace_path = get_workspace_path();
    let relative_input_path = get_input_path(year, day);
    let input_path = workspace_path.join(&relative_input_path);

    if !input_path.exists() {
        print_message("Downloading", format!("puzzle input {year:04}-{day:02}"));

        let url = get_input_url(year, day);
        let input_text = download(&url)?;

        if input_text != include_str!("input_error.txt") {
            fs::create_dir_all(input_path.parent().unwrap())?;
            fs::write(&input_path, input_text)?;
        } else {
            bail!("Can't fetch input for a future day.");
        }

        print_message("Verified", format!("puzzle input `{relative_input_path}`"));
    }

    Ok(())
}
