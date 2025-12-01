use std::{io::Error, num::ParseIntError};
use std::collections::HashMap;

type VentMap = HashMap<(i32, i32), i32>;

struct VentLine {
    start: Point,
    end: Point,
}

struct Point {
    x: i32,
    y: i32,
}

pub fn part1(input: String) -> i32 {
    0
}

pub fn part2(input: String) -> Result<i32, ParseIntError> {
    Ok(0)
}

fn add_line(vent_map: &mut VentMap, line: &VentLine) {
}

fn parse_input(input: &str) -> Result<Vec<VentLine>, ParseIntError> {
    let vent_map: Result<Vec<VentLine>, ParseIntError> = input.lines().into_iter().map(|line| {
   let (start_str, end_str) = line
                .split_once(" -> ")
                .ok_or(ParseIntError)?;
            let (start_x, start_y) = start_str
                .split_once(',')
                .ok_or(ParseIntError("asd"))?;
            let (end_x, end_y) = end_str
                .split_once(',')
                .ok_or(ParseIntError)?;

            let start = Point {
                x: start_x.parse()?,
                y: start_y.parse()?,
            };

            let end = Point {
                x: end_x.parse()?,
                y: end_y.parse()?,
            };

            Ok(VentLine { start, end })
    }).collect()?;

    vent_map
}

    
#[cfg(test)]
mod tests {
    use super::*;

    /* Each line is in format "x1,y1 -> x2,y2"
    Solution proposal, go with hashmap? No need to make a full 2d grid
     */
    static TEST_CASE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_parse_input() {
        let lines: Vec<VentLine> = parse_input(TEST_CASE.to_string());

    }

    #[test]
    fn test_part1() {


        assert_eq!(part1(TEST_CASE.to_string()), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_CASE.to_string()), Ok(230));
    }
}

pub fn day05() -> Result<(), Error> {
    let input = include_str!("input.txt").to_string();
    println!("Day05 part1: {0}", part1(input.clone())); // 2743844
    println!("Day05 part2: {0}", part2(input.clone()).unwrap()); // 6677951
    Ok(())
}
