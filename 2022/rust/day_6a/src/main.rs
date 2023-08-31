use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let line = lines_from_file("sample.txt");
    for i in 0..line.len() {
        let end = if i + 4 < line.len() + 1 { i + 4 } else { break };
        if contains_duplicates(&line[i..end]) == false {
            println!("{end}");
            break;
        }
    }
}

fn contains_duplicates(v: &str) -> bool {
    let set: HashSet<char> = HashSet::from_iter(v.chars());
    if set.len() == 4 {
        return false;
    }
    true
}

#[allow(unused)]
fn lines_from_file(filename: impl AsRef<Path>) -> String {
    let file = File::open(filename).expect("no such file");
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    buf.read_line(&mut line);

    line.trim().to_string()
}
