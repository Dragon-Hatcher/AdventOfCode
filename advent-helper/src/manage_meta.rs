use std::{collections::HashMap, fs, io::BufReader, path::PathBuf, sync::Arc};

use anyhow::{bail, Context, Result};
use regex_macro::regex;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{
    helpers::{get_cookie_jar, get_workspace_path},
    printers::print_message,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Puzzle {
    pub year: u32,
    pub day: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Default)]
pub struct PuzzleInfo {
    pub name: String,
    pub part1_solution: Option<String>,
    pub part2_solution: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Metadata {
    pub active_puzzle: Option<Puzzle>,
    puzzle_infos: HashMap<String, PuzzleInfo>,
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

    pub fn set_active_puzzle(&mut self, year: u32, day: u32) -> Result<()> {
        let new = Some(Puzzle { year, day });

        if self.active_puzzle != new {
            self.active_puzzle = new;
            self.write_to_fs()?;

            print_message(
                "Updated",
                format!("set active puzzle to {year:04}-{day:02}"),
            );
        }

        Ok(())
    }

    fn puzzle_info_key(year: u32, day: u32) -> String {
        format!("{year:04}-{day:02}")
    }

    pub fn get_or_fetch_puzzle_info(&mut self, year: u32, day: u32) -> Result<&PuzzleInfo> {
        let key = Self::puzzle_info_key(year, day);

        if !self.puzzle_infos.contains_key(&key) {
            print_message("Fetching", format!("puzzle info for {year:04}-{day:02}"));
            let puzzle_text = fetch_puzzle_text(year, day)?;

            if puzzle_text == include_str!("input_error.txt") {
                bail!("trying to get info on puzzle before it is released.");
            }

            let puzzle_name_re = regex!("--- Day \\d+: (.*?) ---");
            let answers_re = regex!("Your puzzle answer was <code>(\\w+)</code>");

            let name = puzzle_name_re
                .captures(&puzzle_text)
                .context("Can't find puzzle name?")?[1]
                .to_owned();

            let mut answers = answers_re
                .captures_iter(&puzzle_text)
                .map(|c| c[1].to_owned());

            let info = PuzzleInfo {
                name,
                part1_solution: answers.next(),
                part2_solution: answers.next(),
            };

            self.puzzle_infos.insert(key.clone(), info);

            self.write_to_fs()?;
        }

        Ok(self.puzzle_infos.get_mut(&key).unwrap())
    }
}

fn fetch_puzzle_text(year: u32, day: u32) -> Result<String> {
    let url: Url = format!("https://adventofcode.com/{year}/day/{day}").parse()?;
    let jar = get_cookie_jar(&url)?;

    Ok(reqwest::blocking::ClientBuilder::new()
        .cookie_provider(Arc::new(jar))
        .user_agent("https://github.com/Dragon-Hatcher/AdventOfCode danieldragonhatcher@gmail.com")
        .build()?
        .get(url)
        .send()?
        .text()?)
}
