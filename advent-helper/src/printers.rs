use std::{
    fmt::Display,
    io::{self, IsTerminal},
};
use yansi::Paint;

pub fn print_message(header: &str, msg: impl Display) {
    if io::stdout().is_terminal() {
        println!("{:>12} {}", Paint::green(&header).bold(), msg);
    } else {
        println!("{:>12} {}", header, msg);
    }
}
