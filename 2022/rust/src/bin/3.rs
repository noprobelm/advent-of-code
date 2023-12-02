use aoc::PuzzleInput;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let p = PuzzleInput::new("../data/3.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let priority_mapper = get_priority_mapper();
    let mut priorities: Vec<u32> = Vec::new();

    for line in lines {
        let len = line.len();
        let mid = len / 2;
        let compartment_1: String = line.chars().take(mid).collect();
        let compartment_2: String = line.chars().skip(mid).collect();

        let set_1: HashSet<char> = HashSet::from_iter(compartment_1.chars());
        let set_2: HashSet<char> = HashSet::from_iter(compartment_2.chars());
        let intersection: Vec<char> = set_1.intersection(&set_2).cloned().collect();

        // Our puzzle input guarantees we will only ever have 1 duplicate between the 1st and 2nd compartments
        priorities.push(*priority_mapper.get(&intersection[0]).unwrap());
    }

    let answer: u32 = priorities.iter().sum();
    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let priority_mapper = get_priority_mapper();
    let mut priorities: Vec<u32> = Vec::new();

    for (a, b, c) in lines.into_iter().tuples() {
        let set_1: HashSet<char> = HashSet::from_iter(a.chars());
        let set_2: HashSet<char> = HashSet::from_iter(b.chars());
        let set_3: HashSet<char> = HashSet::from_iter(c.chars());
        let intersection: HashSet<char> = set_1.intersection(&set_2).cloned().collect();
        let intersection: Vec<char> = intersection.intersection(&set_3).cloned().collect();

        // Our puzzle input guarantees we will only ever have 1 duplicate between each set of 3
        priorities.push(*priority_mapper.get(&intersection[0]).unwrap());
    }

    let answer: u32 = priorities.iter().sum();
    answer
}

fn get_priority_mapper() -> HashMap<char, u32> {
    let letters: Vec<char> = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect())
        .unwrap()
        .chars()
        .collect();
    let priorities: Vec<u32> = (1..=58).collect();

    let score_mapper: HashMap<char, u32> =
        letters.into_iter().zip(priorities.into_iter()).collect();

    score_mapper
}
