use std::fs;
use std::path::Path;

fn read_input_file() -> String {
    let filename = Path::new("./data/day06.txt");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

pub fn solution_day06() {
    let input = read_input_file();
    let swarm = LanternFishSwarm::init(input.clone());
    let result_day06_part1 = swarm.simulation(80);
    println!(
        "The solution for the 1st part of the puzzle from day 06 is '{}'!",
        result_day06_part1.count
    );
    let swarm = LanternFishSwarm::init(input.clone());
    let result_day06_part1 = swarm.simulation(256);
    println!(
        "The solution for the 2nd part of the puzzle from day 06 is '{}'!",
        result_day06_part1.count
    );
}

#[derive(Debug, PartialEq)]
struct LanternFishSwarm {
    fishes: Vec<i32>,
    count: i32,
}

impl LanternFishSwarm {
    fn init(input: String) -> Self {
        let fishes = input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|f| f.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let count = fishes.len() as i32;
        LanternFishSwarm { fishes, count }
    }

    fn simulation(self, days: i32) -> LanternFishSwarm {
        let mut fishes_state = self.fishes.clone();
        for d in 0..days {
            println!("Calculate solution for day {}", d);
            let fishes_len = fishes_state.len();
            for index in 0..fishes_len {
                fishes_state[index] -= 1;
                if fishes_state[index] == -1 {
                    fishes_state[index] = 6;
                    fishes_state.push(8);
                }
            }
        }

        let fishes = fishes_state;
        let count = fishes.len() as i32;
        LanternFishSwarm { fishes, count }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_simulation_day01() {
        let input = "3,4,3,1,2".to_string();
        let after_day01 = vec![2, 3, 2, 0, 1];
        let swarm = LanternFishSwarm::init(input.clone());
        let simulation = swarm.simulation(1);
        assert_eq!(after_day01, simulation.fishes);
    }

    #[test]
    fn test_simulation_day02() {
        let input = "3,4,3,1,2".to_string();
        let after_day02 = vec![1, 2, 1, 6, 0, 8];
        let swarm = LanternFishSwarm::init(input.clone());
        let simulation = swarm.simulation(2);
        assert_eq!(after_day02, simulation.fishes);
    }

    #[test]
    fn test_simulation_day03() {
        let input = "3,4,3,1,2".to_string();
        let after_day03 = vec![0, 1, 0, 5, 6, 7, 8];
        let swarm = LanternFishSwarm::init(input.clone());
        let simulation = swarm.simulation(3);
        assert_eq!(after_day03, simulation.fishes);
    }

    #[test]
    fn test_simulation_day12() {
        let input = "3,4,3,1,2".to_string();
        let after_day12 = vec![5, 6, 5, 3, 4, 5, 6, 0, 0, 1, 5, 6, 7, 7, 7, 8, 8];
        let swarm = LanternFishSwarm::init(input.clone());
        let simulation = swarm.simulation(12);
        assert_eq!(after_day12, simulation.fishes);
    }

    #[test]
    fn test_simulation_day18() {
        let input = "3,4,3,1,2".to_string();
        let after_day18 = vec![
            6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
        ];
        let swarm = LanternFishSwarm::init(input.clone());
        let simulation = swarm.simulation(18);
        assert_eq!(after_day18, simulation.fishes);
    }
}
