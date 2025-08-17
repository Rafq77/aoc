use num::integer::lcm;
use rayon::prelude::*;

type TreeMap<'a> = std::collections::HashMap<&'a str, (&'a str, &'a str)>;

fn parse_tree<'a>(roadsigns: &'a str) -> TreeMap<'a> {
    let mut tree = TreeMap::new();

    let _smth: Vec<_> = roadsigns
        .lines()
        .map(|line| {
            let (node_name, targets) = line.split_once(" = ").unwrap();
            let tmp = targets
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();

            tree.entry(node_name).or_insert((tmp.0, tmp.1));
            (node_name, tmp)
        })
        .collect();

    tree
}

fn step_counter(input: &str, part2_switch: bool) -> u64 {
    let (instruction, roadsigns) = input.split_once("\n\n").unwrap();
    let tree = parse_tree(roadsigns);
    let instructions: Vec<char> = instruction.chars().collect();
    let instr_len = instructions.len();

    let starting_node_matcher: &str = match part2_switch {
        false => "AAA",
        true => "A",
    };

    // Get all starting nodes (ending with 'A')
    let starting_nodes: Vec<&str> = tree
        .keys()
        .filter(|k| k.ends_with(starting_node_matcher))
        .cloned()
        .collect();

    // Calculate cycle length for each starting node
    let cycle_lengths: Vec<u64> = starting_nodes
        .par_iter()
        .map(|&start_node| {
            let mut current = start_node;
            let mut steps = 0u64;

            // Keep going until we reach a node ending with 'Z'
            while !current.ends_with("Z") {
                let direction = instructions[steps as usize % instr_len];
                let next_nodes = tree.get(current).unwrap();

                current = match direction {
                    'L' => next_nodes.0,
                    'R' => next_nodes.1,
                    _ => panic!("Invalid direction"),
                };

                steps += 1;
            }

            steps
        })
        .collect();

    // Calculate LCM of all cycle lengths
    cycle_lengths.into_iter().fold(1, lcm)
}

#[cfg(test)]
mod tests {

    static TEST_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static TEST_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    static TEST_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    use super::*;

    #[test]
    fn part2() {
        assert_eq!(2, step_counter(TEST_1, false));
        assert_eq!(6, step_counter(TEST_2, false));
        assert_eq!(6, step_counter(TEST_3, true));
    }
}

pub fn day08() {
    let _input = include_str!("input.txt");
    println!("Day08 part1: {:?}", step_counter(_input, false)); // part 1 12361
    println!("Day08 part2: {:?}", step_counter(_input, true)); // part 2 18215611419223 - was always going with the wrong approach - it was about least common multiple
}
