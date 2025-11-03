use array_tool::vec::Intersect;
use itertools::Itertools;

pub fn eval_rucksacks(str: &str) -> i32 {
    str.lines().map(eval_doubles).sum()
}

pub fn eval_doubles(str: &str) -> i32 {
    let mid = str.len() / 2;
    let l = &str[..mid];
    let r = &str[mid..];

    let unique = l.as_bytes().to_vec().intersect(r.as_bytes().to_vec());

    //dbg!(str, l, r, unique[0]);
    let letter = i32::from(unique[0]);

    /*
    let y = match x {
        5 => 10,
        _ => 15,
    */

    if letter > 96 {
        letter - 96
    } else {
        letter - 65 + 27
    }
}

pub fn eval_3chunk(str: &str) -> i32 {
    let mut total = 0;

    for mut chunk in &str.lines().chunks(3) {
        let x = chunk.next().unwrap();
        let y = chunk.next().unwrap();
        let z = chunk.next().unwrap();

        let unique = x
            .as_bytes()
            .to_vec()
            .intersect(y.as_bytes().to_vec())
            .intersect(z.as_bytes().to_vec());

        let letter = i32::from(unique[0]);
        let pts = if letter > 96 {
            letter - 96
        } else {
            letter - 65 + 27
        };
        total += pts;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let test_string = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, eval_rucksacks(test_string));
    }

    #[test]
    fn doubles_test() {
        let test = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        assert_eq!(16, eval_doubles(&test[0]));
        assert_eq!(38, eval_doubles(&test[1]));
        assert_eq!(42, eval_doubles(&test[2]));
        assert_eq!(22, eval_doubles(&test[3]));
        assert_eq!(20, eval_doubles(&test[4]));
        assert_eq!(19, eval_doubles(&test[5]));
    }

    #[test]
    fn intersect_three() {
        let test_string = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(70, eval_3chunk(test_string));
    }
}

pub fn day03() {
    let backpacks = include_str!("input.txt");

    println!("Day03 part1: {}", eval_rucksacks(backpacks)); // 8018
    println!("Day03 part2: {}", eval_3chunk(backpacks)); // 2518
}
