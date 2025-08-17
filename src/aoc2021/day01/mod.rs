/// Count incrementing pairs in a number list
use std::io::{Error};

use itertools::Itertools;

// Part 1: Just count the pairs
pub fn part1(input: String) -> i32 {
    input.lines().collect_vec().windows(2).filter(|pair| {
        let first: i32 = pair[0].parse().unwrap();
        let second: i32 = pair[1].parse().unwrap();
        second > first
    }).count() as i32
}

// Part 2: Count pairs of sums of three consecutive numbers
pub fn part2(input: String) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .collect_vec().windows(2).filter(|pair_of_three| {

            // Within your filter closure:
            let (a, b, c) = pair_of_three[0];
            let first_sum = a + b + c;

            let (d, e, f) = pair_of_three[1];
            let second_sum = d + e + f;

            second_sum > first_sum
        }).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASE: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_CASE.to_string()), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_CASE.to_string()), 5);
    }

}

pub fn day01() -> Result<(), Error> {
    let day01input  = include_str!("input.txt").to_string();
    println!("Day01 part1: {0}", part1(day01input.clone())); // 1665
    println!("Day01 part2: {0}", part2(day01input.clone())); // 1702
    Ok(())
}