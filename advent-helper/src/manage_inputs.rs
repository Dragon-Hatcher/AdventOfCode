use anyhow::{bail, Context, Result};
use reqwest::{cookie::Jar, Url};
use std::{env, fs, path::PathBuf, sync::Arc};

use crate::printers::print_message;

fn download(url: &str) -> Result<String> {
    let url: Url = url.parse()?;

    let cookie = format!(
        "session={}",
        env::var("AOC_SESSION_ID").context("`AOC_SESSION_ID` must be set")?
    );
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

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
    let workspace_path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
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
    }

    print_message("Verified", format!("puzzle input `{relative_input_path}`"));

    Ok(())
}
