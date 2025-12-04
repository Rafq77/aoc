// use regex::Regex;
// use std::fs;

pub fn part1(input: String) -> i64 {
    parse_input(&input)
        .iter()
        .map(|&range| check_range(range, false).iter().sum::<i64>() as i64)
        .sum()
}

pub fn part2(input: String) -> i64 {
    parse_input(&input)
        .iter()
        .map(|&range| check_range(range, true).iter().sum::<i64>() as i64)
        .sum()
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .split(',')
        .map(|pair| {
            let split_pair = pair.split_once('-').unwrap();
            (
                split_pair.0.parse::<i64>().unwrap(),
                split_pair.1.parse::<i64>().unwrap(),
            )
        })
        .collect()
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

    static RANGES: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_parsing() {
        let ranges = parse_input(RANGES);
        assert_eq!(ranges.len(), 11);
        assert_eq!(ranges[0].0, 11);
        assert_eq!(ranges[0].1, 22);

        assert_eq!(check_range(ranges[0], false), vec![11, 22]);
    }

    #[test]
    fn check_ranges_with_double_repetition() {
        assert_eq!(check_range((11, 22), false), vec![11, 22]);
        assert_eq!(check_range((95, 115), false), vec![99]);
        assert_eq!(check_range((998, 1012), false), vec![1010]);
        assert_eq!(
            check_range((1188511880, 1188511890), false),
            vec![1188511885]
        );
        assert_eq!(check_range((222220, 222224), false), vec![222222]);
        assert_eq!(check_range((1698522, 1698528), false), vec![]);
        assert_eq!(check_range((446443, 446449), false), vec![446446]);
        assert_eq!(check_range((38593856, 38593862), false), vec![38593859]);
    }

    #[test]
    fn check_ranges_with_multiple_repetitions() {
        assert_eq!(check_range((11, 22), true), vec![11, 22]);
        assert_eq!(check_range((95, 115), true), vec![99, 111]);
        assert_eq!(check_range((998, 1012), true), vec![999, 1010]);
        assert_eq!(
            check_range((1188511880, 1188511890), true),
            vec![1188511885]
        );
        assert_eq!(check_range((222220, 222224), true), vec![222222]);
        assert_eq!(check_range((1698522, 1698528), true), vec![]);
        assert_eq!(check_range((446443, 446449), true), vec![446446]);
        assert_eq!(check_range((38593856, 38593862), true), vec![38593859]);
        assert_eq!(check_range((824824821, 824824827), true), vec![824824824]);
        assert_eq!(
            check_range((2121212118, 2121212124), true),
            vec![2121212121]
        );
    }

    #[test]
    fn test_part1() {
        let result = part1(RANGES.to_string());
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part2() {
        let result = part2(RANGES.to_string());
        assert_eq!(result, 4174379265);
    }
}

pub fn day02() {
    let day02input = include_str!("input.txt").to_string();
    println!("Day02 part1: {0}", part1(day02input.clone())); // 804, 881 are still too low // 1227775554
    println!("Day02 part2: {0}", part2(day02input.clone()));
}
