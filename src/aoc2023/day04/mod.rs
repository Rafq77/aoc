use std::collections::HashSet;

fn find_intersections(input: &str) -> (u32, i32) {
    let mut sum: u32 = 0;
    let mut copies = vec![1; input.lines().count()];
    let mut game_number: u32 = 0;
    for line in input.lines() {
        let (_, tmp) = line.split_once(": ").unwrap();
        let (left, right) = tmp.split_once(" | ").unwrap();

        let result = left
            .split_whitespace()
            .collect::<HashSet<_>>()
            .intersection(&right.split_whitespace().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .len() as u32;

        if result >= 1 {
            sum += u32::pow(2, result - 1);

            let card_count_with_bonus = copies[game_number as usize];
            for i in game_number + 1..game_number + result + 1 {
                copies[i as usize] += card_count_with_bonus;
            }
        }
        game_number += 1;
    }

    (sum, copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let test_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!((13, 30), find_intersections(test_string));
    }
}

pub fn day04() {
    let (part1, part2) = find_intersections(include_str!("input.txt"));

    println!("Day04 part1: {}", part1); // 26346
    println!("Day04 part2: {}", part2); // 26346
}
