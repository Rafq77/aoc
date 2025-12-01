use itertools::Itertools;

fn dial(input: &str, flag: bool) -> i32 {
    let mut dial = 50; // we start at 50
    let mut click = 0;

    let instructions = input
        .lines()
        .map(|command| {
            let (dir, value) = command.split_at(1);
            let value: i32 = value.parse().unwrap_or(0);

            match dir {
                "L" => -value,
                "R" => value,
                _ => 0,
            }
        })
        .collect_vec();

    for instr in instructions {
        let start = dial;
        let delta = instr;
        let end = start + delta;

        if flag {
            // Count zero crossings without simulating steps; exclude landing on 0 (handled below).
            let passes = if delta > 0 {
                (end - 1).div_euclid(100) - start.div_euclid(100)
            } else if delta < 0 {
                (start - 1).div_euclid(100) - end.div_euclid(100)
            } else {
                0
            };
            click += passes;
        }

        dial = end.rem_euclid(100);

        if dial == 0 {
            click += 1;
        }
    }
    click
}

#[cfg(test)]
mod tests {
    use super::*;
    static INSTR: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn examples() {
        assert_eq!(3, dial(INSTR, false));
        assert_eq!(6, dial(INSTR, true));
    }
}

pub fn day01() {
    let _input = include_str!("input.txt");
    println!("Day01 part1: {}", dial(_input, false)); // 1076 
    println!("Day01 part2: {}",  dial(_input, true)); //  6379
}
