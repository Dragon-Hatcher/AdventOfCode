mod helpers;
mod manage_inputs;
mod manage_meta;
mod new;
mod options;
mod printers;
mod run;
mod submit;
mod test;
mod switch;

use anyhow::Result;
use options::{Options, SubCommand};

fn main() -> Result<()> {
    let opts: Options = argh::from_env();

    match opts.nested {
        SubCommand::Run(opts) => run::run_command(opts),
        SubCommand::Test(opts) => test::test_command(opts),
        SubCommand::New(opts) => new::new_command(opts),
        SubCommand::Submit(opts) => submit::submit_command(opts, true),
        SubCommand::Switch(opts) => switch::switch_command(opts)
    }
}
