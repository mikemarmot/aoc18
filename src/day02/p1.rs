use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day02.txt").unwrap());
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let res = calc(&input);
    println!("Result of day02 p1: {}", res);
}

fn calc(input: &Vec<String>) -> usize {

    let mut c2 = 0;
    let mut c3 = 0;

    for line in input {
        let mut cnt: HashMap<char, usize> = HashMap::new();
        line.chars().for_each(|c| *cnt.entry(c).or_insert(0) += 1);

        if cnt.values().any(|x| *x == 3) { c3 += 1 };
        if cnt.values().any(|x| *x == 2) { c2 += 1 };
    }

    c2 * c3
}

mod tests {
    #[test]
    fn test_calc() {
        let input: Vec<String> = vec![
            String::from("abcdef"),
            String::from("bababc"),
            String::from("abbcde"),
            String::from("abcccd"),
            String::from("aabcdd"),
            String::from("abcdee"),
            String::from("ababab"),
        ];

        assert_eq!(super::calc(&input), 12);
    }
}