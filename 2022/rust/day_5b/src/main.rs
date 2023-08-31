use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input.txt");

    // Parse out the stack of crates and the moving instructions
    let (mut crates, instructions) = get_puzzle_parts(lines);

    // Each step is a tuple of usizes equal to "num to move", "from", and "to"
    let mut queue: Vec<char> = Vec::new();
    for step in instructions {
        // First collect the crates from the stack
        for _i in 0..step.0 {
            queue.push(crates[step.1 - 1].pop().unwrap());
        }

        // Now dump them on the new stack
        for _i in 0..step.0 {
            crates[step.2 - 1].push(queue.pop().unwrap());
        }
    }

    let mut answer: String = String::new();
    crates
        .iter()
        .for_each(|x| answer.push(*x.last().expect("Empty stack!")));

    println!("{answer}")
}

fn get_puzzle_parts(lines: Vec<String>) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let separator = lines
        .iter()
        .position(|x| x.is_empty())
        .expect("No empty line separating the puzzle input was found!");

    let crates_unparsed = &lines[..separator];
    let instructions_unparsed = &lines[separator + 1..];

    let crates = parse_crates(crates_unparsed.to_vec());
    let instructions = parse_instructions(instructions_unparsed.to_vec());

    (crates, instructions)
}

fn parse_crates(mut crates: Vec<String>) -> Vec<Vec<char>> {
    // Remove the index, we don't need it.
    crates.pop();

    let mut rows: Vec<Vec<char>> = Vec::new();

    // Pad each row so we have a uniform 2d structure that can be transposed
    let max_width = crates.last().expect("Empty vec!").len();
    for row in crates {
        let padded_right = format!("{:max_width$}", row);
        let row: Vec<char> = padded_right
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
    let mut rows: Vec<Vec<char>> = transpose(rows);

    // Finally, remove erroneous whitespace
    for row in &mut rows {
        row.retain(|x| !x.is_whitespace());
        row.reverse();
    }

    rows
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_instructions(instructions: Vec<String>) -> Vec<(usize, usize, usize)> {
    let mut cleaned: Vec<(usize, usize, usize)> = Vec::new();
    for step in instructions {
        let parsed: Vec<usize> = step
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        cleaned.push((parsed[0], parsed[1], parsed[2]))
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
