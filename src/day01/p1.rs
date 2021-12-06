use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day01.txt").unwrap());
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let res = calc(&input);
    println!("Result of day01 p1: {}", res);
}

fn calc(input: &Vec<String>) -> i32 {

    let mut res: i32 = 0;
    for n in input {
        res += n.parse::<i32>().unwrap();
    }
    res
}

mod tests {
    #[test]
    fn test_calc() {
        let input: Vec<String> = vec![
            String::from("+1"),
            String::from("+1"),
            String::from("+1"),

        ];

        assert_eq!(super::calc(&input), 3);
    }
}