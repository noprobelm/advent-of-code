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

        score = play(player, opponent);
        println!("{score}")
    }
    println!("{score}")
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

fn play(player: Move, opponent: Move) -> u32 {
    let mut player_points = match player {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    let opponent_points = match opponent {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    match (player, opponent) {
        (Move::Rock, Move::Rock)
            | (Move::Paper, Move::Paper)
            | (Move::Scissors, Move::Scissors) => player_points += 3,

        (Move::Paper, Move::)
    }


    if player_points > opponent_points {
        player_points += 6
    } else if player_points == opponent_points {
        player_points += 3
    } else {
        player_points += 0
    }
    player_points
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
