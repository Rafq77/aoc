fn replace_digit_words(input: &str) -> String {

    let digit_dict: [(&str, &str); 9] = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut result = String::new();
    let mut tmp_word = String::new();

    for c in input.chars() {
        if c.is_numeric() {
            result.push_str(&tmp_word.to_string());
            result.push(c);
            tmp_word.clear();
        } else {
            tmp_word.push(c);
            let digit_found = digit_dict.iter().any(|&(d, _n)| tmp_word.contains(d));
            if digit_found {
                for &(word, digit) in &digit_dict {
                    if tmp_word.contains(word) {
                        tmp_word = tmp_word.replace(word, digit);
                        result.push_str(&tmp_word.to_string());
                        tmp_word.clear();
                        /* sneaky tricky, some word wrap together, but this shouldn't exclude them. 
                        Therfore leave the last letter in "workping place" to append next chars 
                        and complete potential digit word*/
                        tmp_word.push(word.chars().rev().next().unwrap()); 
                        break;
                    }
                }
            }
        }
    }
    if !tmp_word.is_empty() {
        result.push_str(&tmp_word.to_string());
    }

    result
}

fn calc_trebuchet(_input: &str, words2digits: bool) -> u32 {
    let mut total = 0;
    for line in _input.lines() {
        let mut line_tmp = line.to_owned();

        if words2digits {
            line_tmp = replace_digit_words(&line_tmp);
        }

        let number_string = line_tmp.chars().filter(|c| c.is_digit(10)).collect::<String>();
        let number = number_string
            .chars()
            .take(1)
            .chain(number_string.chars().rev().take(1))
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        total += number;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    static INSTR_SMALL: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    static INSTR_BIG: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn examples() {
        assert_eq!(142, calc_trebuchet(INSTR_SMALL, false));

        let mut lines = INSTR_BIG.lines().into_iter();
        assert_eq!("2o19e", replace_digit_words(&lines.next().unwrap()));
        assert_eq!("82o3e", replace_digit_words(&lines.next().unwrap()));
        assert_eq!("abc1e23exyz", replace_digit_words(&lines.next().unwrap()));
        assert_eq!("x21e34r", replace_digit_words(&lines.next().unwrap()));
        assert_eq!("49e8t7n2", replace_digit_words(&lines.next().unwrap()));
        assert_eq!("z18t234", replace_digit_words(&lines.next().unwrap()));
        assert_eq!("7pqrst6xteen", replace_digit_words(&lines.next().unwrap()));

        assert_eq!(281, calc_trebuchet(INSTR_BIG, true));
    }
}

pub fn day01() {
    let _input = include_str!("input.txt"); 
    println!("Day01 part1: {}",  calc_trebuchet(_input, false)); // 55834
    println!("Day01 part2: {}",  calc_trebuchet(_input, true)); // 53254 is too high, cheat 53221
}

