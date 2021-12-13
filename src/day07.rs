use std::fs;
use std::path::Path;

fn read_input_file() -> String {
    let filename = Path::new("./data/day07.txt");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

pub fn solution_day07() {
    let input = read_input_file();
    let crabs = Crabs::init(input.clone());
    let mut costs = Vec::new();
    for i in 0..crabs.positions.len() {
        let new_costs = crabs.minimize_cost_for_position(i);
        costs.push(new_costs)
    }
    let result_day07_part1 = costs.iter().min().unwrap();
    println!(
        "The solution for the 1st part of the puzzle from day 07 is '{}'!",
        result_day07_part1
    );
}

#[derive(Debug, PartialEq)]
struct Crabs {
    positions: Vec<i32>,
}

impl Crabs {
    fn init(input: String) -> Self {
        let positions = input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|f| f.parse().unwrap())
            .collect::<Vec<i32>>();

        Crabs { positions }
    }

    fn minimize_cost_for_position(&self, position: usize) -> i32 {
        self.positions
            .iter()
            .map(|x| (x - position as i32).abs())
            .sum::<i32>()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_minimum_cost_positions_1() {
        let input = "16,1,2,0,4,2,7,1,2,14".to_string();
        let crabs = Crabs::init(input.clone());
        let result = crabs.minimize_cost_for_position(1);
        assert_eq!(41, result);
    }

    #[test]
    fn test_minimum_cost_positions_2() {
        let input = "16,1,2,0,4,2,7,1,2,14".to_string();
        let crabs = Crabs::init(input.clone());
        let result = crabs.minimize_cost_for_position(2);
        assert_eq!(37, result);
    }

    #[test]
    fn test_minimum_cost_positions_3() {
        let input = "16,1,2,0,4,2,7,1,2,14".to_string();
        let crabs = Crabs::init(input.clone());
        let result = crabs.minimize_cost_for_position(3);
        assert_eq!(39, result);
    }

    #[test]
    fn test_minimum_cost_positions_10() {
        let input = "16,1,2,0,4,2,7,1,2,14".to_string();
        let crabs = Crabs::init(input.clone());
        let result = crabs.minimize_cost_for_position(10);
        assert_eq!(71, result);
    }
}
