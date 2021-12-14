use std::fs;
use std::path::Path;

fn read_input_file() -> String {
    let filename = Path::new("./data/day08.txt");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

pub fn solution_day08() {
    let input = read_input_file();
    let result_day08_part1 = Digits::init(input);
    println!(
        "The solution for the 1st part of the puzzle from day 08 is '{:?}'!",
        result_day08_part1
    );
}

#[derive(Debug, Clone)]
struct Digits {
    pattern: Vec<Vec<usize>>,
    // Mapping
    // 0 ... 6
    // 1 ... 2 (unique)
    // 2 ... 5 (NOT unique)
    // 3 ... 5 (NOT unique)
    // 4 ... 4 (unique)
    // 5 ... 5 (NOT unique)
    // 6 ... 6 (NOT unique)
    // 7 ... 3 (unique)
    // 8 ... 7 (unique)
    // 9 ... 6 (NOT unique)

    // unique => decode
    // NOT unique => -99
}

impl Digits {
    fn init(input: String) -> Self {
        println!("{}", input);
        let pattern = input
            .lines()
            .map(|x|
            x.split('|')
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.len())
            .collect::<Vec<usize>>()).collect::<Vec<_>>();
        
        println!("{:?}", pattern);
        Digits { pattern }
    }

    fn decode_digits(&self, line: usize) -> Vec<i8> {
        self.pattern[line]
            .iter()
            .map(|f| match f {
                2 => 1,
                4 => 4,
                7 => 3,
                8 => 7,
                _ => -99,
            })
            .collect::<Vec<i8>>()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_decode_digits() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
".to_string();
        let values = Digits::init(input);
        let line_00 = values.decode_digits(0);
        assert_eq!(vec![3, -99, -99, 4], line_00);
        let line_01 = values.decode_digits(1);
        assert_eq!(vec![-99, -99, 3, 1], line_01);
        let line_02 = values.decode_digits(2);
        assert_eq!(vec![1, 1, -99, -99], line_02);
        let line_03 = values.decode_digits(3);
        assert_eq!(vec![-99, -99, -99, 1], line_03);
    }
}
