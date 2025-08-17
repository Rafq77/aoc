use itertools::Itertools;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Types {
    // Other,
    HighCard,
    Pair,
    DoublePair,
    Three,
    Full,
    Caret, // four
    Five,
}

#[derive(Debug, PartialEq)]
struct Value {
    high_card: u32,
    pairs: u32,
    threes: u32,
    fours: u32,
    fives: u32,
}

static VALUES: [Value; 7] = [
    Value::new(1, 0, 0, 0, 0),
    Value::new(0, 1, 0, 0, 0),
    Value::new(0, 2, 0, 0, 0),
    Value::new(0, 0, 1, 0, 0),
    Value::new(0, 1, 1, 0, 0),
    Value::new(0, 0, 0, 1, 0),
    Value::new(0, 0, 0, 0, 1),
];

static TYPES_KEYS: [Types; 7] = [
    Types::HighCard,
    Types::Pair,
    Types::DoublePair,
    Types::Three,
    Types::Full,
    Types::Caret,
    Types::Five,
];

impl Value {
    pub const fn new(high_card: u32, pairs: u32, threes: u32, fours: u32, fives: u32) -> Self {
        Self {
            high_card,
            pairs,
            threes,
            fours,
            fives,
        }
    }

    pub fn new_zero() -> Self {
        Self {
            high_card: 0,
            pairs: 0,
            threes: 0,
            fours: 0,
            fives: 0,
        }
    }
}

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
        _ => card.to_digit(10).unwrap(),
    }
}

fn is_high_card(counts: &HashMap<u32, u32>) -> bool {
    counts.len() == 5
}

fn solve(hands: &str, part2: bool) -> u32 {
    let mut types = HashMap::new();

    for hand in hands.lines() {
        // Split the hand into cards and bid consistently
        if let Some((cards_str, bid_str)) = hand.split_whitespace().collect_tuple() {
            let mut counts = HashMap::new();
            for card in cards_str.chars() {
                *counts.entry(parse_card(card)).or_insert(0) += 1;
            }

            if part2 {
                // Get joker count and remove jokers from counts
                let joker_count = counts.remove(&11).unwrap_or(0);

                // Determine the best card type to convert jokers to
                if joker_count > 0 {
                    if joker_count == 5 {
                        // All jokers - make them all aces
                        counts.insert(14, 5);
                    } else {
                        // Find most frequent card (or highest value if tied)
                        let best_card = *counts
                            .iter()
                            .max_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)))
                            .map(|(card, _)| card)
                            .unwrap();

                        // Add jokers to this card type
                        *counts.entry(best_card).or_insert(0) += joker_count;
                    }
                }
            }

            // Check for pairs, threes, fours, and fives
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

            // Check for high card
            if is_high_card(&counts) {
                combos.high_card = 1;
            }

            // Convert cards to values - in part2, J is worth 1 instead of 11
            let hand_values: Vec<u32> = cards_str
                .chars()
                .map(|c| if part2 && c == 'J' { 1 } else { parse_card(c) })
                .collect();

            let bid = bid_str.parse::<u32>().unwrap();

            // Store the hand in the appropriate type bucket
            for (t, v) in TYPES_KEYS.iter().zip(VALUES.iter()) {
                if *v == combos {
                    types
                        .entry(t)
                        .or_insert_with(Vec::new)
                        .push((hand_values, bid));
                    break;
                }
            }
        }
    }

    // Collect all hands into a single list, sorted by overall strength
    let mut all_ranked_hands: Vec<(Types, Vec<u32>, u32)> = Vec::new();

    for hand_type_key in TYPES_KEYS.iter() {
        if let Some(slice) = types.get(hand_type_key) {
            // clone, sort in place, then drain into all_ranked_hands
            let mut hands_of_this_type: Vec<(Vec<u32>, u32)> = slice.clone();
            // compare the card‐vectors first; if equal you could tie‐break by bid
            hands_of_this_type.sort_by(|a, b| a.0.cmp(&b.0));
            for (cards, bid) in hands_of_this_type {
                all_ranked_hands.push((*hand_type_key, cards, bid));
            }
        }
    }

    // `all_ranked_hands` is now sorted by type, then by cards within each type.
    // Calculate total winnings by summing (rank * bid).
    let mut total_winnings = 0;
    for (i, (_type, _cards, bid)) in all_ranked_hands.iter().enumerate() {
        let rank = (i + 1) as u32; // Ranks are 1-based
        total_winnings += rank * bid;
    }
    total_winnings
}

#[cfg(test)]
mod tests {
    use super::*;
    static HANDS: &str = "32T3K 765
T55J5 684
KTJJT 220
KK677 28
QQQJA 483";

    #[test]
    fn given_hands_when_first_card_is_taken_then_() {
        let hand: String = HANDS
            .lines()
            .take(1)
            .map(|line| line.split_whitespace().take(1).collect::<String>())
            .collect();

        let hand_vals: Vec<u32> = hand.chars().map(|ch| parse_card(ch) as u32).collect();

        assert_eq!(vec![3, 2, 10, 3, 13], hand_vals);
    }

    #[test]
    fn given_sorted_ascending_array_when_checking_then_high_card_is_recognized() {
        assert!(is_high_card(
            &[(2, 1), (3, 1), (4, 1), (5, 1), (6, 1)]
                .iter()
                .cloned()
                .collect::<std::collections::HashMap<u32, u32>>()
        ));
        assert!(is_high_card(
            &[(4, 1), (6, 1), (3, 1), (5, 1), (2, 1)]
                .iter()
                .cloned()
                .collect::<std::collections::HashMap<u32, u32>>()
        ));
        assert!(!is_high_card(
            &[(4, 1), (6, 1), (5, 1), (9, 1)]
                .iter()
                .cloned()
                .collect::<std::collections::HashMap<u32, u32>>()
        ));
    }

    #[test]
    fn eval_test() {
        assert_eq!(solve(HANDS, false), 6440); // Example assertion for the provided HANDS static data
        assert_eq!(solve(HANDS, true), 5905); // Example assertion for the provided HANDS static data
    }
}

pub fn day07() {
    let _input = include_str!("input.txt");

    println!("Day07 part1: {:?}", solve(_input, false)); // 145738933 is too low // should be 249483956
    println!("Day07 part2: {:?}", solve(_input, true)); // is 252137472
}
