// use array_tool::vec::Intersect;
use itertools::Itertools;

struct CoordsExtractor {
    array: array2d::Array2D<char>,
    visited_coords: Vec<(usize, usize)>,
    legit_nubmers: Vec<u32>,
}

impl CoordsExtractor {
    fn new(string: &str) -> Self {
        let rows = string
            .lines()
            .into_iter()
            .map(|l| l.chars().collect_vec())
            .collect::<Vec<_>>()
            .to_vec();
        let array = array2d::Array2D::from_rows(&rows).unwrap();
        CoordsExtractor {
            array: array,
            visited_coords: Vec::<(usize, usize)>::new(),
            legit_nubmers: Vec::<u32>::new(),
        }
    }

    pub fn calc_product(&mut self) -> u32 {
        // extract coordinates of special characters
        let special_coords: Vec<(usize, usize)> = self
            .array
            .rows_iter()
            .enumerate()
            .flat_map(|row| {
                row.1
                    .into_iter()
                    .enumerate()
                    .filter(|(_, &ch)| !ch.is_digit(10) && ch != '.')
                    .map(move |(j, _)| (row.0, j))
            })
            .collect();

        special_coords
            .iter()
            .map(|(x, y)| self.extract_number((*x, *y)))
            .sum::<u32>()
    }

    fn extract_number(&mut self, coords: (usize, usize)) -> u32 {
        let mut multipliers = Vec::<u32>::new();

        // Iterate over the surrounding 9 coordinates
        for dx in -1..=1 {
            for dy in -1..=1 {
                let new_x = (coords.0 as isize + dx) as usize;
                let new_y = (coords.1 as isize + dy) as usize;

                // Check if the character is a digit
                if let Some(val) = self.array.get(new_x, new_y) {
                    if val.is_digit(10) {
                        if let Some(num) = self.follow_number((new_x, new_y)) {
                            let number = num.parse::<u32>().unwrap();
                            self.legit_nubmers.push(number.to_owned());
                            multipliers.push(number.to_owned());
                        }
                    }
                }
            }
        }

        match multipliers.len() {
            0 => 0,
            1 => 0,
            _ => multipliers.iter().product::<u32>(),
        }
    }

    fn follow_number(&mut self, coords: (usize, usize)) -> Option<String> {
        // I assume these coords are on a digit
        // lets go leftmost and start building up my digit from there
        let (row, mut col) = coords;
        while col > 0
            && self
                .array
                .get(row, col - 1)
                .map_or(false, |&c| c.is_digit(10))
        {
            col -= 1;
        }

        // Extract all digits of the number
        let mut number = String::new();
        while let Some(&c) = self.array.get(row, col) {
            if self.visited_coords.contains(&(row, col)) {
                return None;
            }

            if c.is_digit(10) {
                self.visited_coords.push((row, col).to_owned());
                number.push(c);
                col += 1;
            } else {
                break;
            }
        }

        Some(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let test_string = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let mut extractor = CoordsExtractor::new(test_string);
        let part2 = extractor.calc_product();

        assert_eq!(4361, extractor.legit_nubmers.iter().sum::<u32>());
        assert_eq!(467835, part2);
    }
}

pub fn day03() {
    let mut extractor = CoordsExtractor::new(include_str!("input.txt"));

    let part2 = extractor.calc_product();
    println!("Day03 part1: {}", extractor.legit_nubmers.iter().sum::<u32>()); // 521515
    println!("Day03 part2: {}", part2); // 69527306
}
