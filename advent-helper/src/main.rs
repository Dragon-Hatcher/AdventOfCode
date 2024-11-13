mod options;
mod printers;
mod run;

use anyhow::Result;
use options::Options;

fn main() -> Result<()> {
    let opts: Options = argh::from_env();

    match opts.nested {
        options::SubCommand::Run(opts) => run::run_command(opts),
    }
}
