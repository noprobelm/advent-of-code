use crate::filesystem::filesystem::FileSystem;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub mod filesystem;

fn main() {
    let lines = lines_from_file("input.txt");
    let mut fs: FileSystem = FileSystem::new();
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

    let dir_sizes = fs.du();
    let mut puzzle_answer: u64 = 0;
    for (_nx, size) in dir_sizes {
        if size <= 100000 {
            puzzle_answer += size
        }
    }
    println!("The sum of all directories with a size <= 100000 is '{puzzle_answer}'");
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
