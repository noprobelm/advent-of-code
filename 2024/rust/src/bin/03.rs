use aoc::PuzzleInput;
use regex::Regex;

fn main() {
    let p = PuzzleInput::new("../data/3.txt");
    let s = p.string();

    let part_1 = part_1(s);
    println!("Part 1: {part_1}");

    // let part_2 = part_2(&lines);
    // println!("Part 2: {part_2}");
}

fn part_1(puzzle_input_str: &str) -> u32 {
    let re = Regex::new(r#"mul\((\d+,\d+)\)"#).unwrap();
    re.captures_iter(puzzle_input_str).fold(0, |acc, m| {
        acc + m.get(1)
            .unwrap()
            .as_str()
            .split(",")
            .map(|n| n.parse::<u32>().expect("Invalid u32!"))
            .collect::<Vec<u32>>()
            .iter()
            .fold(1, |acc, n| acc * n)
    })
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;
    answer
}
