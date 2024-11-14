use crate::{
    manage_meta::{Metadata, Puzzle},
    options::SwitchOptions,
};
use anyhow::Result;

pub fn switch_command(opts: SwitchOptions) -> Result<()> {
    let Puzzle { year, day } =
        Metadata::new_from_fs().resolve_selected_puzzle(opts.year, opts.day)?;
    Metadata::new_from_fs().set_active_puzzle(year, day)?;

    Ok(())
}
