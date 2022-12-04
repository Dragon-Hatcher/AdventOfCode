use std::fmt::Display;

mod get_data;

#[derive(Debug)]
enum Error {
    ExpectedDayNumber,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected day number")
    }
}

impl std::error::Error for Error {}

fn main() -> Result<(), anyhow::Error> {
    let Some(day) = std::env::args().nth(1) else { return Err(Error::ExpectedDayNumber.into()); };
    let day = day.parse::<usize>()?;
    get_data::get_data(day)?;
    println!("Successfully added new day.");

    Ok(())
}
