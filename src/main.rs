use std::io::Error;

mod aoc2020;
mod aoc2022;
mod aoc2023;
mod aoc2024;

fn main() -> Result<(), Error> {
    aoc2020::aoc2020()?;
    aoc2022::aoc2022();
    aoc2023::aoc2023();
    aoc2024::aoc2024();
    Ok(())
}