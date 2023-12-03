use std::{
    env,
    fmt::Display,
    fs,
    io::{self, IsTerminal},
    path::PathBuf,
    process,
    sync::Arc,
};

use anyhow::{Context, Result};
use argh::FromArgs;
use reqwest::{cookie::Jar, Url};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, UtcOffset};
use yansi::Paint;

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

fn main() -> Result<()> {
    let Opt {
        year,
        day,
        command,
        args,
    } = argh::from_env();

    let year = year.unwrap_or(current_year());
    let day = day.unwrap_or(current_day());

    match command {
        Command::Run => run(year, day, &args),
        Command::Test => test(year, day, &args),
        Command::Bench => bench(year, day, &args),
        Command::New => new(year, day),
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
        fs::write(&input_path, text)?;
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
    let bin_name = format!("{year:04}{day:02}");

    let workspace_path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let manifest_path = workspace_path.join("Cargo.toml");
    let bin_path = workspace_path.join(format!("{year:04}/{day:02}.rs"));

    const TEMPLATE: &str = include_str!("template.rs");
    let bin_display = bin_path
        .strip_prefix(&workspace_path)
        .unwrap_or(&bin_path)
        .display();

    if bin_path.exists() {
        print("Verified", format!("{bin_display} already exists"));
    } else {
        let rendered = TEMPLATE
            .replace("{ year }", &format!("{year:04}"))
            .replace("{ day }", &format!("{day:02}"));
        fs::create_dir_all(bin_path.parent().unwrap())?;
        fs::write(&bin_path, rendered)?;
        print("Created", bin_display);
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
    struct Binary {
        name: String,
        path: PathBuf,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct Binaries {
        bin: Vec<Binary>,
    }

    let manifest = fs::read_to_string(&manifest_path)?;
    let index = manifest.find("[[bin]]").unwrap();
    let (main, binaries) = manifest.split_at(index);
    let mut bins: Binaries = toml::from_str(binaries)?;
    let to_add = Binary {
        name: bin_name.clone(),
        path: bin_path.strip_prefix(&workspace_path)?.to_owned(),
    };
    let added = !bins.bin.contains(&to_add);
    bins.bin.push(to_add);
    bins.bin.sort();
    bins.bin.dedup();
    let binaries = toml::to_string(&bins)?;
    fs::write(&manifest_path, main.to_owned() + &binaries)?;
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
