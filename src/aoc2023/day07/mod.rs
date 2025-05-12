#[cfg(test)]
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

#[cfg(test)]
#[derive(Debug, PartialEq)]
struct Value {
    high_card: u32,
    pairs: u32,
    threes: u32,
    fours: u32,
    fives: u32,
}

#[cfg(test)]
impl Value {
    fn new(high_card: u32, pairs: u32, threes: u32, fours: u32, fives: u32) -> Self {
        Self {
            high_card: high_card,
            pairs: pairs,
            threes: threes,
            fours: fours,
            fives: fives,
        }
    }
    fn new_zero() -> Self {
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
        _ => card.to_digit(10).unwrap() as u32,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use itertools::{sorted, Itertools};

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

    fn is_high_card(counts: &Vec<u32>) -> bool {
        let sorted: Vec<_> = sorted(counts).collect();

        return sorted
            .iter()
            .zip(sorted.iter().skip(1))
            .all(|(a, b)| **b == **a + 1);
    }

    #[test]
    fn given_sorted_ascending_array_when_checking_then_high_card_is_recognized() {
            let counts_ordered_ascending = vec![2, 3, 4, 5, 6];
            let counts_unordered_ascending = vec![4, 6, 3, 5, 2];
            let counts_unordered_no_match = vec![4, 6, 13, 5, 9];

            assert!(is_high_card(&counts_ordered_ascending));
            assert!(is_high_card(&counts_unordered_ascending));
            assert!(!is_high_card(&counts_unordered_no_match));
    }

    #[test]
    fn eval_test() {
        let values = [
            Value::new(1, 0, 0, 0, 0),
            Value::new(0, 1, 0, 0, 0),
            Value::new(0, 2, 0, 0, 0),
            Value::new(0, 0, 1, 0, 0),
            Value::new(0, 1, 1, 0, 0),
            Value::new(0, 0, 0, 1, 0),
            Value::new(0, 0, 0, 0, 1),
        ];

        let mut types = HashMap::new();
        let types_keys = [
            Types::HighCard,
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
            let counts_keys = counts.keys().copied().collect();
            if is_high_card(&counts_keys) {
                combos.high_card = 1;
            }

            for (t, v) in types_keys.iter().zip(values.iter()) {
                if *v == combos {
                    if let Some((hand_card, value)) = hand.split_whitespace().collect_tuple() {
                        // Convert hand_card to vector of u32
                        let hand_card: Vec<u32> = hand_card.chars().map(|ch| parse_card(ch) as u32).collect();
                        let hand_value: u32 = value.parse().unwrap();

                        types.entry(t).or_insert_with(Vec::new).push((hand_card, hand_value));
                    }
                }
            }
        }

        // Sort by strength
        for result_type in types.values_mut() {
            result_type.sort_by(|a, b| a.1.cmp(&b.1));
        }

        // sum multiplications by incrementing 
        // let mut rank = 1;
        // let tmp2 = types[&Types::DoublePair].clone().iter().sum();

         // Collect all hands into a single list, sorted by overall strength
         let mut all_ranked_hands: Vec<(Types, Vec<u32>, u32)> = Vec::new();

         for hand_type_key in types_keys.iter() {
             if let Some(hands_of_this_type) = types.get(hand_type_key) {
                 for (cards, bid) in hands_of_this_type {
                     all_ranked_hands.push((*hand_type_key, cards.clone(), *bid));
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
 
         // The old tmp2 line is no longer needed and was incorrect.
         // You can now use total_winnings. For example, in a test:
         // dbg!(total_winnings);
         assert_eq!(total_winnings, 6440); // Example assertion for the provided HANDS static data


        // let tmp2 = types[&Types::DoublePair].clone()[1].1; // TODO 
        // let tmp3 = &tmp2[0];
        // let tmp4 = tmp2[1];
        // types[&Types::Pair][1].produce(0, |acc, (hand_card, hand_value)| {
        //     let mut sum = 0;
        //     for card in hand_card {
        //         sum += *card;
        //     }
        //     acc + sum
        // });


        // TODO rank and multiply

        dbg!(&types);
    }
}

pub fn day07() {
    let _input = include_str!("input.txt");

    // println!("Day07 answers: {:?}", parse_card(_input));
    // println!("Day07 answers: {:?}",_input.chars().map(parse_card).collect::<Vec<u32>>());
}
