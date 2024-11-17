use crate::{
    helpers::{ellipsize, get_last_run_output},
    manage_inputs::ensure_input_fetched,
    manage_meta::Metadata,
    new::cargo::Binaries,
    options::RunOptions,
    printers::print_message,
};
use anyhow::Result;
use interop::{Part, PartChoice, Puzzle};
use std::{
    collections::HashMap,
    process::{self},
    sync::mpsc::{channel, Sender},
};
use yansi::Paint;

pub fn run_command(opts: RunOptions) -> Result<()> {
    let mut meta = Metadata::new_from_fs();

    if opts.all {
        run_all(opts.year, &opts.args)?;
    } else {
        let puzzle = meta.resolve_selected_puzzle(opts.year, opts.day)?;
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

fn run_all(year: Option<u32>, args: &[String]) -> Result<()> {
    let mut puzzles: Vec<Puzzle> = Binaries::new_from_fs()?
        .binaries()
        .iter()
        .map(|b| b.name.parse::<Puzzle>().unwrap())
        .filter(|p| year.map(|y| y == p.year).unwrap_or(true))
        .collect();

    puzzles.sort();

    let mut meta = Metadata::new_from_fs();

    let (sender, receiver) = channel();

    for &puzzle in &puzzles {
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

    let mut results: HashMap<(Puzzle, Part), RunSummary> = HashMap::default();

    loop {
        print_header();

        let mut all = true;

        let mut last_year = None;
        let mut row_count = 0;
        for &puzzle in &puzzles {
            let info = meta.get_or_fetch_puzzle_info(puzzle)?;

            if last_year.is_some() && Some(puzzle.year) != last_year {
                print_separator();
                row_count += 1;
            }
            last_year = Some(puzzle.year);
            row_count += 1;

            let results1 = results.get(&(puzzle, Part::One));
            let part1 = draw_part(results1);
            all = all && results1.is_some();

            let results2 = results.get(&(puzzle, Part::Two));
            let part2 = draw_part(results2);
            all = all && (results2.is_some() || !puzzle.could_have_part_2());

            print_row(puzzle.year, puzzle.day, &info.name, &part1, &part2);
        }
        print_footer();

        if all {
            break;
        }

        print!("\x1b[{}A\r", row_count + 4);

        let (puzzle, part, summary) = receiver.recv().unwrap();
        results.insert((puzzle, part), summary);
    }

    Ok(())
}

enum RunSummary {
    FailedToRun,
    Ran {
        output: String,
        correct_output: Option<bool>,
    },
}

fn execute_day(
    puzzle: Puzzle,
    part: Part,
    args: &[String],
    expected: Option<String>,
    sender: Sender<(Puzzle, Part, RunSummary)>,
) {
    let send = |s: RunSummary| sender.send((puzzle, part, s));

    if process::Command::new(env!("CARGO"))
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
        .is_err()
    {
        _ = send(RunSummary::FailedToRun);
        return;
    }

    let Some(output) = get_last_run_output(puzzle, part) else {
        _ = send(RunSummary::FailedToRun);
        return;
    };

    let correct_output = expected.map(|e| e == output);
    _ = send(RunSummary::Ran {
        output,
        correct_output,
    });
}

fn draw_part(run: Option<&RunSummary>) -> String {
    let Some(RunSummary::Ran {
        output,
        correct_output,
    }) = run
    else {
        return format!("{:>width$} │  ", "", width = PART_WIDTH);
    };

    let output = output.replace("\n", "↩");
    let trimmed_output = ellipsize(&output, PART_WIDTH - 11);

    let correct_char = match correct_output {
        Some(true) => Paint::green(&'✓'),
        Some(false) => Paint::red(&'✗'),
        None => Paint::new(&'?').fixed(245),
    };

    format!(
        "{:<w$} │ {}",
        if correct_output == &Some(false) {
            Paint::red(&trimmed_output).bold()
        } else {
            Paint::new(&trimmed_output).bold()
        },
        correct_char,
        w = PART_WIDTH,
    )
}

fn print_row(year: u32, day: u32, name: &str, part1: &str, part2: &str) {
    println!(
        "│ {}: {:<n_width$} │ {:<width$} │ {:<width$} │",
        Paint::cyan(&format!("{year:04} / {day:02}")).bold(),
        ellipsize(name, NAME_WIDTH),
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

fn print_separator() {
    println!(
        "├────────────{pc:─^n_width$}─┼─{pc:─^width$}─┼───┼─{pc:─^width$}─┼───┤",
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
