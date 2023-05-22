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
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid move for player"),
        };

        score += play(player, opponent);
    }
    println!("{score}")
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

fn play(player: Outcome, opponent: Move) -> u32 {
    let score = match (player, opponent) {
        (Outcome::Win, Move::Rock) => 8,
        (Outcome::Draw, Move::Rock) => 4,
        (Outcome::Lose, Move::Rock) => 3,
        (Outcome::Win, Move::Paper) => 9,
        (Outcome::Draw, Move::Paper) => 5,
        (Outcome::Lose, Move::Paper) => 1,
        (Outcome::Win, Move::Scissors) => 7,
        (Outcome::Draw, Move::Scissors) => 6,
        (Outcome::Lose, Move::Scissors) => 2,
    };

    score
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
