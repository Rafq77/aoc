use std::io::Error;

mod day01;
mod day02;
mod day03;

pub fn aoc2021() -> Result<(), Error> {
    println!("*** YEAR 2021!!!! ***");
    day01::day01()?;
    day02::day02()?;
    day03::day03()?;
    Ok(())
}