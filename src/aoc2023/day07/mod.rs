#[cfg(test)]
#[derive(Eq, PartialEq, Hash, Debug)]
enum Types {
    Other,
    Pair,
    DoublePair,
    Three,
    Full,
    Caret, // four
    Five,
}

#[cfg(test)]
#[derive(Debug, PartialEq)]
struct Value {
    pairs: u32,
    threes: u32,
    fours: u32,
    fives: u32,
}

#[cfg(test)]
impl Value {
    fn new(pairs: u32, threes: u32, fours: u32, fives: u32) -> Self {
        Self {
            pairs: pairs,
            threes: threes,
            fours: fours,
            fives: fives,
        }
    }
    fn new_zero() -> Self {
        Self {
            pairs: 0,
            threes: 0,
            fours: 0,
            fives: 0,
        }
    }
}

#[cfg(test)]
fn parse_card(card: char) -> u32 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap() as u32,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;
    static HANDS: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn parse_test() {
        let hand: String = HANDS
            .lines()
            .take(1)
            .map(|line| line.split_whitespace().take(1).collect::<String>())
            .collect();

        let hand_vals: Vec<u32> = hand.chars().map(|ch| parse_card(ch) as u32).collect();

        assert_eq!(vec![3, 2, 10, 3, 13], hand_vals);
    }

    #[test]
    fn eval_test() {
        let values = [
            Value::new(1, 0, 0, 0),
            Value::new(2, 0, 0, 0),
            Value::new(0, 1, 0, 0),
            Value::new(1, 1, 0, 0),
            Value::new(0, 0, 1, 0),
            Value::new(0, 0, 0, 1),
        ];

        let mut types = HashMap::new();
        let types_keys = [
            Types::Pair,
            Types::DoublePair,
            Types::Three,
            Types::Full,
            Types::Caret,
            Types::Five,
        ];

        for hand in HANDS.lines() {

        let mut counts = HashMap::new();
        for card in hand.split_whitespace().take(1).collect::<String>().chars() {
            *counts.entry(parse_card(card)).or_insert(0) += 1;
        }

        dbg!(&counts);
        let mut combos = Value::new_zero();
        for &count in counts.values() {
            match count {
                2 => combos.pairs += 1,
                3 => combos.threes += 1,
                4 => combos.fours += 1,
                5 => combos.fives += 1,
                _ => (),
            }
        }

        for (t, v) in types_keys.iter().zip(values.iter()) {
            if *v == combos {
                types.entry(t).or_insert_with(Vec::new).push(hand.clone());
            }
        }
    }

        dbg!(&types);
    }
}

pub fn day07() {
    let _input = include_str!("input.txt");

    //println!("Day07 answers: {:?}", parse_cards(_input)); //
}
