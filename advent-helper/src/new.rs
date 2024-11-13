use crate::{
    helpers::{get_bin_name, get_bin_path, get_workspace_path},
    manage_meta::{Metadata, Puzzle},
    options::NewOptions,
    printers::print_message,
};
use anyhow::Result;
use cargo::Binaries;
use std::{
    fs::{self},
    process,
};

mod cargo;

pub fn new_command(opts: NewOptions) -> Result<()> {
    let Puzzle { year, day } =
        Metadata::new_from_fs().resolve_selected_puzzle(opts.year, opts.day)?;

    create_bin_file(year, day)?;
    update_manifest(year, day)?;
    set_active_puzzle(year, day)?;
    open_bin_file(year, day);

    print_message(
        "Completed",
        format!("use cargo advent run -y {year} -d {day} to run"),
    );

    Ok(())
}

fn create_bin_file(year: u32, day: u32) -> Result<()> {
    let workspace_path = get_workspace_path();
    let relative_bin_path = get_bin_path(year, day);
    let bin_path = workspace_path.join(&relative_bin_path);

    if bin_path.exists() {
        print_message("Verified", format!("{relative_bin_path} already exists"));
    } else {
        const TEMPLATE: &str = include_str!("solution_template.rs");

        let rendered = TEMPLATE
            .replace("{ year }", &format!("{year:04}"))
            .replace("{ day }", &format!("{day:02}"));

        fs::create_dir_all(bin_path.parent().unwrap())?;
        fs::write(&bin_path, rendered)?;
        print_message("Created", relative_bin_path);
    }

    Ok(())
}

fn update_manifest(year: u32, day: u32) -> Result<()> {
    let bin_name = get_bin_name(year, day);

    let mut binaries = Binaries::new_from_fs()?;
    let added = binaries.ensure_has(year, day);

    if added {
        binaries.write_to_fs()?;
        print_message("Added", format!("{bin_name} to Cargo manifest"));
    } else {
        print_message("Verified", format!("{bin_name} already in Cargo manifest"));
    }

    Ok(())
}

fn open_bin_file(year: u32, day: u32) {
    let workspace_path = get_workspace_path();
    let relative_bin_path = get_bin_path(year, day);
    let bin_path = workspace_path.join(&relative_bin_path);

    _ = process::Command::new("code").arg(bin_path).status();
}

fn set_active_puzzle(year: u32, day: u32) -> Result<()> {
    let mut metadata = Metadata::new_from_fs();
    metadata.active_puzzle = Some(Puzzle { year, day });
    metadata.write_to_fs()?;

    print_message("Updated", format!("set active puzzle to {year:04}-{day:02}"));

    Ok(())
}
