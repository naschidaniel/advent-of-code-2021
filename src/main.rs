use std::env;
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u8>().unwrap();
    match day {
        1 => day01::solution_day01(),
        2 => day02::solution_day02(),
        3 => day03::solution_day03(),
        4 => day04::solution_day04(),
        5 => day05::solution_day05(),
        6 => day06::solution_day06(),
        7 => day07::solution_day07(),
        _ => panic!("There is no solution for this day {}!", args[1]),
    }
    let end_time = start_time.elapsed();
    println!("Duration {} in Seconds", end_time.as_secs());
}
