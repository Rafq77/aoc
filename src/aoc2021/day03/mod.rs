use std::{io::Error, num::ParseIntError};

use itertools::Itertools;

pub fn part1(input: String) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let line_length = lines[0].len();

    // Instead of this abomination it would be easier to go by rows and just have a hashmap counting 0s and 1s for each column...
    let gamma: String = (0..line_length)
        .map(|col| {
            let ones_count = lines
                .iter()
                .map(|line| line.chars().nth(col).unwrap()) // this is really counterintuitive
                .filter(|&c| c == '1')
                .count();
            if ones_count >= lines.len() / 2 {
                '1'
            } else {
                '0'
            }
        })
        .collect();

    let gamma_rate = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_rate = gamma_rate ^ ((1 << line_length) - 1); // XOR to get epsilon

    gamma_rate * epsilon_rate
}

pub fn part2(input: String) -> Result<i32, ParseIntError> {
    let lines = input.lines().collect_vec();

    let oxygen = get_desired_bit(&lines, BitCriteria::Oxygen);
    let co2 = get_desired_bit(&lines, BitCriteria::CO2);

    Ok(i32::from_str_radix(co2, 2)? * i32::from_str_radix(oxygen, 2)?)
}

#[derive(PartialEq)]
enum BitCriteria {
    Oxygen,
    CO2,
}

fn get_desired_bit<'a>(lines: &'a [&str], criteria: BitCriteria) -> &'a str {
    let line_length = lines[0].len();
    let mut lines_clone = lines.to_owned();

    for i in 0..line_length {
        let ones_count = lines_clone
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .filter(|&c| c == '1')
            .count();

        let desired_bit = match criteria {
            BitCriteria::Oxygen => {
                if ones_count >= lines_clone.len().div_ceil(2) {
                    '1'
                } else {
                    '0'
                }
            }
            BitCriteria::CO2 => {
                if ones_count >= lines_clone.len().div_ceil(2) {
                    '0'
                } else {
                    '1'
                }
            }
        };

        lines_clone = lines_clone
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == desired_bit)
            .collect_vec();

        if lines_clone.len() == 1 {
            break;
        }
    }
    lines_clone[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        // Determine which bit is the most common in each column.
        // Tricky, because inputs are transposed (you get rows in each iteration)
        // For example, the first column has five 0 bits and seven 1 bits.
        // So the most common bit is 1.
        // Doing that for all of them should produce 01001, or 22 in decimal.
        // Then take the XOR of that value, wchich should be 9.
        // Multiplying and return the multiplication of both values: 198

        assert_eq!(part1(TEST_CASE.to_string()), 198);
    }

    #[test]
    fn test_part2() {
        // For second part selecting the most common bit filter the input
        // So that only lines with that bit in that column are kept.
        // Effectively reducing the input with each iteration.
        assert_eq!(part2(TEST_CASE.to_string()), Ok(230));
    }
}

pub fn day03() -> Result<(), Error> {
    let input = include_str!("input.txt").to_string();
    println!("Day03 part1: {0}", part1(input.clone())); // 2743844
    println!("Day03 part2: {0}", part2(input.clone()).unwrap()); // 6677951
    Ok(())
}
