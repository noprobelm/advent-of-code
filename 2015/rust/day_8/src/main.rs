use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input.txt");
    let part_1 = part_1(&lines);
    println!("{part_1}")
}

fn part_1(lines: &Vec<String>) -> u32 {
    let mut count: usize = 0;
    /*
     * I'm having great difficulty getting the Regex crate to create a pattern that matches
     * double quote characters. We know every string has one each at the beginning and ending
     * of the String we're looking at, so we'll just count them manually
     */
    let pattern = r#"\\x..|\\\\|\"|\\""#;
    let re = Regex::new(pattern).unwrap();
    for line in lines {
        // Manual counting of the quote characters here by starting len_code at 2
        let mut len_code = 2;
        let trimmed = &line[1..line.len() - 1];
        let mut len_str = re
            .split(trimmed)
            .filter(|&x| x.len() > 0)
            .fold(0, |acc, x| acc + x.len());

        len_code += len_str;

        let matches: Vec<_> = re.find_iter(trimmed).map(|m| m.as_str()).collect();
        for chars in matches.iter() {
            len_str += 1;
            len_code += chars.len();
        }

        let difference = len_code - len_str;
        count += difference;
    }
    count.try_into().unwrap()
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
