mod day01;

fn main() {
    day01::read_profile_file();
    let result_day01 = day01::solution_day01();
    println!("The solution for the puzzle of day 01 = '{}'!", result_day01)
}