use aoc::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let p = PuzzleInput::new("../data/1.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;
    for line in lines {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let digit = format!("{}{}", first, last);

        answer += digit.parse::<u32>().unwrap();
    }
    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut answer: u32 = 0;
    let patterns: [&str; 18] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let mapper: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for line in lines {
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut matches: Vec<u32> = Vec::new();
        while start < line.len() {
            if patterns.contains(&&line[start..=end]) {
                let matched = &line[start..=end];
                // We matched a word
                if let Some(val) = mapper.get(matched) {
                    matches.push(*val);
                    start = end;
                // We matched a digit
                } else {
                    matches.push(matched.parse::<u32>().expect("Invalid u32"));
                    start += 1;
                    end = start;
                }
            } else {
                end += 1;
            }
            if end == line.len() {
                start += 1;
                end = start;
            }
        }
        let n = [*matches.first().unwrap(), *matches.last().unwrap()]
            .iter()
            .fold(0, |acc, x| acc * 10 + x);
        println!("{line}: {:?}, {n}", matches);

        answer += n;
    }
    answer
}
