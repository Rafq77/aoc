use itertools::Itertools;

fn make_move_cmd(str: &str) -> (i32, i32, i32) {
    str.split_ascii_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect_tuple()
        .unwrap()
}

fn move_tab(_tab: &mut Vec<Vec<char>>, cmd: (i32, i32, i32), is9000: bool) {
    let from = (cmd.1 - 1) as usize;
    let to = (cmd.2 - 1) as usize;
    let n = cmd.0;

    match is9000 {
        true => {
            for _i in 0..n {
                let letter = _tab[from].pop().unwrap();
                _tab[to].push(letter);
            }
        }
        false => {
            let split_at = _tab[from].len() - (n as usize);
            let mut letter = _tab[from].split_off(split_at);
            _tab[to].append(&mut letter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        //manual interpert
        let mut table: Vec<Vec<_>> = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        move_tab(&mut table, (1, 2, 1), true);
        move_tab(&mut table, (3, 1, 3), true);
        move_tab(&mut table, (2, 2, 1), true);
        move_tab(&mut table, (1, 1, 2), true);

        let result = table
            .into_iter()
            .map(|col| col.last().unwrap().clone())
            .collect::<Vec<_>>();
        dbg!(result);
        // CMZ
    }

    #[test]
    fn cmd_translation_test() {
        let test = [
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        assert_eq!((1, 2, 1), make_move_cmd(test[0]));
        assert_eq!((3, 1, 3), make_move_cmd(test[1]));
        assert_eq!((2, 2, 1), make_move_cmd(test[2]));
        assert_eq!((1, 1, 2), make_move_cmd(test[3]));
    }
}

pub fn day05() {
    /* init =
    [N]             [R]             [C]
    [T] [J]         [S] [J]         [N]
    [B] [Z]     [H] [M] [Z]         [D]
    [S] [P]     [G] [L] [H] [Z]     [T]
    [Q] [D]     [F] [D] [V] [L] [S] [M]
    [H] [F] [V] [J] [C] [W] [P] [W] [L]
    [G] [S] [H] [Z] [Z] [T] [F] [V] [H]
    [R] [H] [Z] [M] [T] [M] [T] [Q] [W]
     1   2   3   4   5   6   7   8   9  */

    let mut table: Vec<Vec<_>> = vec![
        vec!['R', 'G', 'H', 'Q', 'S', 'B', 'T', 'N'],
        vec!['H', 'S', 'F', 'D', 'P', 'Z', 'J'],
        vec!['Z', 'H', 'V'],
        vec!['M', 'Z', 'J', 'F', 'G', 'H'],
        vec!['T', 'Z', 'C', 'D', 'L', 'M', 'S', 'R'],
        vec!['M', 'T', 'W', 'V', 'H', 'Z', 'J'],
        vec!['T', 'F', 'P', 'L', 'Z'],
        vec!['Q', 'V', 'W', 'S'],
        vec!['W', 'H', 'L', 'M', 'T', 'D', 'N', 'C'],
    ];
    let mut table_part2 = table.to_owned();

    let _input = include_str!("input.txt");
    let _cmds = _input.split_once("\n\n").unwrap().1;

    for line in _cmds.lines() {
        let cmd = make_move_cmd(line);
        move_tab(&mut table, cmd, true);
        move_tab(&mut table_part2, cmd, false);
    }

    let result_part1: String = table
        .into_iter()
        .map(|col| *col.last().unwrap())
        .collect::<Vec<_>>()
        .iter()
        .collect();

    let result_part2: String = table_part2
        .into_iter()
        .map(|col| *col.last().unwrap())
        .collect::<Vec<_>>()
        .iter()
        .collect();
    println!("Day05 part1: {}", result_part1); // part1 PTWLTDSJV
    println!("Day05 part2: {}", result_part2); // part2 WZMFVGGZP
}
