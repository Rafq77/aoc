use std::fs::File;
use std::io::{BufReader, BufRead, Error};

//let data: Vec<u32> = data_str.lines_any().filter_map(|s| s.trim().parse()).collect();

fn day01() -> Result<(), Error> {
    let path = "days2020/Day01.txt";

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

fn day02() -> Result<(), Error> {
    let path = "days2020/Day02.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut lines : Vec<String> = vec![];

    for line in buffered.lines() {
        //let my_int: u32 = line?.trim().parse().unwrap();
        lines.push(line?);
    }
    
    let mut count = 0; // part 1
    let mut count2 = 0; // part 2
    
    for line in lines
    {
        let words: Vec<&str> = line.split_whitespace().collect();

        let mut minmax = words[0].split('-');

        let _min = minmax.next().unwrap().parse::<u32>().unwrap();
        let _max = minmax.next().unwrap().parse::<u32>().unwrap();
        let key = words[1].trim_end_matches(':');
        let target = words[2];
        
        let value : u32 = target.matches(key).collect::<Vec<&str>>().len() as u32;
        
        if value >= _min && value <= _max
        {
            count += 1;
        }

        let a = target.chars().nth((_min - 1) as usize).unwrap();
        let b = target.chars().nth((_max - 1) as usize).unwrap();

        let key_char = key.chars().nth(0).unwrap();

        if a != b
        {
            if a == key_char || b == key_char
            {
                count2 += 1;

            }
        }

        //println!("Line min {:?} max {} key {} target {}, count {} value {}", _min, _max, key, target, count, value);
    }

    println!("cnt {}, cnt2 {} ", count, count2);

    Ok(())
}

fn main() -> Result<(), Error> {
    day01()?;
    day02()?;
    Ok(())
}