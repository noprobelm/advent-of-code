use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

struct Rucksack {
    contents: Vec<u32>,
}

impl Rucksack {
    fn combined_calories(&self) -> u32 {
        let mut combined: u32 = 0;
        for i in &self.contents {
            combined += i;
        }
        combined
    }
}

fn main() {
    let lines: Vec<String> = lines_from_file("input.txt");

    let rucksacks: Vec<Rucksack> = rucksacks_from_lines(lines);

    let mut combined: Vec<u32> = Vec::new();
    for rucksack in rucksacks {
        combined.push(rucksack.combined_calories())
    }

    let largest = find_largest(&combined);
    println!("The largest number of combined calories among the elves is: {largest}");

    let three_largest_combined = combine_three_largest(&combined);
    println!("The combined calories of the three elves carrying the most calories is: {three_largest_combined}");
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn rucksacks_from_lines(lines: Vec<String>) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    let mut contents: Vec<u32> = Vec::new();
    for i in lines {
        match i.parse() {
            Ok(num) => contents.push(num),
            Err(_) => {
                rucksacks.push(Rucksack {
                    contents: contents.clone(),
                });
                contents.clear();
            }
        }
    }
    rucksacks.push(Rucksack {
        contents: contents.clone(),
    });

    rucksacks
}

fn find_largest(calories: &Vec<u32>) -> &u32 {
    calories.iter().max().unwrap()
}

fn combine_three_largest(calories: &Vec<u32>) -> u32 {
    let mut sorted = calories.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    sorted[0..3].iter().sum()
}
