use crate::{
    manage_inputs::ensure_input_fetched, manage_meta::Metadata, options::TestOptions,
    printers::print_message,
};
use anyhow::Result;
use interop::Puzzle;
use std::process;

pub fn test_command(opts: TestOptions) -> Result<()> {
    let mut meta = Metadata::new_from_fs();
    let puzzle = meta.resolve_selected_puzzle(opts.year, opts.day)?;

    test_single_day(puzzle, &opts.args)?;
    meta.set_active_puzzle(puzzle)?;

    Ok(())
}

fn test_single_day(puzzle: Puzzle, args: &[String]) -> Result<()> {
    ensure_input_fetched(puzzle)?;

    print_message("Testing", format!("puzzle solution {puzzle}"));

    let bin_name = puzzle.get_bin_name();
    let status = process::Command::new(env!("CARGO"))
        .args(["test", "--release", "--bin", &bin_name, "--"])
        .args(args)
        .status()?;

    process::exit(status.code().unwrap_or(1));
}
