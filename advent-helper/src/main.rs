mod options;
mod printers;
mod manage_inputs;
mod manage_meta;
mod helpers;
mod run;
mod new;

use anyhow::Result;
use options::{Options, SubCommand};

fn main() -> Result<()> {
    let opts: Options = argh::from_env();

    match opts.nested {
        SubCommand::Run(opts) => run::run_command(opts),
        SubCommand::New(opts) => new::new_command(opts)
    }
}
