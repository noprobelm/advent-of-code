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
        let rucksack = Rucksack::new(line);
        let common = rucksack.find_common();
        priorities.push(*priority_mapper.get(&common).unwrap());
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

struct Rucksack {
    contents: String,
}

impl Rucksack {
    fn new(contents: String) -> Self {
        Rucksack { contents }
    }

    fn first_compartment(&self) -> String {
        let len = self.contents.len();
        let mid = len / 2;
        self.contents.chars().take(mid).collect()
    }

    fn second_compartment(&self) -> String {
        let len = self.contents.len();
        let mid = len / 2;
        self.contents.chars().skip(mid).collect()
    }

    fn find_common(&self) -> char {
        let first: HashSet<char> = HashSet::from_iter(self.first_compartment().chars());
        let second: HashSet<char> = HashSet::from_iter(self.second_compartment().chars());

        let duplicate: Vec<char> = first.intersection(&second).cloned().collect();

        // Our puzzle input guarantees no rucksack will contain more than 1 dup value
        duplicate[0]
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
