use std::fs::File;
use std::io::{BufReader, BufRead, Error};

//let data: Vec<u32> = data_str.lines_any().filter_map(|s| s.trim().parse()).collect();

pub fn day01() -> Result<(), Error> {
    let path = "src/aoc2020/Day01.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut data : Vec<u32> = vec![];

    for line in buffered.lines() {
        let my_int: u32 = line?.trim().parse().unwrap();
        data.push(my_int);
    }

    let len = data.len();
    for i in 1..len {
    for j in 2..len {
        let a = data[i];
        let b = data[j];
        let tot = a + b;

        if tot == 2020
        {
            println!("{} product of a,b", a*b);
        }
        
        for k in 3..len {
            let c = data[k];
            let tot3 = a + b + c;

            if tot3 == 2020
            {
                println!("{} product of a,b,c", a*b*c);
            }
        }
    }
    }

    Ok(())
}
