use rayon::prelude::*;

type TreeMap<'a> = std::collections::HashMap<&'a str, (&'a str, &'a str)>;

fn parse_tree(roadsigns: &str) -> TreeMap {
    let mut tree = TreeMap::new();

    let _smth: Vec<_> = roadsigns
    .lines()
    .map(|line| {
        let (node_name, targets) = line.split_once(" = ").unwrap();
        let tmp = targets.trim_matches(|c| c== '(' || c == ')').split_once(", ").unwrap();

        tree.entry(node_name).or_insert((tmp.0, tmp.1));
        (node_name, tmp)
    }).collect();

    return tree;
}

fn step_counter(input: &str) -> u32 {
    let (instruction, roadsigns) = input.split_once("\n\n").unwrap();
    let tree = parse_tree(roadsigns);

    let mut walking_nodes: Vec<&&str> = tree.keys().to_owned().filter(|k| k.ends_with("AAA")).collect();
    let mut step_counter = 0;
    let mut it = instruction.chars().cycle();

    while !walking_nodes.iter().all(|&n| n.ends_with("ZZZ")) {
        let direction = it.next().unwrap();

        walking_nodes = walking_nodes.par_iter().map(|&node| {
            let current = tree.get(node).unwrap();
            match direction {
                'L' => &current.0,
                'R' => &current.1,
                _ => panic!("WTF")
            }
        }).collect();

        step_counter += 1;
    }
    step_counter
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
        assert_eq!(2, step_counter(TEST_1));
        assert_eq!(6, step_counter(TEST_2));
        //assert_eq!(6, step_counter(TEST_3));
    }
}

pub fn day08() {
    let _input = include_str!("input.txt");
    println!("Day08 part1: {:?}", step_counter(_input)); // 12361
}
