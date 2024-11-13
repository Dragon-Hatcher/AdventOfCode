use std::time::Duration;
use yansi::Paint;

pub fn print_run(name: &str, output: &str, runtime: Duration) {
    let width = 46_usize.saturating_sub(name.chars().count() + 2);

    println!(
        "{}: {:>width$}\n{}",
        Paint::cyan(&name).bold(),
        Paint::new(format!("({runtime:?})")).fixed(245),
        Paint::new(output).bold(),
        width = width
    )
}
