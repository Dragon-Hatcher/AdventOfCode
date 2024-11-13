use crate::{
    helpers::{get_bin_path, get_binary_name, get_workspace_path},
    options::NewOptions,
    printers::print_message,
};
use anyhow::Result;
use cargo::Binaries;
use std::{fs, process};

mod cargo;

pub fn new_command(opts: NewOptions) -> Result<()> {
    let year = opts.year.unwrap_or(2015);
    let day = opts.day.unwrap_or(1);

    create_bin_file(year, day)?;
    update_manifest(year, day)?;
    open_bin_file(year, day);

    print_message(
        "Completed",
        format!("Use cargo advent run -y {year} -d {day} to run"),
    );

    Ok(())
}

fn create_bin_file(year: u32, day: u32) -> Result<()> {
    let workspace_path = get_workspace_path();
    let relative_bin_path = get_bin_path(year, day);
    let bin_path = workspace_path.join(&relative_bin_path);

    if bin_path.exists() {
        print_message("Verified", format!("{relative_bin_path} already exists."));
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
    let bin_name = get_binary_name(year, day);

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
