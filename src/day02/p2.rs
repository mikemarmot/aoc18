use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day02.txt").unwrap());
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let res = calc(&input);
    println!("Result of day02 p2: {:?}", res);
}

fn calc(input: &Vec<String>) -> Result<String, ()> {

    let input: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();

    for (i, a) in input.iter().enumerate() {
        
        for b in &input[i+1..] {
            let mut rest: Vec<char> = Vec::new();
            for j in 0..b.len() {
                if a[j] == b[j] { 
                    rest.push(a[j]); 
                }
            }
            if rest.len() == b.len() - 1 {
                return Ok(rest.iter().collect::<String>())
            }            
        }
    }

    Err(())
}

mod tests {

    #[test]
    fn test_calc() {
        let input: Vec<String> = vec![
            String::from("abcde"),
            String::from("fghij"),
            String::from("klmno"),
            String::from("pqrst"),
            String::from("fguij"),
            String::from("axcye"),
            String::from("wvxyz"),
        ];

        assert_eq!(super::calc(&input), Ok(String::from("fgij")));
    }
}