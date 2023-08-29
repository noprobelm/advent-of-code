use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let lines: Vec<String> = lines_from_file("input.txt");
    let mut score = 0;
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        let opponent = match split[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid move for opponent"),
        };

        let player = match split[1] {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Invalid move for player"),
        };

        let outcome = match player {
            _ if player.beats() == opponent => Outcome::Win,
            _ if player.beaten_by() == opponent => Outcome::Lose,
            _ => Outcome::Draw,
        };

        score += outcome.score() + player.score();
    }
    println!("{score}")
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    const fn score(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    const fn beaten_by(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    const fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    const fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
