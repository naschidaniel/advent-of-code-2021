use std::fs;
use std::path::Path;

#[derive(Default)]
struct Bingo {
    boards: Vec<Board>,
    tips: Vec<i32>,
}

#[derive(Debug)]
struct Board {
    attempts: i32,
    available_numbers: Vec<i32>,
    column_0: Vec<i32>,
    column_1: Vec<i32>,
    column_2: Vec<i32>,
    column_3: Vec<i32>,
    column_4: Vec<i32>,
    row_0: Vec<i32>,
    row_1: Vec<i32>,
    row_2: Vec<i32>,
    row_3: Vec<i32>,
    row_4: Vec<i32>,
    score: i32,
}

fn read_input_file() -> Vec<String> {
    let filename = Path::new("./data/day04.txt");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let inputs: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    inputs
}

pub fn solution_day04() {
    let input = read_input_file();
    let tips: Vec<i32> = input
        .first()
        .unwrap()
        .split(",")
        .into_iter()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    let mut input_board = input.iter();
    input_board.next();
    input_board.next();

    let mut numbers = Vec::new();
    for line in input_board {
        if line.len() < 1 {
            continue;
        }
        let entry = line
            .trim()
            .replace("  ", " ")
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        numbers.push(entry);
    }
    let boards = numbers.chunks(5).collect::<Vec<_>>();
    let bingo_part1 = Bingo::init(boards.clone(), tips.clone());
    let result_day04_part1 = bingo_part1.play("best");
    println!(
        "The solution for the 1st part of the puzzle from day 04 is '{}'!",
        result_day04_part1
    );
    let bingo_part2 = Bingo::init(boards.clone(), tips.clone());
    let result_day04_part1 = bingo_part2.play("last");
    println!(
        "The solution for the 2nd part of the puzzle from day 04 is '{}'!",
        result_day04_part1
    );
}

impl Bingo {
    fn init(input: Vec<&[Vec<i32>]>, tips: Vec<i32>) -> Self {
        let mut new_bingo: Bingo = Bingo::default();
        new_bingo.tips = tips;

        for i in input {
            let board = Board::init_board(i);
            new_bingo.boards.push(board);
        }
        new_bingo
    }

    fn play(mut self, position: &str) -> i32 {
        let tips_len = self.tips.len();
        for i in 5..tips_len + 1 {
            for board in self.boards.iter_mut() {
                if board.score > 0 {
                    continue;
                }
                if board.validate_board(self.tips[0..i].to_vec()) {
                    let score = board.calculate_score(self.tips[0..i].to_vec(), self.tips[i - 1]);
                    board.score = score;
                    board.attempts = i.clone() as i32;
                }
            }
        }
        if position == "best" {
            self.get_best_board()
        } else if position == "last" {
            self.get_last_board()
        } else {
            panic!()
        }
    }

    fn get_best_board(mut self) -> i32 {
        self.boards
            .sort_by(|a, b| a.attempts.partial_cmp(&b.attempts).unwrap());
        self.boards[0].score
    }

    fn get_last_board(mut self) -> i32 {
        self.boards
            .sort_by(|a, b| a.attempts.partial_cmp(&b.attempts).unwrap());
        let len = self.boards.len() - 1;
        self.boards[len].score
    }
}

impl Board {
    fn init_board(input_board: &[Vec<i32>]) -> Self {
        let mut new_board = Board {
            attempts: 0,
            available_numbers: vec![0; 25],
            column_0: vec![0; 5],
            column_1: vec![0; 5],
            column_2: vec![0; 5],
            column_3: vec![0; 5],
            column_4: vec![0; 5],
            row_0: vec![0; 5],
            row_1: vec![0; 5],
            row_2: vec![0; 5],
            row_3: vec![0; 5],
            row_4: vec![0; 5],
            score: 0,
        };

        new_board.available_numbers = input_board.iter().flat_map(|x| x.iter()).cloned().collect();
        for (index, row) in input_board.iter().enumerate() {
            let row_values = row.to_owned();

            match index {
                0 => new_board.row_0 = row_values,
                1 => new_board.row_1 = row_values,
                2 => new_board.row_2 = row_values,
                3 => new_board.row_3 = row_values,
                4 => new_board.row_4 = row_values,
                _ => panic!(),
            }

            for col in 0..5 {
                let col_value = input_board[col][index].to_owned();
                match index {
                    0 => new_board.column_0[col] = col_value,
                    1 => new_board.column_1[col] = col_value,
                    2 => new_board.column_2[col] = col_value,
                    3 => new_board.column_3[col] = col_value,
                    4 => new_board.column_4[col] = col_value,
                    _ => panic!(),
                }
            }
        }
        new_board
    }

    fn calculate_score(&self, tips: Vec<i32>, last_input: i32) -> i32 {
        let values: Vec<i32> = self
            .available_numbers
            .iter()
            .filter(|f| !tips.contains(f))
            .copied()
            .collect();
        let sum: i32 = values.iter().sum();
        (sum * last_input) as i32
    }

    fn validate_board(&self, tips: Vec<i32>) -> bool {
        fn _validate_values(values: &Vec<i32>, tips: &Vec<i32>) -> bool {
            for v in values {
                if !&tips.contains(&v) {
                    return false;
                }
            }
            true
        }

        if _validate_values(&self.row_0.to_vec(), &tips) {
            return true;
        }
        if _validate_values(&self.row_1.to_vec(), &tips) {
            return true;
        }
        if _validate_values(&self.row_2.to_vec(), &tips) {
            return true;
        }
        if _validate_values(&self.row_3.to_vec(), &tips) {
            return true;
        }
        if _validate_values(&self.row_4.to_vec(), &tips) {
            return true;
        }
        if _validate_values(&self.column_0.to_vec(), &tips) {
            return true;
        }
        if _validate_values(&self.column_1.to_vec(), &tips) {
            return true;
        }
        if _validate_values(&self.column_2.to_vec(), &tips) {
            return true;
        }
        if _validate_values(&self.column_3.to_vec(), &tips) {
            return true;
        }
        if _validate_values(&self.column_4.to_vec(), &tips) {
            return true;
        }
        false
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_board_invalid() {
        let tips = vec![14, 21, 17, 24, 10];
        let input_board = &[
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7],
        ];

        let board = Board::init_board(input_board);

        assert_eq!(false, board.validate_board(tips));
    }

    #[test]
    fn test_board_valid() {
        let tips = vec![14, 21, 17, 24, 4];
        let input_board = &[
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7],
        ];

        let board = Board::init_board(input_board);
        assert_eq!(true, board.validate_board(tips));
    }

    #[test]
    fn test_bingo() {
        let tips = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let input_board1 = &[
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19],
        ];

        let input_board2 = &[
            vec![3, 15, 0, 2, 22],
            vec![9, 18, 13, 17, 5],
            vec![19, 8, 7, 25, 23],
            vec![20, 11, 10, 24, 4],
            vec![14, 21, 16, 12, 6],
        ];

        let input_board3 = &[
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7],
        ];

        let bingo_part1 = Bingo::init(vec![input_board1, input_board2, input_board3], tips.clone());
        let result = bingo_part1.play("best");
        assert_eq!(4512, result);

        let bingo_part2 = Bingo::init(vec![input_board1, input_board2, input_board3], tips.clone());
        let result = bingo_part2.play("last");
        assert_eq!(1924, result);
    }
}
