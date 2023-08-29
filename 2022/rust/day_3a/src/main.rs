use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let priority_mapper = get_priority_mapper();
    let lines: Vec<String> = lines_from_file("input.txt");
    let mut priorities: Vec<u32> = Vec::new();

    for line in lines {
        let len = line.len();
        let mid = len / 2;
        let first_compartment: String = line.chars().take(mid).collect();
        let second_compartment: String = line.chars().skip(mid).collect();

        let first_set: HashSet<char> = HashSet::from_iter(first_compartment.chars());
        let second_set: HashSet<char> = HashSet::from_iter(second_compartment.chars());
        let duplicate: Vec<char> = first_set.intersection(&second_set).cloned().collect();

        // Our puzzle input guarantees we will only ever have 1 duplicate between the 1st and 2nd compartments
        priorities.push(*priority_mapper.get(&duplicate[0]).unwrap());
    }

    let answer: u32 = priorities.iter().sum();
    println!("{answer}")
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

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
