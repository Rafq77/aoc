use std::collections::HashMap;

// A/X - Rock
// B/Y - Paper
// C/Z - Scissors

// 1 Rock
// 2 Paper
// 3 Scissors
// 0 loss // X
// 3 draw // Y
// 6 win  // Z

// 1 paper beats rock // 2 > 1
// 2 scissors beats paper 3 > 2
// 3 rock beats scissors 1 > 3

fn rps_round_1(_round: &str) -> i32 {
    let rps = HashMap::from([
        ("A X", 3 + 1),
        ("A Y", 6 + 2),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 6 + 1),
        ("C Y", 2),
        ("C Z", 3 + 3),
    ]);
    rps[_round]
}

fn rps_round_2(_round: &str) -> i32 {
    let rps = HashMap::from([
        ("A X", 3),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),
        ("B X", 1),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 2),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6),
    ]);
    rps[_round]
}

pub fn rps_scheduler_1(str: &str) -> i32 {
    str.lines().map(rps_round_1).sum::<i32>()
}

pub fn rps_scheduler_2(str: &str) -> i32 {
    str.lines().map(rps_round_2).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rps_test_1() {
        let test = "A Y";
        assert_eq!(8, rps_round_1(test));
    }

    #[test]
    fn rps_test_2() {
        let test = "A Y";
        assert_eq!(4, rps_round_2(test));
        assert_eq!(1, rps_round_2("B X"));
        assert_eq!(7, rps_round_2("C Z"));
    }

    #[test]
    fn example_part1() {
        let test = "A Y\nB X\nC Z";
        assert_eq!(15, rps_scheduler_1(test));
    }

    #[test]
    fn example_part2() {
        let test = "A Y\nB X\nC Z";
        assert_eq!(12, rps_scheduler_2(test));
    }
}

pub fn day02() {
    let rounds = include_str!("input.txt");

    println!("result est: {}", rps_scheduler_1(rounds)); // 15523
    println!("result est: {}", rps_scheduler_2(rounds)); // 15702
}
