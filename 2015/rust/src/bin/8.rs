use aoc::PuzzleInput;
use regex::Regex;

fn main() {
    let p = PuzzleInput::new("../data/8.txt");
    let lines = p.lines();
    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

fn part_1(lines: &Vec<&str>) -> u32 {
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

fn part_2(lines: &Vec<&str>) -> u32 {
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
        // Manual counting of the quote characters + their encoded versions by starting len_encoded at 6
        let mut len_encoded = 6;
        let trimmed = &line[1..line.len() - 1];
        let len_str = re
            .split(trimmed)
            .filter(|&x| x.len() > 0)
            .fold(0, |acc, x| acc + x.len());

        len_code += len_str;
        len_encoded += len_str;

        let matches: Vec<_> = re.find_iter(trimmed).map(|m| m.as_str()).collect();
        for chars in matches.iter() {
            len_code += chars.len();
            match chars {
                &"\\\\" => len_encoded += 4,
                &"\\\"" => len_encoded += 4,
                &_ => len_encoded += 5,
            }
        }
        count += len_encoded - len_code;
    }
    count.try_into().unwrap()
}
