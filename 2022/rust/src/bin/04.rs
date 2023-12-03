use aoc::PuzzleInput;
use std::collections::HashSet;

fn main() {
    let p = PuzzleInput::new("../data/4.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}
fn part_1(lines: &Vec<&str>) -> u32 {
    // Placeholder for our puzzle answer
    let mut overlapping: u32 = 0;

    for line in lines {
        let split: Vec<String> = line.split(",").map(String::from).collect();

        // The puzzle input guarantees we will only ever have 2 results from our split
        let range_1: Vec<u32> = split[0]
            .split("-")
            .map(|x| x.parse::<u32>().expect("Not a valid u32!"))
            .collect();
        let range_1: HashSet<u32> = (range_1[0]..=range_1[1]).collect();

        let range_2: Vec<u32> = split[1]
            .split("-")
            .map(|x| x.parse::<u32>().expect("Not a valid u32!"))
            .collect();
        let range_2: HashSet<u32> = (range_2[0]..=range_2[1]).collect();

        let intersection: Vec<u32> = range_1.intersection(&range_2).cloned().collect();

        if intersection == range_1.iter().cloned().collect::<Vec<u32>>()
            || intersection == range_2.iter().cloned().collect::<Vec<u32>>()
        {
            overlapping += 1
        }
    }
    overlapping
}

fn part_2(lines: &Vec<&str>) -> u32 {
    // Placeholder for our puzzle answer
    let mut overlapping: u32 = 0;

    for line in lines {
        let split: Vec<String> = line.split(",").map(String::from).collect();

        // The puzzle input guarantees we will only ever have 2 results from our split
        let range_1: Vec<u32> = split[0]
            .split("-")
            .map(|x| x.parse::<u32>().expect("Not a valid u32!"))
            .collect();

        let range_2: Vec<u32> = split[1]
            .split("-")
            .map(|x| x.parse::<u32>().expect("Not a valid u32!"))
            .collect();

        if range_1[0] <= range_2[1] && range_2[0] <= range_1[1] {
            overlapping += 1
        }
    }
    overlapping
}
