use std::fs;
use std::path::Path;

#[derive(Default)]
struct Bingo {
    boards: Vec<Board>,
    tips: Vec<u32>,
}

#[derive(Debug)]
struct Board {
    available_numbers: Vec<u32>,
    column_0: [u32; 5],
    column_1: [u32; 5],
    column_2: [u32; 5],
    column_3: [u32; 5],
    column_4: [u32; 5],
    row_0: [u32; 5],
    row_1: [u32; 5],
    row_2: [u32; 5],
    row_3: [u32; 5],
    row_4: [u32; 5],
}

fn read_input_file() -> Vec<String> {
    let filename = Path::new("./data/day04.txt");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let inputs: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    inputs
}

pub fn solution_day04() {
    let input = read_input_file();
    let tips: Vec<u32> = input
        .first()
        .unwrap()
        .split(",")
        .into_iter()
        .map(|f| f.parse::<u32>().unwrap())
        .collect();

    let mut input_board = input.iter();
    input_board.next();
    input_board.next();

    let mut boards: Vec<Board> = Vec::new();
    let mut board = Vec::new();
    for line in input_board {
        let entry =  line.trim().replace("  ", " ");
        if entry.len() > 1 {
            board.push(entry);
        }
    }
    for line in board.chunks(5) {
        println!("{:?}", line);
    }

    //let result_day03_part1 = power_supporting(&input);
    //println!("The solution for the 1st part of the puzzle from day 03 is '{}'!", result_day03_part1);
    //let result_day03_part2 = life_support_rating(&input);
    //println!("The solution for the 2nd part of the puzzle from day 03 is '{}'!", result_day03_part2);
}

impl Bingo {
    fn init(input: Vec<Vec<[&str; 5]>>, tips: Vec<u32>) -> Self {
        let mut new_bingo: Bingo = Bingo::default();
        new_bingo.tips = tips;

        for i in input {
            let board = Board::init_board(i);
            new_bingo.boards.push(board);
        }
        new_bingo
    }

    fn play(self) -> i32 {
        let tips_len = self.tips.len();
        for i in 5..tips_len + 1 {
            for board in self.boards.iter() {
                println!("{:?}", self.tips[0..i].to_vec());
                if board.validate_board(self.tips[0..i].to_vec()) {
                    return board.calculate_score(self.tips[0..i].to_vec(), self.tips[i - 1]);
                }
            }
        }
        0
    }
}

impl Board {
    fn init_board(input_board: Vec<[&str; 5]>) -> Self {
        let mut new_board = Board {
            available_numbers: vec![0; 25],
            column_0: [0; 5],
            column_1: [0; 5],
            column_2: [0; 5],
            column_3: [0; 5],
            column_4: [0; 5],
            row_0: [0; 5],
            row_1: [0; 5],
            row_2: [0; 5],
            row_3: [0; 5],
            row_4: [0; 5],
        };

        let flat_input_board: Vec<&str> =
            input_board.iter().flat_map(|x| x.iter()).cloned().collect();
        new_board.available_numbers = flat_input_board
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        for (index, row) in input_board.iter().enumerate() {
            let row_values = row.map(|x| x.parse::<u32>().unwrap());

            match index {
                0 => new_board.row_0 = row_values,
                1 => new_board.row_1 = row_values,
                2 => new_board.row_2 = row_values,
                3 => new_board.row_3 = row_values,
                4 => new_board.row_4 = row_values,
                _ => panic!(),
            }

            for col in 0..5 {
                let col_value = input_board[col][index].parse::<u32>().unwrap();
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

    fn calculate_score(&self, tips: Vec<u32>, last_input: u32) -> i32 {
        let values: Vec<u32> = self
            .available_numbers
            .iter()
            .filter(|f| !tips.contains(f))
            .copied()
            .collect();
        let sum: u32 = values.iter().sum();
        (sum * last_input) as i32
    }

    fn validate_board(&self, tips: Vec<u32>) -> bool {
        fn _validate_values(values: &Vec<u32>, tips: &Vec<u32>) -> bool {
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
    use super::*;

    #[test]
    fn test_board_invalid() {
        let input_tips = vec!["14", "21", "17", "24", "10"];
        let input_board = vec![
            ["14", "21", "17", "24", "4"],
            ["10", "16", "15", "9", "19"],
            ["18", "8", "23", "26", "20"],
            ["22", "11", "13", "6", "5"],
            ["2", "0", "12", "3", "7"],
        ];

        let board = Board::init_board(input_board);
        let tips = input_tips
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        assert_eq!(false, board.validate_board(tips));
    }

    #[test]
    fn test_board_valid() {
        let input_tips = vec!["14", "21", "17", "24", "4"];
        let input_board = vec![
            ["14", "21", "17", "24", "4"],
            ["10", "16", "15", "9", "19"],
            ["18", "8", "23", "26", "20"],
            ["22", "11", "13", "6", "5"],
            ["2", "0", "12", "3", "7"],
        ];

        let board = Board::init_board(input_board);
        let tips = input_tips
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        assert_eq!(true, board.validate_board(tips));
    }

    #[test]
    fn test_bingo() {
        let input_tips = vec![
            "7", "4", "9", "5", "11", "17", "23", "2", "0", "14", "21", "24", "10", "16", "13",
            "6", "15", "25", "12", "22", "18", "20", "8", "19", "3", "26", "1",
        ];
        let input_board1 = vec![
            ["22", "13", "17", "11", "0"],
            ["8", "2", "23", "4", "24"],
            ["21", "9", "14", "16", "7"],
            ["6", "10", "3", "18", "5"],
            ["1", "12", "20", "15", "19"],
        ];

        let input_board2 = vec![
            ["3", "15", "0", "2", "22"],
            ["9", "18", "13", "17", "5"],
            ["19", "8", "7", "25", "23"],
            ["20", "11", "10", "24", "4"],
            ["14", "21", "16", "12", "6"],
        ];

        let input_board3 = vec![
            ["14", "21", "17", "24", "4"],
            ["10", "16", "15", "9", "19"],
            ["18", "8", "23", "26", "20"],
            ["22", "11", "13", "6", "5"],
            ["2", "0", "12", "3", "7"],
        ];

        let tips = input_tips
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let bingo = Bingo::init(vec![input_board1, input_board2, input_board3], tips);
        let result = bingo.play();
        assert_eq!(4512, result)
    }
}
