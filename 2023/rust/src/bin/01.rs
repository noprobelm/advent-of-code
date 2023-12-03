use aoc::PuzzleInput;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let p = PuzzleInput::new("../data/1.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: TODO: Solve overlapping patterns at the end of a string, like 'twone', which our regex will only see as 'two'");
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;
    for line in lines {
        let nums: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let first = nums.first().unwrap();
        let last = nums.last().unwrap();
        answer += format!("{first}{last}")
            .parse::<u32>()
            .expect("Invalid u32");
    }
    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut answer: u32 = 0;
    let mapper: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("oneeight", "18"),
        ("twoone", "21"),
        ("threeeight", "38"),
        ("sevennine", "79"),
    ]);

    let mut pattern = mapper
        .keys()
        .map(|k| format!("({k})"))
        .collect::<Vec<String>>();
    for i in 1..=9 {
        pattern.push(format!("{i}"));
    }
    let re = Regex::new(pattern.join("|").as_str()).unwrap();
    for line in lines {
        let matches: Vec<&str> = re
            .find_iter(line)
            .map(|m| {
                if let Some(v) = mapper.get(m.as_str()) {
                    return *v;
                } else {
                    return m.as_str();
                }
            })
            .collect();
        let nums = format!("{}{}", matches.first().unwrap(), matches.last().unwrap());
        let n = nums
            .chars()
            .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap());

        answer += n
    }
    answer
}
