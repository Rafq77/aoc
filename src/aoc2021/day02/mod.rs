use std::io::Error;

pub fn part1(input: String) -> i32 {
    let position = input.lines().fold((0, 0), |(horizontal, depth), line| {

        let (direction, value) = line.split_once(" ").unwrap();
        let value: i32 = value.parse().unwrap();
        match direction {
            "forward" => (horizontal + value, depth),
            "down" => (horizontal, depth + value),
            "up" => (horizontal, depth - value),
            _ => (horizontal, depth),
        }
    });

    position.0 * position.1
}
pub fn part2(input: String) -> i32 {
    let position = input.lines().fold((0, 0, 0), |(horizontal, depth, aim), line| {

        let (direction, value) = line.split_once(" ").unwrap();
        let value: i32 = value.parse().unwrap();
        match direction {
            "forward" => (horizontal + value, depth + (aim * value), aim),
            "down" => (horizontal, depth, aim + value),
            "up" => (horizontal, depth, aim - value),
            _ => (horizontal, depth, aim),
        }
    });

    position.0 * position.1
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_CASE.to_string()), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_CASE.to_string()), 900);
    }

}

pub fn day02() -> Result<(), Error> {
    let input  = include_str!("input.txt").to_string();
    println!("Day02 part1: {0}", part1(input.clone())); // 1924923
    println!("Day02 part2: {0}", part2(input.clone())); // 1982495697
    Ok(())
}