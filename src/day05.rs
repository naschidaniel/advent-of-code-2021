use std::fs;
use std::path::Path;
use std::collections::HashSet;

fn read_input_file() -> String {
    let filename = Path::new("./data/day05.txt");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

pub fn solution_day05() {
    let input = read_input_file();
    let measurements = Measurements::init(input);
    let result_day05_part1 = measurements.count_points_bigger_than_2();
    println!("The solution for the 1st part of the puzzle from day 05 is '{}'!", result_day05_part1);
    // let result_day05_part2 = bingo_part2.play("last");
    // println!(
    //     "The solution for the 2nd part of the puzzle from day 05 is '{}'!",
    //     result_day05_part2
    // );
}

#[derive(Debug, PartialEq)]
struct Measurements {
    segments: Vec<Point>,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Measurements {
    fn init(input: String) -> Self {
        let mut valid_values: Vec<Point> = Vec::new();
        for line in input.lines() {
            let mut points = line.split(" -> ");
            let mut p1 = points.next().unwrap().split(",");
            let mut p2 = points.next().unwrap().split(",");

            let x1 = p1.next().unwrap().parse::<i32>().unwrap();
            let y1 = p1.next().unwrap().parse::<i32>().unwrap();
            let x2 = p2.next().unwrap().parse::<i32>().unwrap();
            let y2 = p2.next().unwrap().parse::<i32>().unwrap();

            if x1 == x2 || y1 == y2 {
                if x1 != x2 {
                    let range_x = match x1 < x2 {
                        true => x1..=x2,
                        false => x2..=x1,
                    };

                    for x in range_x {
                        valid_values.push(Point { x, y: y1 });
                    }
                }
                if y1 != y2 {
                    let range_y = match y1 < y2 {
                        true => y1..=y2,
                        false => y2..=y1,
                    };
                    for y in range_y {
                        valid_values.push(Point { x: x1, y });
                    }
                }
            }
        }
        valid_values.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        Measurements { segments: valid_values }
        
    }

    fn count_points_bigger_than_2(self) -> i32 {
        let segment_strings = self.segments.iter().map(|p| format!("{},{}", p.x, p.y)).collect::<Vec<String>>();
        let segment_hashmap: HashSet<String> = segment_strings.iter().cloned().collect();
        let mut count = 0;
        for element in segment_hashmap.iter() {
            if segment_strings.iter().filter(|n| n == &element).count() >= 2 {
                count += 1
            }
        }
        count
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_init_line_segment_x() {
        let line = "9,7 -> 7,7".to_string();
        let test_segment = Measurements{ segments: vec![
            Point { x: 7, y: 7 },
            Point { x: 8, y: 7 },
            Point { x: 9, y: 7 },
        ]};
        let segment = Measurements::init(line);
        assert_eq!(test_segment, segment)
    }

    #[test]
    fn test_init_line_segment_y() {
        let line = "1,1 -> 1,3".to_string();
        let test_segment = Measurements{ segments: vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 3 },
        ]};
        let segment = Measurements::init(line);
        assert_eq!(test_segment, segment)
    }

    #[test]
    fn test_measurement() {
        let lines = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2".to_string();
        let test_segment = Measurements{ segments: vec![
            Point { x: 0, y: 9 },
            Point { x: 0, y: 9 },
            Point { x: 1, y: 9 },
            Point { x: 1, y: 9 },
            Point { x: 1, y: 4 },
            Point { x: 2, y: 9 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 2, y: 9 },
            Point { x: 2, y: 4 },
            Point { x: 3, y: 9 },
            Point { x: 3, y: 4 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 9 },
            Point { x: 4, y: 4 },
            Point { x: 5, y: 9 },
            Point { x: 5, y: 4 },
            Point { x: 6, y: 4 },
            Point { x: 7, y: 4 },
            Point { x: 7, y: 0 },
            Point { x: 7, y: 1 },
            Point { x: 7, y: 2 },
            Point { x: 7, y: 3 },
            Point { x: 7, y: 4 },
            Point { x: 8, y: 4 },
            Point { x: 9, y: 4 },
        ]};
        let segment = Measurements::init(lines);
        assert_eq!(test_segment, segment)
    }

    #[test]
    fn count_points_bigger_than_2() {
        let lines = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2".to_string();
        let measurements = Measurements::init(lines);
        let result = measurements.count_points_bigger_than_2();
        assert_eq!(5, result)

    }
}
