use crate::{helpers::get_cookie_jar, printers::print_message};
use anyhow::{bail, Result};
use interop::Puzzle;
use reqwest::Url;
use std::{fs, sync::Arc};

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

fn get_input_url(Puzzle { year, day }: Puzzle) -> String {
    format!("https://adventofcode.com/{year}/day/{day}/input")
}

pub fn ensure_input_fetched(puzzle: Puzzle) -> Result<()> {
    let input_path = puzzle.get_input_path();

    if !input_path.exists() {
        print_message("Downloading", format!("puzzle input {puzzle}"));

        let url = get_input_url(puzzle);
        let input_text = download(&url)?;

        if input_text != include_str!("input_error.txt") {
            fs::create_dir_all(input_path.parent().unwrap())?;
            fs::write(&input_path, input_text)?;
        } else {
            bail!("Can't fetch input for a future day.");
        }

        print_message("Written", format!("puzzle input {puzzle}"));
    }

    Ok(())
}
