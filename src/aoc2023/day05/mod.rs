#[cfg(test)]
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Crops {
    seeds: Vec<u32>,
    encoded_maps: Vec<Vec<(u32, u32, u32)>>,
}

impl Crops {
    fn new(seeds: Vec<u32>, encoded_maps: Vec<Vec<(u32, u32, u32)>>) -> Self {
        Self {
            seeds,
            encoded_maps,
        }
    }
}


#[cfg(test)]
impl Crops {
    pub fn calc_location_simple(&self) -> u32 {
        self.calc_location(&self.seeds)
    }

    pub fn calc_location_range(&self) -> u32 {
        let input_numbers: Vec<u32> = self
            .seeds
            .to_owned()
            .chunks(2)
            .flat_map(|ch| ch[0]..(ch[0] + ch[1]))
            .collect();

        self.calc_location(&input_numbers)
    }

    fn calc_location(&self, seeds: &Vec<u32>) -> u32 {
        let maps: Vec<HashMap<u32, u32>> = self
            .encoded_maps
            .iter()
            .map(|encoded_map| {
                encoded_map
                    .iter()
                    .flat_map(|(start, step, end)| {
                        (*start..(start + end)).map(move |i| (step + (i - start), i))
                    })
                    .collect()
            })
            .collect();

        /* Original unfolded version for educational purposes
        for num in seeds {
            let output = maps
                .iter()
                .fold(*num, |acc, map| *map.get(&acc).unwrap_or(&acc));

            println!("Input: {}, Output: {}", num, output);
        } */
        seeds
            .iter()
            .map(|num| {
                maps.iter()
                    .fold(*num, |acc, map| *map.get(&acc).unwrap_or(&acc))
            })
            .min()
            .unwrap_or(0)
    }
}

pub fn parse_crops(input: &str) -> Crops {
    let numbers: Vec<u32> = input
        .lines()
        .take(1)
        .flat_map(|line| line.trim_start_matches("seeds: ").split_whitespace())
        .filter_map(|num_str| num_str.parse().ok())
        .collect();

    let encoded_maps: Vec<Vec<(u32, u32, u32)>> = input
        .split("\n\n")
        .map({
            |section: &str| {
                section
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let values: Vec<u32> = line
                            .split_whitespace()
                            .filter_map(|num_str| num_str.parse().ok())
                            .collect();
                        (values[0], values[1], values[2])
                    })
                    .collect()
            }
        })
        .collect();

    Crops::new(numbers, encoded_maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INSTR_SMALL: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parsing() {
        let crops = parse_crops(INSTR_SMALL);
        assert_eq!(35, crops.calc_location_simple());
        assert_eq!(46, crops.calc_location_range());
    }

    #[test]
    fn example_part1() {
        let encoded_maps: Vec<Vec<(u32, u32, u32)>> = vec![
            vec![(50, 98, 2), (52, 50, 48)],
            vec![(0, 15, 38), (37, 52, 2), (39, 0, 15)],
            vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
            vec![(88, 19, 78), (18, 25, 70)],
            vec![(45, 88, 23), (81, 45, 19), (68, 64, 13)],
            vec![(0, 69, 1), (1, 0, 69)],
            vec![(60, 56, 37), (56, 93, 4)],
        ];

        let mut crops = Crops::new(vec![79, 14, 55, 13], encoded_maps);

        //dbg!(crops);
        //println!("{}", crops.calc_location_simple());
        //println!("{}", crops.calc_location_range());
    }
}

pub fn day05() {
    let _crops = parse_crops(include_str!("input.txt"));

    //println!("Day05 part1: {}", crops.calc_location_simple()); // part1 462648396 // brutforced
    //println!("Day05 part2: {}", crops.calc_location_range()); // part2 ? 2520479 // nightly ;)
}
