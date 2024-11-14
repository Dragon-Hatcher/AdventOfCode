use crate::{
    helpers::get_bin_name,
    manage_inputs::ensure_input_fetched,
    manage_meta::{Metadata, Puzzle},
    options::TestOptions,
    printers::print_message,
};
use anyhow::Result;
use std::process;

pub fn test_command(opts: TestOptions) -> Result<()> {
    let Puzzle { year, day } =
        Metadata::new_from_fs().resolve_selected_puzzle(opts.year, opts.day)?;

    test_single_day(year, day, &opts.args)?;

    Ok(())
}

fn test_single_day(year: u32, day: u32, args: &[String]) -> Result<()> {
    Metadata::new_from_fs().set_active_puzzle(year, day)?;
    ensure_input_fetched(year, day)?;

    let bin_name = get_bin_name(year, day);

    print_message("Testing", format!("puzzle solution {bin_name}"));

    let status = process::Command::new(env!("CARGO"))
        .args(["test", "--release", "--bin", &bin_name, "--"])
        .args(args)
        .status()?;

    process::exit(status.code().unwrap_or(1));
}
