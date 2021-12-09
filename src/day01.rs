
use std::fs;
use std::path::Path;

fn read_profile_file() -> Vec<u32> {
    let filename = Path::new("./data/day01.txt");
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let depths: Vec<u32> = contents.split("\n").map(|x| x.parse::<u32>().unwrap()).collect();
    depths
}

fn increase_values(depths: &[u32]) -> i32 {
    let mut increased_measurements = 0; 
    for (index, _depth) in depths.iter().enumerate() {
        if index == 0 {
            continue;
        }
        if depths[index - 1] < depths[index] {
            increased_measurements += 1;
        }
    }
    increased_measurements
}

fn increase_values_moving_window(depths: &[u32]) -> i32 {
    let mut depths_moving_window = Vec::new();
    for (index, _depth) in depths.iter().enumerate() {
        if index < 2 {
            continue;
        }
        let sum = depths[index - 2] + depths[index - 1] + depths[index];
        depths_moving_window.push(sum)
    }
    increase_values(&depths_moving_window)
}

pub fn solution_day01() {
    let depths = read_profile_file();
    let result_day01_part1 = increase_values(&depths);
    println!("The solution for the 1st part of the puzzle from day 01 is '{}'!", result_day01_part1);

    let result_day01_part2 = increase_values_moving_window(&depths);
    println!("The solution for the 2nd part of the puzzle from day 01 is '{}'!", result_day01_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increase_values_increase() {
        let result = increase_values(&[199, 200]);
        assert_eq!(1, result);
    }

    #[test]
    fn test_increase_values_decrease() {
        let result = increase_values(&[200, 199]);
        assert_eq!(0, result);
    }

    #[test]
    fn test_increase_values_example() {
        let result = increase_values(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(7, result);
    }

    #[test]
    fn test_increase_values_moving_window_example() {
        let result = increase_values_moving_window(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(5, result);
    }
}