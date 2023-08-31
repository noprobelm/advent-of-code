use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input.txt");

    // Segments the stack of crates from the instructions
    let (mut crates, instructions) = get_puzzle_parts(lines);

    for step in instructions {
        for i in 0..step[0] {
            let item: char = crates[step[1] - 1].pop_front().unwrap();
            crates[step[2] - 1].push_front(item);
        }
    }

    let mut answer: String = String::new();
    crates
        .iter()
        .for_each(|x| answer.push(*x.front().expect("Empty stack!")));

    println!("{answer}")
}

fn get_puzzle_parts(lines: Vec<String>) -> (Vec<VecDeque<char>>, Vec<Vec<usize>>) {
    let separator = lines
        .iter()
        .position(|x| x.is_empty())
        .expect("No empty line found!");

    let crates_unparsed = &lines[..separator];
    let instructions_unparsed = &lines[separator + 1..];

    let crates = parse_crates(crates_unparsed.to_vec());
    let instructions = parse_instructions(instructions_unparsed.to_vec());

    (crates, instructions)
}

fn parse_crates(mut crates: Vec<String>) -> Vec<VecDeque<char>> {
    // Remove the index, we don't need it.
    crates.pop();

    let mut rows: Vec<VecDeque<char>> = Vec::new();

    // Pad each row so we have a uniform 2d structure
    let max_width = crates.last().expect("Empty vec!").len();
    for row in crates {
        let padded_right = format!("{:max_width$}", row);
        let row: VecDeque<char> = padded_right
            .chars()
            .skip(1)
            .step_by(2)
            .collect::<String>()
            .chars()
            .step_by(2)
            .collect();

        rows.push(row);
    }

    // Transpose our data to accurately represent the grid of crates
    let mut rows: Vec<VecDeque<char>> = transpose(rows);

    // Remove erroneous whitespace
    for row in &mut rows {
        row.retain(|x| !x.is_whitespace())
    }

    rows
}

fn transpose<T>(v: Vec<VecDeque<T>>) -> Vec<VecDeque<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<VecDeque<T>>()
        })
        .collect()
}

fn parse_instructions(instructions: Vec<String>) -> Vec<Vec<usize>> {
    let mut cleaned: Vec<Vec<usize>> = Vec::new();
    for step in instructions {
        let parsed: Vec<usize> = step
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        cleaned.push(parsed)
    }

    cleaned
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
