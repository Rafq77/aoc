fn parse_instruction(instructions: &str) -> Vec<(&str, i32)> {
    instructions
        .lines()
        .map(|l| {
            let instr = l
                .split_once(' ')
                .map(|(d, n)| (d, n.parse::<i32>().unwrap()))
                .unwrap();
            instr
        })
        .collect::<Vec<_>>()
}

fn part1n2(cmds: Vec<(&str, i32)>) -> (i32, i32) {
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut s = std::collections::HashSet::new();
    s.insert((0, 0));

    let mut knots: [(i32, i32); 10] = Default::default();
    let mut k9 = std::collections::HashSet::new();
    k9.insert((0, 0));

    for (dir, len) in cmds {
        let d = match dir {
            "L" => (-1, 0),
            "U" => (0, 1),
            "R" => (1, 0),
            "D" => (0, -1),
            &_ => todo!(),
        };

        for _ in 0..len {
            h = (h.0 + d.0, h.1 + d.1);
            knots[0] = h;

            if i32::abs_diff(h.0, t.0) > 1 || i32::abs_diff(h.1, t.1) > 1 {
                t = (h.0 - d.0, h.1 - d.1);
                s.insert(t); //part 1
            }

            for i in 1..knots.len() {
                let (left, right) = knots.split_at_mut(i);
                let current = left[i - 1];
                let next = &mut right[0];

                if i32::abs_diff(current.0, next.0) > 1 || i32::abs_diff(current.1, next.1) > 1 {
                    let dist = (next.0 - current.0, next.1 - current.1);
                    let len = i32::max(dist.0.abs(), dist.1.abs());
                    let bonus = (dist.0 / len, dist.1 / len);
                    *next = (current.0 + bonus.0, current.1 + bonus.1);
                }
            }

            //println!("knots rnd: {:?}", knots);
            k9.insert(*knots.last().unwrap()); // part2
        }
    }

    (s.len() as i32, k9.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    static DIRECTIONS_SMALL: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static DIRECTIONS_BIG: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn example_part1() {
        assert_eq!((13, 1), part1n2(parse_instruction(DIRECTIONS_SMALL)));
        assert_eq!(36, part1n2(parse_instruction(DIRECTIONS_BIG)).1);
    }
}

pub fn day09() {
    let _input = include_str!("input.txt");
    println!("Day09 answers: {:?}", part1n2(parse_instruction(_input))); // 6311, 2482
}
