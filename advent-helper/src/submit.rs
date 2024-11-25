use crate::{
    helpers::{get_cookie_jar, get_last_run_output},
    manage_meta::Metadata,
    options::SubmitOptions,
    printers::print_message,
};
use anyhow::{bail, Context, Result};
use interop::{Part, Puzzle};
use regex_macro::regex;
use reqwest::Url;
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    sync::Arc,
    thread,
    time::Duration,
};
use yansi::Paint;

pub fn submit_command(opts: SubmitOptions, confirm: bool) -> Result<()> {
    let mut meta = Metadata::new_from_fs();
    let puzzle = meta.resolve_selected_puzzle(opts.year, opts.day)?;

    meta.set_active_puzzle(puzzle)?;

    let response_type = 'get_resp: {
        let puzzle_info = meta.get_or_fetch_puzzle_info(puzzle)?;

        let next_part = match (&puzzle_info.part1_solution, &puzzle_info.part2_solution) {
            (None, None) => Part::One,
            (Some(_), None) => Part::Two,
            _ => break 'get_resp ResponseType::AlreadySolved,
        };

        let answer = opts
            .answer
            .clone()
            .or(get_last_run_output(puzzle, next_part))
            .context("No answer provided.")?;

        if !is_acceptable_answer(&answer) {
            bail!("Invalid answer {answer}");
        }

        if confirm && !confirm_submit(&answer) {
            return Ok(());
        }

        print_message("Submitting", "submitting answer");

        let response = submit_data(puzzle, next_part, answer)?;
        let response_type = analyze_response(&response);

        response_type?
    };

    match response_type {
        ResponseType::AlreadySolved => {
            println!(
                "{}",
                Paint::yellow("You've already solved both parts of this puzzle.")
            );
            meta.refetch_puzzle(puzzle, false)?;
        }
        ResponseType::TooSoon { min, sec } => {
            countdown(min, sec);
            return submit_command(opts, false);
        }
        ResponseType::Incorrect => {
            println!("{}", Paint::red("Incorrect solution."))
        }
        ResponseType::Correct => {
            println!("🎄⭐🎄 {} 🎄⭐🎄", Paint::green("Correct!"));
            meta.refetch_puzzle(puzzle, false)?;
        }
    }

    Ok(())
}

fn is_acceptable_answer(answer: &str) -> bool {
    let answer = answer.trim();
    let valid = regex!("^\\w{4, 20}$");

    answer != "<program panicked>" && valid.is_match(answer)
}

fn confirm_submit(answer: &str) -> bool {
    print!(
        "Are you sure you want to submit `{}`? (Y/n): ",
        Paint::bold(answer)
    );
    _ = stdout().flush();

    loop {
        let mut line = String::new();
        _ = stdin().read_line(&mut line);

        match line.trim() {
            "y" | "" => return true,
            "n" => return false,
            _ => continue,
        }
    }
}

fn get_submit_url(Puzzle { year, day }: Puzzle) -> Url {
    format!("https://adventofcode.com/{year}/day/{day}/answer")
        .parse()
        .unwrap()
}

fn submit_data(puzzle: Puzzle, part: Part, answer: String) -> Result<String> {
    let url = get_submit_url(puzzle);
    let jar = get_cookie_jar(&url)?;

    let mut form_data = HashMap::new();
    form_data.insert("level", part.to_num_str().to_owned());
    form_data.insert("answer", answer);

    Ok(reqwest::blocking::ClientBuilder::new()
        .cookie_provider(Arc::new(jar))
        .user_agent("https://github.com/Dragon-Hatcher/AdventOfCode danieldragonhatcher@gmail.com")
        .build()?
        .post(url)
        .form(&form_data)
        .send()?
        .text()?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResponseType {
    AlreadySolved,
    TooSoon { min: u32, sec: u32 },
    Incorrect,
    Correct,
}

fn analyze_response(response: &str) -> Result<ResponseType> {
    if response.contains("You don't seem to be solving the right level.") {
        Ok(ResponseType::AlreadySolved)
    } else if response.contains("You gave an answer too recently") {
        let to_wait_re = regex!("You have ((\\d+)m )?(\\d+)s left to wait");
        let captures = to_wait_re.captures(response).unwrap();

        let min: u32 = captures
            .get(2)
            .map(|n| n.as_str().parse().unwrap())
            .unwrap_or_default();
        let sec: u32 = captures[3].parse().unwrap();

        Ok(ResponseType::TooSoon { min, sec })
    } else if response.contains("That's not the right answer") {
        Ok(ResponseType::Incorrect)
    } else if response.contains("That's the right answer") {
        Ok(ResponseType::Correct)
    } else {
        bail!("Error accessing AOC.")
    }
}

fn countdown(mut min: u32, mut sec: u32) {
    println!();

    while min > 0 || sec > 0 {
        print!("\x1b[1A\r");

        let time = if min > 0 {
            format!("{min}m {sec}s")
        } else {
            format!("{sec}s")
        };
        println!(
            "{}",
            Paint::yellow(&format!("Too soon. Waiting {time} to submit again.       "))
        );

        if sec == 0 {
            min -= 1;
            sec = 59;
        } else {
            sec -= 1;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
