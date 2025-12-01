use std::io::Error;

mod day01;
mod day02;
mod day03;
mod day04;
// mod day05;

pub fn aoc2021() -> Result<(), Error> {
    println!("*** YEAR 2021!!!! ***");
    day01::day01()?;
    day02::day02()?;
    day03::day03()?;
    day04::day04()?;
    // day05::day05()?;
    Ok(())
}
