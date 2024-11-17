use crate::{manage_meta::Metadata, options::SwitchOptions};
use anyhow::Result;

pub fn switch_command(opts: SwitchOptions) -> Result<()> {
    let mut meta = Metadata::new_from_fs();
    let puzzle = meta.resolve_selected_puzzle(opts.year, opts.day)?;

    meta.set_active_puzzle(puzzle)?;

    Ok(())
}
