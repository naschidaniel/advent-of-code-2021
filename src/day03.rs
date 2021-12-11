use std::fs;
use std::path::Path;

fn read_input_file() -> Vec<String> {
    let filename = Path::new("./data/day03.txt");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let inputs: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    inputs
}

pub fn solution_day03() {
    let input = read_input_file();
    let result_day03_part1 = power_supporting(&input);
    println!(
        "The solution for the 1st part of the puzzle from day 03 is '{}'!",
        result_day03_part1
    );
    let result_day03_part2 = life_support_rating(&input);
    println!(
        "The solution for the 2nd part of the puzzle from day 03 is '{}'!",
        result_day03_part2
    );
}

fn most_and_least_common_value(position: Vec<char>) -> [&'static str; 2] {
    let mut ones = 0;
    let mut zeros = 0;

    for c in position.iter() {
        match c {
            '1' => ones += 1,
            '0' => zeros += 1,
            _ => panic!(),
        }
    }

    let mut value = "0";

    if ones >= zeros {
        value = "1";
    }

    let value_inverted = match value {
        "1" => "0",
        "0" => "1",
        _ => panic!(),
    };

    [value, value_inverted]
}

fn power_supporting(input: &Vec<String>) -> i32 {
    let input_length = input.first().unwrap().len();
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for index in 0..input_length {
        let mut values_at_position: Vec<char> = Vec::new();
        for measurement in input {
            let value = measurement.chars().nth(index).unwrap();
            values_at_position.push(value);
        }

        let [gamma_value_at_position, epsilon_value_at_position] =
            most_and_least_common_value(values_at_position);

        gamma_rate.push_str(gamma_value_at_position);
        epsilon_rate.push_str(epsilon_value_at_position);
    }

    let gamma_rate_decimal = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate_decimal = i32::from_str_radix(&epsilon_rate, 2).unwrap();

    gamma_rate_decimal * epsilon_rate_decimal
}

fn life_support_rating(input: &Vec<String>) -> i32 {
    let mut oxygen_generator_values = input.clone();
    let mut co2_scrubber_values = input.clone();

    let input_length = input.first().unwrap().len();
    for index in 0..input_length {
        let mut oxygen_values_at_position: Vec<char> = Vec::new();
        if oxygen_generator_values.len() <= 1 && co2_scrubber_values.len() <= 1 {
            break;
        }
        if oxygen_generator_values.len() > 1 {
            for measurement in oxygen_generator_values.iter() {
                let value = measurement.chars().nth(index).unwrap();
                oxygen_values_at_position.push(value);
            }
            let [most_common_value, _] = most_and_least_common_value(oxygen_values_at_position);
            oxygen_generator_values = oxygen_generator_values
                .into_iter()
                .filter(|x| {
                    x.chars()
                        .nth(index)
                        .unwrap()
                        .to_owned()
                        .to_string()
                        .as_str()
                        == most_common_value
                })
                .collect();
        }

        let mut co2_values_at_position: Vec<char> = Vec::new();
        if co2_scrubber_values.len() > 1 {
            for measurement in co2_scrubber_values.iter() {
                let value = measurement.chars().nth(index).unwrap();
                co2_values_at_position.push(value);
            }
            let [_, least_common_value] = most_and_least_common_value(co2_values_at_position);
            co2_scrubber_values = co2_scrubber_values
                .into_iter()
                .filter(|x| {
                    x.chars()
                        .nth(index)
                        .unwrap()
                        .to_owned()
                        .to_string()
                        .as_str()
                        == least_common_value
                })
                .collect();
        }
    }
    let oxygen_generator_rating_decimal =
        i32::from_str_radix(&oxygen_generator_values[0], 2).unwrap();
    let co2_scrubber_rating_decimal = i32::from_str_radix(&co2_scrubber_values[0], 2).unwrap();
    co2_scrubber_rating_decimal * oxygen_generator_rating_decimal
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_power_supporting() {
        let input = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let input_converted = input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let result = power_supporting(&input_converted);
        assert_eq!(198, result);
    }

    #[test]
    fn test_life_support_rating() {
        let input = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let input_converted = input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let result = life_support_rating(&input_converted);
        assert_eq!(230, result);
    }
}
