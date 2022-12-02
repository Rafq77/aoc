use std::io::Error;

mod aoc2020;
mod aoc2022;

fn main() -> Result<(), Error> {
    aoc2020::aoc2020()?;
    aoc2022::aoc2022();
    Ok(())
}