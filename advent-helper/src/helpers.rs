use anyhow::{Context, Result};
use interop::{Part, Puzzle};
use reqwest::{cookie::Jar, Url};
use std::{env, fs};

pub fn get_cookie_jar(url: &Url) -> Result<Jar> {
    let cookie = format!(
        "session={}",
        env::var("AOC_SESSION_ID").context("`AOC_SESSION_ID` must be set")?
    );
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, url);

    Ok(jar)
}

pub fn get_last_run_output(puzzle: Puzzle, part: Part) -> Option<String> {
    let path = puzzle.get_run_output_path(part);
    fs::read_to_string(path).ok()
}

pub fn ellipsize(str: &str, max_width: usize) -> String {
    if str.len() > max_width {
        let mut str: String = str.chars().take(max_width - 1).collect();
        str = str.trim_end().to_owned();
        str.push('…');
        str
    } else {
        str.to_owned()
    }
}
