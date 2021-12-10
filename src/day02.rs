
use std::fs;
use std::path::Path;

fn read_input_file() -> Vec<String> {
    let filename = Path::new("./data/day02.txt");
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let inputs: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    inputs
}

pub fn solution_day02() {
    let intput = read_input_file();
    let result_day02_part1 = move_position_and_aim(&intput, false);
    println!("The solution for the 1st part of the puzzle from day 02 is '{}'!", result_day02_part1);
    let result_day02_part2 = move_position_and_aim(&intput, true);
    println!("The solution for the 2nd part of the puzzle from day 02 is '{}'!", result_day02_part2);
}

fn move_position_and_aim(input: &Vec<String>, calculate_aim: bool) -> i32 {
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim = 0;
    for entry in input {
        let mut entry = entry.split_whitespace();
        let direction = entry.next().unwrap();
        let value = entry.next().unwrap().parse::<i32>().unwrap();

        if calculate_aim {
            match direction {
                "forward" => { 
                    depth = depth + aim * value; 
                    position = position + value;    
                },
                "up" => aim = aim - value,
                "down" => aim = aim + value,
                _ => panic!("The input is not supported!")
            }
        } else {
            match direction {
                "forward" => position = position + value,
                "up" => depth = depth - value,
                "down" => depth = depth + value,
                _ => panic!("The input is not supported!")
            }
        }
    }
    position * depth
}


mod tests {
    use super::*;

    #[test]
     fn test_move_position() {
        let input = ["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"];
        let input_converted = input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let result = move_position_and_aim(&input_converted, false);
        assert_eq!(150, result);
    }

    #[test]
    fn test_move_position_and_aim() {
        let input = ["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"];
        let input_converted = input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let result = move_position_and_aim(&input_converted, true);
        assert_eq!(900, result);
    }
}