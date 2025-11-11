use std::io::Error;

pub fn both_parts(input: String) -> (i32, i32) {
    let mut bingo_game = parse_input(&input);
    let mut first_winner = -1;
    let mut last_winner = -1;
    let total_bingo_cards = bingo_game.bingo_boards.len();

    let mut completed_bingo_cards = vec![false; bingo_game.bingo_boards.len()];

    for number in bingo_game.numbers_to_draw {
        for (index, board) in &mut bingo_game.bingo_boards.iter_mut().enumerate() {
            board.draw_number(number);
            if board.check_bingo() {
                if first_winner == -1 {
                    first_winner = board.get_sum_of_unmarked() * number;
                }

                if completed_bingo_cards
                    .iter()
                    .filter(|p| p.eq(&&true))
                    .count()
                    == total_bingo_cards - 1
                {
                    last_winner = board.get_sum_of_unmarked() * number;
                }

                completed_bingo_cards[index] = true;
            }
        }
    }

    (first_winner, last_winner)
}

const BOARD_SIZE: usize = 5;

struct BingoBoard {
    // 2d array 5x5
    board: Vec<Vec<i32>>,
    // how to mark numbers that have been drawn
    marked: Vec<Vec<bool>>,
    fast_access: std::collections::HashMap<i32, (usize, usize)>,
}

impl BingoBoard {
    fn new(board: Vec<Vec<i32>>) -> Self {
        assert!(board.len() == BOARD_SIZE);
        for row in &board {
            assert!(row.len() == BOARD_SIZE);
        }

        let mut map = std::collections::HashMap::new();

        for (i, _) in board.iter().enumerate().take(BOARD_SIZE) {
            // Fancy for old school: for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                map.insert(board[i][j], (i, j));
            }
        }

        Self {
            board,
            marked: vec![vec![false; BOARD_SIZE]; BOARD_SIZE],
            fast_access: map,
        }
    }

    fn draw_number(&mut self, number: i32) {
        if let Some(&(i, j)) = self.fast_access.get(&number) {
            self.marked[i][j] = true;
        }
    }

    fn check_bingo(&self) -> bool {
        // check rows
        let mut bingo = self
            .marked
            .iter()
            .any(|row| row.iter().all(|v| v.eq(&true)));

        // check column
        bingo = bingo
            || self
                .marked
                .iter()
                .enumerate()
                .any(|(j, _)| self.marked.iter().all(|row| row[j].eq(&true)));

        bingo
    }

    fn get_sum_of_unmarked(&self) -> i32 {
        self.board
            .iter()
            .zip(&self.marked)
            .map(|(row, marked_row)| {
                row.iter()
                    .zip(marked_row)
                    .filter_map(
                        |(&val, &is_marked)| {
                            if !is_marked {
                                Some(val)
                            } else {
                                None
                            }
                        },
                    )
                    .sum::<i32>()
            })
            .sum()
    }
}

struct BingoGame {
    bingo_boards: Vec<BingoBoard>,
    numbers_to_draw: Vec<i32>,
}

fn parse_input(input: &str) -> BingoGame {
    let groups: Vec<&str> = input.split("\n\n").collect();
    assert!(groups.len() > 1);

    let boards: Vec<BingoBoard> = groups
        .iter()
        .skip(1)
        .map(|group| {
            let tmp = group
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect()
                })
                .collect();

            BingoBoard::new(tmp)
        })
        .collect();

    BingoGame {
        bingo_boards: boards,
        numbers_to_draw: groups[0]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASE: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    fn get_test_bingo_board() -> Vec<Vec<i32>> {
        vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ]
    }

    #[test]
    fn test_bingo_board() {
        let board = BingoBoard::new(get_test_bingo_board());

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                assert!(board.marked[i][j] == false);
            }
        }

        assert!(board.board[4][4] == 25);
        assert!(board.board[2][3] == 14);
    }

    #[test]
    fn test_bingo_board_winning_row() {
        let mut board = BingoBoard::new(get_test_bingo_board());
        let mut board2 = BingoBoard::new(get_test_bingo_board());

        assert!(board.check_bingo() == false);
        board.draw_number(1);
        assert!(board.check_bingo() == false);
        board.draw_number(2);
        assert!(board.check_bingo() == false);
        board.draw_number(3);
        assert!(board.check_bingo() == false);
        board.draw_number(4);
        assert!(board.check_bingo() == false);
        board.draw_number(5);
        assert!(board.check_bingo() == true);

        assert!(board.get_sum_of_unmarked().eq(&310));

        board2.draw_number(16);
        assert!(board2.check_bingo() == false);
        board2.draw_number(20);
        board2.draw_number(19);
        board2.draw_number(17);
        board2.draw_number(18);
        assert!(board2.check_bingo() == true);
    }

    #[test]
    fn test_bingo_board_winning_column() {
        let mut board = BingoBoard::new(get_test_bingo_board());

        assert!(board.check_bingo() == false);
        board.draw_number(1);
        assert!(board.check_bingo() == false);
        board.draw_number(6);
        assert!(board.check_bingo() == false);
        board.draw_number(11);
        assert!(board.check_bingo() == false);
        board.draw_number(16);
        assert!(board.check_bingo() == false);
        board.draw_number(21);
        assert!(board.check_bingo() == true);
    }

    #[test]
    fn test_test_input() {
        let bingo_game = parse_input(TEST_CASE);
        assert!(bingo_game.numbers_to_draw.first().unwrap().eq(&7));
        assert!(bingo_game.numbers_to_draw[1] == 4);
        assert!(bingo_game.numbers_to_draw.last().unwrap().eq(&1));

        assert!(bingo_game.bingo_boards.len() == 3);
        assert!(bingo_game.bingo_boards.first().unwrap().board[1][2] == 23);
    }

    #[test]
    fn test_both_parts() {
        // Typical bingo, where row or column win the game. No diagonals!
        // Winning number is determined by the sum of UNMARKED number multiplied by the last winning number.

        assert_eq!(both_parts(TEST_CASE.to_string()), (4512, 1924));
    }
}

pub fn day04() -> Result<(), Error> {
    let input = include_str!("input.txt").to_string();
    let (part1, part2) = both_parts(input.clone());
    println!("Day04 part1: {0}", part1); // 35711
    println!("Day04 part2: {0}", part2); // 5586
    Ok(())
}
