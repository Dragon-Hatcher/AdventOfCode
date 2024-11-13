use crate::{
    manage_inputs::ensure_input_fetched,
    options::{Part, RunOptions},
    printers::print_message,
};
use anyhow::Result;
use std::process;

pub fn run_command(opts: RunOptions) -> Result<()> {
    let year = opts.year.unwrap_or(2015);
    let day = opts.day.unwrap_or(1);

    run_single_day(year, day, opts.part, &opts.args)?;

    Ok(())
}

fn get_binary_name(year: u32, day: u32) -> String {
    format!("{year:04}-{day:02}")
}

fn run_single_day(year: u32, day: u32, parts: Part, args: &[String]) -> Result<()> {
    ensure_input_fetched(year, day)?;

    let bin_name = get_binary_name(year, day);

    let (part_msg, part_flag) = match parts {
        Part::One => ("part 1", "one"),
        Part::Two => ("part 2", "two"),
        Part::Both => ("both parts", "both"),
    };
    print_message(
        "Running",
        format!("puzzle solution {bin_name} ({part_msg})"),
    );

    let status = process::Command::new(env!("CARGO"))
        .args([
            "run",
            "--release",
            "--bin",
            &bin_name,
            "--",
            "--part",
            part_flag,
        ])
        .args(args)
        .status()?;

    process::exit(status.code().unwrap_or(1));
}
