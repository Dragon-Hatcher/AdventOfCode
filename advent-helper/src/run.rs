use crate::{
    helpers::get_bin_name,
    manage_inputs::ensure_input_fetched,
    manage_meta::{set_active_puzzle, Metadata, Puzzle},
    options::{Part, RunOptions},
    printers::print_message,
};
use anyhow::Result;
use std::process;

pub fn run_command(opts: RunOptions) -> Result<()> {
    let Puzzle { year, day } =
        Metadata::new_from_fs().resolve_selected_puzzle(opts.year, opts.day)?;

    run_single_day(year, day, opts.part, &opts.args)?;

    Ok(())
}

fn run_single_day(year: u32, day: u32, parts: Part, args: &[String]) -> Result<()> {
    set_active_puzzle(year, day)?;
    ensure_input_fetched(year, day)?;

    let bin_name = get_bin_name(year, day);

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
