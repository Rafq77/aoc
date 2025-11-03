use std::ops::RangeInclusive;

pub fn is_overlaping(str: &str, any: bool) -> bool {
    let mut _nums: Vec<RangeInclusive<i32>> = str
        .split(',')
        .filter_map(|x| {
            x.split_once('-')
                .map(|(n, m)| n.parse::<i32>().unwrap()..=m.parse::<i32>().unwrap())
        })
        .collect();

    let r2 = _nums.pop().unwrap();
    let r1 = _nums.pop().unwrap();
    let r3 = *r2.start()..=*r2.end(); // cloning because Ranges can't
                                      // have to use r3 because r2 is "borrowed when converting to iter"

    let mut _result = false;
    let mut _result2 = false;

    if any {
        _result = r2.into_iter().any(|n| r1.contains(&n));
        _result2 = r1.into_iter().any(|n| r3.contains(&n));
    } else {
        _result = r2.into_iter().all(|n| r1.contains(&n));
        _result2 = r1.into_iter().all(|n| r3.contains(&n));
    }

    _result || _result2
}

pub fn find_full_overlaps(_str: &str) -> i32 {
    _str.split('\n')
        .map(|s| is_overlaping(s, false))
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap()
}

pub fn find_partial_overlaps(_str: &str) -> i32 {
    _str.split('\n')
        .map(|s| is_overlaping(s, true))
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_test() {
        let r2 = 4..6;
        let r1 = 6..6;

        let r3 = r2.start.clone()..r2.end.clone();

        let _result2 = r2.into_iter().all(|n| *(&r1.contains(&n)));
        let _result = r1.into_iter().all(|n| r3.contains(&n));
        assert!(_result || _result2);
    }

    #[test]
    fn example_part1() {
        let test_string = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(2, find_full_overlaps(test_string));
    }

    #[test]
    fn doubles_test() {
        let test = [
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
            "19-77,9-9",
        ];

        assert!(!is_overlaping(&test[0], false));
        assert!(!is_overlaping(&test[1], false));
        assert!(!is_overlaping(&test[2], false));
        assert!(is_overlaping(&test[3], false));
        assert!(is_overlaping(&test[4], false));
        assert!(!is_overlaping(&test[5], false));
        assert!(!is_overlaping(&test[6], false));
    }
}

pub fn day04() {
    let _assignments = include_str!("input.txt");

    println!("Day04 part1: {}", find_full_overlaps(_assignments)); // 552 is too high // 494 is correct....
    println!("Day04 part2: {}", find_partial_overlaps(_assignments)); // second is 833 // just replace all/all with any/any
}
