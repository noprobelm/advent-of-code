use crate::filesystem::filesystem::FileSystem;
use petgraph::graph::NodeIndex;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub mod filesystem;

fn main() {
    let lines = lines_from_file("input.txt");
    let mut fs: FileSystem = FileSystem::new(70000000);
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        if split[0] == "dir" {
            fs.mkdir(fs.working, split[1]);
        } else if split[0].chars().next().unwrap().is_digit(10) {
            fs.fallocate(
                fs.working,
                split[1],
                split[0].parse::<u64>().expect("Invalid u64!"),
            );
        } else if split[1] == "cd" {
            fs.cd(split[2]);
        }
    }

    let update_size: u64 = 30000000;

    let dir_sizes = fs.du();
    let used_space = dir_sizes.get(&fs.root).unwrap();
    let free_space = fs.capacity - used_space;
    let space_needed = update_size - free_space;
    let answer: u64 = dir_sizes
        .into_values()
        .filter(|x| *x > space_needed)
        .min()
        .unwrap();
    println!("{:?}", answer);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
