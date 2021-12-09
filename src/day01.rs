
use std::fs;
use std::path::Path;

pub fn read_profile_file() -> Vec<u32> {
    let filename = Path::new("./data/day01.txt");
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let depths: Vec<u32> = contents.split("\n").map(|x| x.parse::<u32>().unwrap()).collect();
    return depths
}

pub fn solution_day01() -> i32 {
    let depths = read_profile_file();
    return increase_values(&depths);
}
pub fn increase_values(depths: &[u32]) -> i32 {
    let mut increased_measurements = 0; 
    for (index, _depth) in depths.iter().enumerate() {
        if index == 0 {
            continue;
        }
        if depths[index - 1] < depths[index] {
            increased_measurements += 1;
        }
    }
    return increased_measurements;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_day01_increase() {
        let result = increase_values(&[199, 200]);
        assert_eq!(1, result);
    }

    #[test]
    fn test_solution_day01_decrease() {
        let result = increase_values(&[200, 199]);
        assert_eq!(0, result);
    }

    #[test]
    fn test_solution_day01_example() {
        let result = increase_values(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(7, result);
    }
}