use crate::{manage_meta::Metadata, options::NewOptions, printers::print_message};
use anyhow::Result;
use cargo::Binaries;
use interop::Puzzle;
use std::{
    fs::{self},
    process,
};

pub mod cargo;

pub fn new_command(opts: NewOptions) -> Result<()> {
    let mut meta = Metadata::new_from_fs();
    let puzzle = meta.resolve_selected_puzzle(opts.year, opts.day)?;

    create_bin_file(puzzle)?;
    update_manifest(puzzle)?;
    meta.set_active_puzzle(puzzle)?;
    open_bin_file(puzzle);

    print_message(
        "Completed",
        format!(
            "use cargo advent run -y {} -d {} to run",
            puzzle.year, puzzle.day
        ),
    );

    Ok(())
}

fn create_bin_file(puzzle: Puzzle) -> Result<()> {
    let bin_rel_path = puzzle.get_bin_rel_path();
    let bin_path = puzzle.get_bin_path();

    if bin_path.exists() {
        print_message("Verified", format!("{} already exists", bin_rel_path));
    } else {
        const TEMPLATE: &str = include_str!("solution_template.rs");

        let rendered = TEMPLATE
            .replace("{ year }", &format!("{:04}", puzzle.year))
            .replace("{ day }", &format!("{:02}", puzzle.day));

        fs::create_dir_all(bin_path.parent().unwrap())?;
        fs::write(&bin_path, rendered)?;
        print_message("Created", bin_rel_path);
    }

    Ok(())
}

fn update_manifest(puzzle: Puzzle) -> Result<()> {
    let bin_name = puzzle.get_bin_name();

    let mut binaries = Binaries::new_from_fs()?;
    let added = binaries.ensure_has(puzzle);

    if added {
        binaries.write_to_fs()?;
        print_message("Added", format!("{bin_name} to Cargo manifest"));
    } else {
        print_message("Verified", format!("{bin_name} already in Cargo manifest"));
    }

    Ok(())
}

fn open_bin_file(puzzle: Puzzle) {
    _ = process::Command::new("code")
        .arg(puzzle.get_bin_path())
        .status();
}
