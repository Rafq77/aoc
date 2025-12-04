// use regex::Regex;
// use std::fs;

use std::default;

pub fn part1(input: String) -> i32 {
    input.lines()
    .map(|line| find_jolt(line))
    .map(|jolt_str| jolt_str.parse::<i32>().unwrap())
    .sum()
}


pub fn part2(input: String) -> i64 {
    0
}

struct Digit {
    index: i32,
    value: char
}

impl Digit {
    fn default() -> Self {
        Digit {
            index: 0,
            value: '0'
        }
    }
}

fn find_jolt(input: &str) -> String {
    let mut left: Digit = Digit::default();
    let mut right: Digit = Digit::default();

    let chars: Vec<char> = input.chars().collect();

    for (i, &c) in chars.iter().take(chars.len()-1).enumerate() {
        if c > left.value {
            left.index = i as i32;
            left.value = c; 
        }
    }

    for (i, &c) in chars.iter().rev().take(chars.len()- 1 - left.index as usize).enumerate() {
        if c > right.value {
            right.index = (chars.len() - 1 - i) as i32;
            right.value = c; 
        }
    }

    left.value.to_string() + &right.value.to_string()
}

fn check_range(range: (i64, i64), multiple_repetitions: bool) -> Vec<i64> {
    let mut invalid_id_count = Vec::<i64>::new();
    for num in range.0..=range.1 {
        let num_str = num.to_string();
        let num_str_len = num_str.len();

        if !multiple_repetitions {
            // ignore numbers of uneven length
            if num_str_len % 2 != 0 {
                continue;
            }

            // get the copy length of a number
            let copy_len = num_str_len / 2;
            let first_half = num_str[..copy_len].to_string();
            let second_half = num_str[copy_len..].to_string();

            if first_half == second_half {
                invalid_id_count.push(num);
            }
        } else {
            // Allow any exact repetition length (e.g., 111, 1010, 121212).
            if num_str_len < 2 {
                continue;
            }

            // Try every chunk length that divides the full length; if all chunks
            // match the first chunk, the ID is a repetition.
            let mut is_repetition = false;
            for chunk_len in 1..=num_str_len / 2 {
                if num_str_len % chunk_len != 0 {
                    continue;
                }

                let chunk = &num_str[..chunk_len];
                if num_str
                    .as_bytes()
                    .chunks(chunk_len)
                    .all(|part| part == chunk.as_bytes())
                {
                    is_repetition = true;
                    break;
                }
            }

            if is_repetition {
                invalid_id_count.push(num);
            }
        }
    }

    invalid_id_count
}

mod tests {
    use super::*;
    

    #[allow(dead_code)]
    static BATTERIES: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_single_jolting() {
        let lines = BATTERIES.lines().collect::<Vec<&str>>();
        assert_eq!(find_jolt(lines[0]), "98");
        assert_eq!(find_jolt(lines[1]), "89");
        assert_eq!(find_jolt(lines[2]), "78");
        assert_eq!(find_jolt(lines[3]), "92");
    }

    #[test]
    fn test_part1() {
        let result = part1(BATTERIES.to_string());
        assert_eq!(result, 357);
    }

    #[test]
    fn test_12_jolting() {
        let lines = BATTERIES.lines().collect::<Vec<&str>>();
        assert_eq!(find_jolt(lines[0]), "987654321111");
        assert_eq!(find_jolt(lines[1]), "811111111119");
        assert_eq!(find_jolt(lines[2]), "434234234278");
        assert_eq!(find_jolt(lines[3]), "888911112111");
    }

    #[test]
    fn test_part2() {
        let result = part2(BATTERIES.to_string());
        assert_eq!(result, 3121910778619);
    }
}

pub fn day03() {
    let day02input = include_str!("input.txt").to_string();
    println!("Day03 part1: {0}", part1(day02input.clone())); // 17207
    // println!("Day03 part2: {0}", part2(day02input.clone()));
}
