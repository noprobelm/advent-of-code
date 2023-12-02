use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub struct PuzzleInput {
    data: String,
}

impl PuzzleInput {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let data = fs::read_to_string(path)
            .expect("Unable to read file")
            .trim()
            .to_string();
        PuzzleInput { data }
    }

    pub fn string(&self) -> &str {
        self.data.as_str()
    }
    pub fn lines(&self) -> Vec<&str> {
        self.data.split("\n").collect()
    }
}
