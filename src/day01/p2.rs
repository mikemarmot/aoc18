use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day01.txt").unwrap());
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let res = calc(&input);
    println!("Result of day01 p2: {}", res);
}

fn calc(input: &Vec<String>) -> i32 {

    let mut freq: i32 = 0;
    let mut store: Vec<i32> = vec![0];

    let input = input.into_iter().map(|n| n.parse::<i32>().unwrap()).cycle();

    for n in input {
        freq += n;

        if store.contains(&freq) {
            return freq;
        } else {
            store.push(freq);
        }
    }
    
    42
}

mod tests {
    #[test]
    fn test_calc() {
        let input: Vec<String> = vec![
            String::from("+3"),
            String::from("+3"),
            String::from("+4"),
            String::from("-2"),
            String::from("-4"),
        ];

        assert_eq!(super::calc(&input), 10);
    }
}