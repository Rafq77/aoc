use std::io::Error;

mod day01;
mod day02;

pub fn aoc2020() -> Result<(), Error> {
    println!("*** YEAR 2020!!!! ***");
    day01::day01()?;
    day02::day02()?;
    Ok(())
}
