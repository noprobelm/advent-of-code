use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    // Placeholder for our puzzle answer
    let mut overlapping: u32 = 0;

    let lines = lines_from_file("input.txt");
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
    println!("{overlapping}")
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
