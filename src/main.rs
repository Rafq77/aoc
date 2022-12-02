use std::io::Error;

mod aoc2020;

fn main() -> Result<(), Error> {
    aoc2020::aoc2020()?;
    Ok(())
}