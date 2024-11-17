use crate::{
    helpers::get_last_run_output, manage_inputs::ensure_input_fetched, manage_meta::Metadata,
    new::cargo::Binaries, options::RunOptions, printers::print_message,
};
use anyhow::Result;
use interop::{Part, PartChoice, Puzzle};
use std::{
    collections::HashMap,
    fmt::Display,
    process::{self},
    sync::mpsc::{channel, Sender},
};
use yansi::Paint;

pub fn run_command(opts: RunOptions) -> Result<()> {
    let mut meta = Metadata::new_from_fs();
    let puzzle = meta.resolve_selected_puzzle(opts.year, opts.day)?;

    if opts.all {
        // TODO: Only require year.
        run_all(puzzle.year, &opts.args)?;
    } else {
        meta.set_active_puzzle(puzzle)?;
        run_single_day(puzzle, opts.part, &opts.args)?;
    }

    Ok(())
}

fn run_single_day(puzzle: Puzzle, parts: PartChoice, args: &[String]) -> Result<()> {
    ensure_input_fetched(puzzle)?;

    print_message("Running", format!("puzzle solution {puzzle} ({parts})"));

    let bin_name = puzzle.get_bin_name();
    let status = process::Command::new(env!("CARGO"))
        .args([
            "run",
            "--release",
            "--bin",
            &bin_name,
            "--",
            "--part",
            parts.to_flag(),
        ])
        .args(args)
        .status()?;

    process::exit(status.code().unwrap_or(1));
}

const PART_WIDTH: usize = 30;
const NAME_WIDTH: usize = 30;

fn run_all(year: u32, args: &[String]) -> Result<()> {
    let year_str = format!("{year:04}");

    let mut days: Vec<u32> = Binaries::new_from_fs()?
        .binaries()
        .iter()
        .filter(|b| b.name.starts_with(&year_str))
        .map(|b| b.name.split_once("-").unwrap().1.parse().unwrap())
        .collect();

    days.sort();

    let mut meta = Metadata::new_from_fs();

    let (sender, receiver) = channel();

    for &day in &days {
        let puzzle = Puzzle { year, day };
        let info = meta.get_or_fetch_puzzle_info(puzzle)?;

        let args1 = args.to_owned();
        let sender1 = sender.clone();
        let expected1 = info.part1_solution.clone();
        std::thread::spawn(move || execute_day(puzzle, Part::One, &args1, expected1, sender1));

        let args2 = args.to_owned();
        let sender2 = sender.clone();
        let expected2 = info.part2_solution.clone();
        std::thread::spawn(move || execute_day(puzzle, Part::Two, &args2, expected2, sender2));
    }

    let mut results: HashMap<(Puzzle, Part), (bool, Option<bool>, String)> = HashMap::default();

    loop {
        print_header();

        let mut all = true;

        for &day in &days {
            let puzzle = Puzzle { year, day };
            let info = meta.get_or_fetch_puzzle_info(puzzle)?;

            let results1 = results.get(&(puzzle, Part::One));
            all = all && results1.is_some();
            let part1 = part(
                results1.map(|r| r.2.clone()),
                results1.map(|r| r.1).flatten(),
            );

            let results2 = results.get(&(puzzle, Part::Two));
            all = all && (results2.is_some() || day == 25);
            let part2 = part(
                results2.map(|r| r.2.clone()),
                results2.map(|r| r.1).flatten(),
            );

            print_row(year, day, &info.name, part1, part2);
        }
        print_footer();

        if all {
            break;
        }

        print!("\x1b[{}A\r", days.len() + 4);

        let next = receiver.recv().unwrap();
        results.insert(
            (next.puzzle, next.part),
            (next.status, next.expected, next.out),
        );
    }

    Ok(())
}

struct RunInfo {
    puzzle: Puzzle,
    part: Part,
    status: bool,
    expected: Option<bool>,
    out: String,
}

fn execute_day(
    puzzle: Puzzle,
    part: Part,
    args: &[String],
    expected: Option<String>,
    sender: Sender<RunInfo>,
) {
    let mut info = RunInfo {
        puzzle,
        part,
        status: false,
        expected: None,
        out: String::new(),
    };

    let Ok(output) = process::Command::new(env!("CARGO"))
        .args([
            "run",
            "--release",
            "--bin",
            &puzzle.get_bin_name(),
            "--",
            "--part",
            part.to_flag(),
        ])
        .args(args)
        .output()
    else {
        _ = sender.send(info);
        return;
    };

    info.status = output.status.code() == Some(0);

    let Some(out) = get_last_run_output(puzzle, part) else {
        _ = sender.send(info);
        return;
    };

    info.out = out;
    info.expected = expected.map(|e| e == info.out);
    _ = sender.send(info);
}

fn part(result: Option<String>, correct: Option<bool>) -> impl Display {
    let result = result.unwrap_or_default().replace("\n", "↩");

    let trimmed_result = if result.len() <= PART_WIDTH - 11 {
        result.clone()
    } else {
        let sub: String = result.chars().take(PART_WIDTH - 11 - 1).collect();
        format!("{sub}…",)
    };

    let correct_char = match correct {
        Some(true) => Paint::green(&'✓'),
        Some(false) => Paint::red(&'✗'),
        None => Paint::new(&'?').fixed(245),
    };

    format!(
        "{:<w$} │ {}",
        if correct == Some(false) {
            Paint::red(&trimmed_result).bold()
        } else {
            Paint::new(&trimmed_result).bold()
        },
        correct_char,
        w = PART_WIDTH,
    )
}

fn print_row(year: u32, day: u32, name: &str, part1: impl Display, part2: impl Display) {
    let mut name = name.to_owned();
    if name.len() > NAME_WIDTH {
        name = name.chars().take(NAME_WIDTH - 1).collect();
        name.push('…');
    };

    println!(
        "│ {}: {:<n_width$} │ {:<width$} │ {:<width$} │",
        Paint::cyan(&format!("{year:04} / {day:02}")).bold(),
        name,
        part1,
        part2,
        n_width = NAME_WIDTH,
        width = PART_WIDTH + 4
    );
}

fn print_header() {
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
}

fn print_footer() {
    println!(
        "╰────────────{pc:─^n_width$}─┴─{pc:─^width$}─┴───┴─{pc:─^width$}─┴───╯",
        pc = "",
        n_width = NAME_WIDTH,
        width = PART_WIDTH
    );
}
