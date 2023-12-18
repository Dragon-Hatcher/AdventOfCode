use std::{
    env,
    fmt::Display,
    fs::{self},
    io::{self, IsTerminal},
    path::PathBuf,
    process::{self, Output},
    sync::Arc,
};

use anyhow::{bail, Context, Result};
use argh::FromArgs;
use cargo::{display_bin_path, get_bin_name, get_bin_path, get_binaries, write_binaries};
use json::{AllMetadata, DayMeta};
use regex_macro::regex;
use reqwest::{cookie::Jar, Url};
use time::{OffsetDateTime, UtcOffset};
use yansi::Paint;

use crate::{
    human::Time,
    json::{RunSummary, Summary},
};

mod cargo;
mod human;
mod json;

/// 🎄 Festive Advent of Code solution management modified from rossmacarthur/advent
#[derive(Debug, FromArgs)]
#[argh(example = "cargo advent -y 2022 -d 17")]
struct Opt {
    /// the puzzle year
    #[argh(option, short = 'y')]
    year: Option<u32>,

    /// the puzzle day
    #[argh(option, short = 'd')]
    day: Option<u32>,

    /// run on all applicable days
    #[argh(switch, short = 'a')]
    all: bool,

    /// the subcommand: run
    #[argh(positional)]
    command: Command,

    #[argh(positional, greedy)]
    args: Vec<String>,
}

#[derive(Debug)]
enum Command {
    Run,
    Test,
    Bench,
    New,
}

impl argh::FromArgValue for Command {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "run" => Ok(Self::Run),
            "test" => Ok(Self::Test),
            "bench" => Ok(Self::Bench),
            "new" => Ok(Self::New),
            _ => Err("expected one of: run".into()),
        }
    }
}

const RELEASE_TIMEZONE: UtcOffset = time::macros::offset!(-5);
const MAX_DAY: u32 = 25;

fn current_year() -> u32 {
    let now = OffsetDateTime::now_utc().to_offset(RELEASE_TIMEZONE);

    use time::Month as M;
    match now.month() {
        M::November | M::December => now.year() as u32,
        _ => now.year() as u32 - 1,
    }
}

fn current_day() -> u32 {
    let now = OffsetDateTime::now_utc().to_offset(RELEASE_TIMEZONE);

    use time::Month as M;
    match now.month() {
        M::December => (now.day() as u32).clamp(0, MAX_DAY),
        _ => MAX_DAY,
    }
}

fn has_occurred(year: u32, day: u32) -> bool {
    let cur_year = current_year();
    let cur_day = current_day();
    year < cur_year || (year == cur_year && day <= cur_day)
}

fn main() -> Result<()> {
    let Opt {
        year,
        day,
        all,
        command,
        args,
    } = argh::from_env();

    let f_year = year.unwrap_or(current_year());
    let f_day = day.unwrap_or(current_day());

    match (command, all) {
        (Command::Run, false) => run(f_year, f_day, &args),
        (Command::Run, true) => run_all(year, &args),
        (Command::Test, false) => test(f_year, f_day, &args),
        (Command::Test, true) => bail!("The --all flag cannot be used with test."),
        (Command::Bench, false) => bench(f_year, f_day, &args),
        (Command::Bench, true) => bail!("The --all flag cannot be used with bench."),
        (Command::New, false) => new(f_year, f_day),
        (Command::New, true) => bail!("The --all flag cannot be used with new."),
    }
}

fn print(header: &str, message: impl Display) {
    if io::stdout().is_terminal() {
        println!("{:>12} {}", Paint::green(&header).bold(), message);
    } else {
        println!("{:>12} {}", header, message);
    }
}

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

fn ensure_input_fetched(year: u32, day: u32) -> Result<()> {
    let workspace_path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let input_path = workspace_path.join(format!("input/{year:04}/{day:02}.txt"));

    fs::create_dir_all(input_path.parent().unwrap())?;

    let input_display = input_path
        .strip_prefix(&workspace_path)
        .unwrap_or(&input_path)
        .display();

    if !input_path.exists() {
        print(
            "Downloading",
            format!("puzzle input (year: {year:04}, day: {day:02})"),
        );
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let text = download(&url)?;

        if text != include_str!("error.txt") {
            fs::write(&input_path, text)?;
        } else {
            bail!("Tried to fetch input for future day.");
        }
    }

    print("Verified", format!("puzzle input `{input_display}`"));

    Ok(())
}

