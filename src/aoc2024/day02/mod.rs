// use regex::Regex;
// use std::fs;

pub fn part1(input: String) -> i32 {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            levels.windows(2).map(|w| w[1] - w[0]).all(|d| {
                (1..=3).contains(&d.abs()) && d.signum() == (levels[1] - levels[0]).signum()
            })
        })
        .count() as i32

    // .map(|line| {
    //     let nums: Vec<i32> = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
    //     nums.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0])) || (1..=3).contains(&(w[0] - w[1]))) as i32
    // })
    // .sum()

    // .map(|line| {
    //         match line.split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>().iter().is_sorted() {
    //             false => 0,
    //             true => 1
    //         }
    //     }).sum::<i32>()
}

pub fn part2(input: String) -> i32 {
    fn is_safe(levels: &[i32]) -> bool {
        levels
            .windows(2)
            .map(|w| w[1] - w[0])
            .all(|d| (1..=3).contains(&d.abs()) && d.signum() == (levels[1] - levels[0]).signum())
    }

    input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            is_safe(&levels)
                || (0..levels.len()).any(|i| is_safe(&[&levels[..i], &levels[i + 1..]].concat()))
        })
        .count() as i32
}

mod tests {
    // use super::*;
    // use ParsePersonError::*;

    #[test]
    fn empty_input() {
        let part1 = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();

        assert_eq!(super::part1(part1.clone()), 2);
        // assert_eq!(part2(part1), 4);
    }
}

pub fn day02() {
    let day02input  = include_str!("input.txt").to_string();
    // let day02input = fs::read_to_string("input.txt").expect("Had problems opening day02.txt");
    println!("Day02 part1: {0}", part1(day02input.clone())); // 313, 294 is too high! //252 was just right!
    println!("Day02 part2: {0}", part2(day02input)); // 324 tedious...
}
