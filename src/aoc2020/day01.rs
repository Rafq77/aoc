use std::fs::File;
use std::io::{BufRead, BufReader, Error};

//let data: Vec<u32> = data_str.lines_any().filter_map(|s| s.trim().parse()).collect();

pub fn day01() -> Result<(u32, u32), Error> {
    let path = "src/aoc2020/Day01.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut data: Vec<u32> = vec![];
    let mut part1result = 0;
    let mut part2result = 0;

    for line in buffered.lines() {
        let my_int: u32 = line?.trim().parse().unwrap();
        data.push(my_int);
    }

    let mut flag = false;

    let len = data.len();
    for i in 1..len {
        for j in 2..len {
            let a = data[i];
            let b = data[j];
            let tot = a + b;

            if tot == 2020 {
                part1result = a * b;
            }

            for c in data.iter().take(len).skip(3) {
                let tot3 = a + b + c;

                if tot3 == 2020 {
                    part2result = a * b * c;
                    flag = true;
                    break;
                }
            }
        }
        if flag {
            break;
        };
    }

    Ok((part1result, part2result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        let result = day01();
        assert!(result.is_ok());
        let (part1, part2) = result.unwrap();
        println!("Day01 part1: {}", part1); // 988771
        println!("Day01 part2: {}", part2); // 171933104
    }
}