fn run(year: u32, day: u32, args: &[String]) -> Result<()> {
    ensure_input_fetched(year, day)?;

    let bin_name = format!("{year:04}{day:02}");

    let status = process::Command::new(env!("CARGO"))
        .args(["run", "--release", "--bin", &bin_name])
        .args(args)
        .status()?;

    process::exit(status.code().unwrap_or(1))
}

fn get_day_meta(year: u32, day: u32) -> Result<DayMeta> {
    if !has_occurred(year, day) {
        return Ok(DayMeta {
            name: None,
            answer1: None,
            answer2: None,
        });
    }

    let url = format!("https://adventofcode.com/{year}/day/{day}");
    let text = download(&url)?;

    let title_re = regex!(r#"<h2>--- Day \d+: (.?*)---</h2>"#);
    let answer_re = regex!(r#"Your puzzle answer was <code>(.*?)</code>"#);

    let name = title_re
        .captures(&text)
        .map(|m| m[1].trim().to_owned().replace("&apos;", "'"));
    let mut answers = answer_re.captures_iter(&text);
    let answer1 = answers.next().map(|m| m[1].trim().to_owned());
    let answer2 = answers.next().map(|m| m[1].trim().to_owned());

    Ok(DayMeta {
        name,
        answer1,
        answer2,
    })
}

fn load_metadata() -> Result<AllMetadata> {
    let workspace_path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let meta_path = workspace_path.join(format!("input/metadata.json"));

    fs::create_dir_all(meta_path.parent().unwrap())?;

    let meta = if !meta_path.exists() {
        let m = AllMetadata {
            days: Default::default(),
        };
        write_metadata(&m)?;
        m
    } else {
        let text = fs::read_to_string(meta_path)?;
        serde_json::from_str(&text)?
    };

    Ok(meta)
}

fn write_metadata(meta: &AllMetadata) -> Result<()> {
    let workspace_path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let meta_path = workspace_path.join(format!("input/metadata.json"));
    fs::write(meta_path, serde_json::to_string(meta)?)?;

    Ok(())
}

fn run_all(year: Option<u32>, args: &[String]) -> Result<()> {
    let binaries = get_binaries()?;
    let year_str = year.map(|y| format!("{y:04}")).unwrap_or_default();
    let binaries = binaries
        .bin
        .iter()
        .map(|b| &b.name)
        .filter(|n| n.starts_with(&year_str));

    let mut metadata = load_metadata()?;

    let mut success = true;

    const PART_WIDTH: usize = 30;
    const NAME_WIDTH: usize = 30;

    println!(
        "╭────────────{pc:─^n_width$}─┬─{pc:─^width$}─────┬─{pc:─^width$}─────╮",
        pc = "",
        n_width = NAME_WIDTH,
        width = PART_WIDTH
    );
    println!(
        "│ {:^n_width$} │ {:^width$} │ {:^width$} │",
        Paint::new("Puzzle").bold(),
        Paint::new("Part 1").bold(),
        Paint::new("Part 2").bold(),
        n_width = NAME_WIDTH + 11,
        width = PART_WIDTH + 4
    );
    println!(
        "├────────────{pc:─^n_width$}─┼─{pc:─^width$}─┬───┼─{pc:─^width$}─┬───┤",
        pc = "",
        n_width = NAME_WIDTH,
        width = PART_WIDTH
    );

    let mut prev_year = 0;

    for bin in binaries {
        let year: u32 = bin.chars().take(4).collect::<String>().parse()?;
        let day: u32 = bin.chars().skip(4).collect::<String>().parse()?;

        if year != prev_year && prev_year != 0 {
            println!(
                "├────────────{pc:─^n_width$}─┼─{pc:─^width$}─┼───┼─{pc:─^width$}─┼───┤",
                pc = "",
                n_width = NAME_WIDTH,
                width = PART_WIDTH
            );                    
        }
        prev_year = year;

        let Output { status, stdout, .. } = process::Command::new(env!("CARGO"))
            .args([
                "run",
                "--quiet",
                "--features",
                "json",
                "--release",
                "--bin",
                &bin,
                "--",
                "--output",
                "json",
            ])
            .args(args)
            .output()?;

        let stdout = String::from_utf8_lossy(&stdout);
        let Summary::Run(runs) = serde_json::from_str(&stdout)? else { panic!("Got benchmark data?")};

        let part1 = runs.iter().find(|p| p.name == "Part 1");
        let part2 = runs.iter().find(|p| p.name == "Part 2");

        if !metadata.days.contains_key(bin) {
            let m = get_day_meta(year, day)?;
            metadata.days.insert(bin.clone(), m);
            write_metadata(&metadata)?;
        }

        let day_meta = metadata.days.get(bin).unwrap();

        fn part(p: Option<&RunSummary>, expected: &Option<String>) -> impl Display {
            match p {
                Some(p) => {
                    let result = &p.result;
                    let result = result.replace("\n", "↩");
                    let trimmed_result = if result.len() <= PART_WIDTH - 11 {
                        result.clone()
                    } else {
                        let sub: String = result.chars().take(PART_WIDTH - 11 - 1).collect();
                        format!("{sub}…",)
                    };

                    let time = format!("{}", Time::new(p.time.as_secs_f64()));
                    let width = PART_WIDTH.saturating_sub(result.chars().count() + 1);

                    let correct = expected.as_ref().map(|e| e == &p.result);
                    let correct_char = match correct {
                        Some(true) => Paint::green('✓'),
                        Some(false) => Paint::red('✗'),
                        None => Paint::fixed(245, '?'),
                    };

                    format!(
                        "{} {:>width$} │ {}",
                        if correct == Some(false) {
                            Paint::red(trimmed_result).bold()
                        } else {
                            Paint::new(trimmed_result).bold()
                        },
                        if p.time.as_secs_f64() > 1.0 {
                            Paint::yellow(format!("({})", time))
                        } else {
                            Paint::fixed(245, format!("({})", time))
                        },
                        correct_char,
                        width = width
                    )
                }
                None => format!("{:>width$} │  ", "", width = PART_WIDTH),
            }
        }

        let mut puzzle_name = day_meta
            .name
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_default()
            .to_owned();
        if puzzle_name.len() > NAME_WIDTH { 
            puzzle_name = puzzle_name.chars().take(NAME_WIDTH - 1).collect();
            puzzle_name.push('…');
        };

        println!(
            "│ {}: {:<n_width$} │ {:<width$} │ {:<width$} │",
            Paint::cyan(format!("{year:04} / {day:02}")).bold(),
            puzzle_name,
            part(part1, &day_meta.answer1),
            part(part2, &day_meta.answer2),
            n_width = NAME_WIDTH,
            width = PART_WIDTH + 4
        );

        success = success && status.success();
    }

    println!(
        "╰────────────{pc:─^n_width$}─┴─{pc:─^width$}─┴───┴─{pc:─^width$}─┴───╯",
        pc = "",
        n_width = NAME_WIDTH,
        width = PART_WIDTH
    );

    if !success {
        process::exit(1);
    }

    Ok(())
}

fn test(year: u32, day: u32, args: &[String]) -> Result<()> {
    ensure_input_fetched(year, day)?;

    let bin_name = format!("{year:04}{day:02}");

    let status = process::Command::new(env!("CARGO"))
        .args(["test", "--release", "--bin", &bin_name])
        .args(args)
        .status()?;

    process::exit(status.code().unwrap_or(1))
}

fn bench(year: u32, day: u32, args: &[String]) -> Result<()> {
    ensure_input_fetched(year, day)?;

    let bin_name = format!("{year:04}{day:02}");

    let (cargo_args, bin_args) = match args.iter().position(|a| a == "--") {
        Some(i) => (&args[..i], &args[i + 1..]),
        None => (args, &[][..]),
    };
    let status = process::Command::new(env!("CARGO"))
        .args(["run", "--release", "--bin", &bin_name])
        .args(cargo_args)
        .args(["--", "--bench"])
        .args(bin_args)
        .status()?;

    process::exit(status.code().unwrap_or(1))
}

fn new(year: u32, day: u32) -> Result<()> {
    let bin_name = get_bin_name(year, day);
    let bin_path = get_bin_path(year, day);
    let bin_display = display_bin_path(year, day);

    if bin_path.exists() {
        print("Verified", format!("{bin_display} already exists"));
    } else {
        const TEMPLATE: &str = include_str!("template.rs");

        let rendered = TEMPLATE
            .replace("{ year }", &format!("{year:04}"))
            .replace("{ day }", &format!("{day:02}"));
        fs::create_dir_all(bin_path.parent().unwrap())?;
        fs::write(&bin_path, rendered)?;
        print("Created", bin_display);
    }

    let mut bins = get_binaries()?;
    let added = bins.ensure_has(year, day);
    write_binaries(bins)?;

    if added {
        print("Added", format!("{bin_name} binary to Cargo manifest"));
    } else {
        print(
            "Verified",
            format!("{bin_name} binary already exists in Cargo manifest"),
        );
    }

    print(
        "Completed",
        format!("Use `cargo advent -y {year} -d {day} run` to run"),
    );

    _ = process::Command::new("code").arg(bin_path).status();

    Ok(())
}
