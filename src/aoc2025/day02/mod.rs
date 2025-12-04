// use regex::Regex;
// use std::fs;

pub fn part1(input: String) -> i64 {
    parse_input(&input)
        .iter()
        .map(|&range| check_range(range).iter().sum::<i64>() as i64)
        .sum()
}

pub fn part2(input: String) -> i32 {
    0
    // fn is_safe(levels: &[i32]) -> bool {
    //     levels
    //         .windows(2)
    //         .map(|w| w[1] - w[0])
    //         .all(|d| (1..=3).contains(&d.abs()) && d.signum() == (levels[1] - levels[0]).signum())
    // }

    // input
    //     .lines()
    //     .filter(|line| {
    //         let levels: Vec<i32> = line
    //             .split_whitespace()
    //             .map(|n| n.parse().unwrap())
    //             .collect();
    //         is_safe(&levels)
    //             || (0..levels.len()).any(|i| is_safe(&[&levels[..i], &levels[i + 1..]].concat()))
    //     })
    //     .count() as i32
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input.split(',').map(|pair| {
        let split_pair = pair.split_once('-').unwrap();
        (
            split_pair.0.parse::<i64>().unwrap(),
            split_pair.1.parse::<i64>().unwrap(),
        )
    }).collect()
}

fn check_range(range: (i64, i64)) -> Vec<i64> {
    let mut invalid_id_count = Vec::<i64>::new();
    for num in range.0..=range.1 {
        let num_str = num.to_string();
        let num_str_len = num_str.len();

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
    }

    invalid_id_count
}

mod tests {
    use super::*;


    static RANGES: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_parsing() {
        let ranges = parse_input(RANGES);
        assert_eq!(ranges.len(), 11);
        assert_eq!(ranges[0].0, 11);
        assert_eq!(ranges[0].1, 22);

        assert_eq!(check_range(ranges[0]), vec![11, 22]);
    }

    #[test]
    fn check_ranges() {
        assert_eq!(check_range((11, 22)), vec![11, 22]);
        assert_eq!(check_range((95, 115)), vec![99]);
        assert_eq!(check_range((998, 1012)), vec![1010]);
        assert_eq!(check_range((1188511880, 1188511890)), vec![1188511885]);
        assert_eq!(check_range((222220, 222224)), vec![222222]);
        assert_eq!(check_range((1698522, 1698528)), vec![]);
        assert_eq!(check_range((446443, 446449)), vec![446446]);
        assert_eq!(check_range((38593856, 38593862)), vec![38593859]);
    }

    #[test]
    fn test_part1() {
        let result = part1(RANGES.to_string());
        assert_eq!(result, 1227775554);
    }
}

pub fn day02() {
    let day02input = include_str!("input.txt").to_string();
    // let day02input = fs::read_to_string("input.txt").expect("Had problems opening day02.txt");
    println!("Day02 part1: {0}", part1(day02input.clone())); // 804, 881 are still too low
    // println!("Day02 part2: {0}", part2(day02input)); // 324 tedious...
}
