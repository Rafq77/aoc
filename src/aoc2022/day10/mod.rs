fn parse_instruction(instructions: &str) -> Vec<(&str, i32)> {
    instructions
        .lines()
        .map(|l| {
            let instr = l
                .split_once(' ')
                .map(|(d, n)| (d, n.parse::<i32>().unwrap_or(0)))
                .unwrap_or(("noop", 0));
            instr
        })
        .collect::<Vec<_>>()
}

fn part1n2(cmds: Vec<(&str, i32)>) -> (i32, i32) {
    let strengths = [20, 60, 100, 140, 180, 220];
    let mut set = std::collections::HashMap::new();
    let mut cycle = 0;
    let mut reg_x = 1;
    let mut printer = 0;
    let mut crt = String::new();
    for instr in cmds {
        match instr.0 {
            "noop" => cycle += 1,
            "addx" => cycle += 2,
            &_ => todo!(),
        }

        if strengths.contains(&cycle) || strengths.contains(&(cycle - 1)) {
            let mut tmpCycle = cycle;

            if cycle % 2 == 1 {
                tmpCycle = cycle - 1;
            }

            let mul = tmpCycle * reg_x;
            set.entry(tmpCycle).or_insert(mul);

            //println!("cyc: {tmpCyc:3} regX: {reg_x:2} - mul: {mul}");
        }

        // part2
        let double = match instr.0 {
            "noop" => 1,
            "addx" => 2,
            &_ => todo!(),
        };
        for _ in 0..double {
            if [reg_x - 1, reg_x, reg_x + 1].contains(&printer) {
                crt += "#";
            } else {
                crt += ".";
            }
            printer += 1;

            if printer >= 40 {
                crt += "\n";
                printer = 0;
            }
        }

        reg_x += instr.1; // add after cycle
    }

    println!("{crt}");
    (set.values().sum(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INSTR_SMALL: &str = "noop
addx 3
addx -5";

    static INSTR_BIG: &str = include_str!("example_big.txt");

    #[test]
    fn example_part1() {
        assert_eq!((13140, 0), part1n2(parse_instruction(INSTR_BIG)));
        assert_eq!((0, 0), part1n2(parse_instruction(INSTR_SMALL)));
    }
}

pub fn day10() {
    let _input = include_str!("input.txt");
    println!("Day10 answers: {:?}", part1n2(parse_instruction(_input))); // 16480, PLEFULPB
}
