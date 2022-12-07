use itertools::Itertools;

fn find_sequence(str: &str, window_size: usize) -> i32 {
    let table = str.chars().collect::<Vec<char>>();
    let windows = table.windows(window_size);

    let s = windows
        .filter(|w| w.iter().duplicates().collect::<Vec<_>>().is_empty())
        .take(1)
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .to_owned()
        .into_iter()
        .collect::<String>();
    let pos = (str.find(&s).unwrap() + window_size) as i32;

    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_translation_test() {
        let test = [
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",    // 7
            "bvwbjplbgvbhsrlpgdmjqwftvncz",      // 5
            "nppdvjthqldpwncqszvftbrmjlhg",      // 6
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", // 10
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",  //11
        ];

        assert_eq!(19, find_sequence(test[0], 14));
        assert_eq!(7, find_sequence(test[0], 4));
        assert_eq!(5, find_sequence(test[1], 4));
        assert_eq!(6, find_sequence(test[2], 4));
        assert_eq!(10, find_sequence(test[3], 4));
        assert_eq!(11, find_sequence(test[4], 4));
    }
}

pub fn day06() {
    let _input = include_str!("input.txt");
    println!("Day06 part1: {}", find_sequence(_input, 4)); // 1658
    println!("Day06 part2: {}", find_sequence(_input, 14)); // 2260
}
