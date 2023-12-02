use aoc::PuzzleInput;

fn main() {
    let p = PuzzleInput::new("../data/2.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}")
}

fn part_1(lines: &Vec<&str>) -> u32 {
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
    score
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut score: u32 = 0;
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        let opponent_move = match split[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid move"),
        };

        let outcome = match split[1] {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome"),
        };

        let player_move = match outcome {
            Outcome::Win => opponent_move.beaten_by(),
            Outcome::Lose => opponent_move.beats(),
            Outcome::Draw => opponent_move,
        };

        score += player_move.score() + outcome.score();
    }
    score
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
