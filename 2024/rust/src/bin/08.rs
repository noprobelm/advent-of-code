use aoc::PuzzleInput;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    let p = PuzzleInput::new("../data/8.txt");
    let part_1 = part_1(&p);
    println!("Part 1: {}", part_1);
    let part_2 = part_2(&p);
    println!("Part 2: {}", part_2);
}

fn part_1(puzzle_input: &PuzzleInput) -> u32 {
    let matrix = Matrix::from_chars(puzzle_input.lines());
    let mut antinodes: HashSet<Position> = HashSet::new();
    matrix.iter().for_each(|antenna| {
        matrix.iter().for_each(|other| {
            if antenna.variant != '.'
                && antenna.variant == other.variant
                && antenna.position != other.position
            {
                let diff = antenna.position - other.position;
                let first_antinode = antenna.position
                    - Position {
                        x: diff.x * 2,
                        y: diff.y * 2,
                    };
                if matrix.contains(first_antinode) {
                    antinodes.insert(first_antinode);
                }
                let second_antinode = other.position - diff;
                if matrix.contains(second_antinode) {
                    antinodes.insert(second_antinode);
                }
            }
        });
    });
    println!("{:?}", antinodes.len());
    0
}

fn part_2(puzzle_input: &PuzzleInput) -> u32 {
    0
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct Matrix {
    xmax: i32,
    ymax: i32,
    matrix: Vec<Vec<Antenna>>,
}

impl Matrix {
    fn from_chars(matrix: Vec<&str>) -> Self {
        let xmax = matrix.len();
        let ymax = matrix[0].len();
        Matrix {
            xmax: xmax as i32,
            ymax: ymax as i32,
            matrix: matrix
                .iter()
                .enumerate()
                .map(|(row, s)| {
                    s.chars()
                        .enumerate()
                        .map(|(col, c)| {
                            Antenna::new(
                                Position {
                                    x: col as i32,
                                    y: row as i32,
                                },
                                c,
                            )
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Antenna> {
        self.matrix.iter().flat_map(|row| row.iter())
    }

    fn contains(&self, pos: Position) -> bool {
        pos.x < self.xmax && pos.y < self.ymax && pos.x >= 0 && pos.y >= 0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct Antenna {
    position: Position,
    variant: char,
}

impl Antenna {
    fn new(position: Position, variant: char) -> Self {
        Antenna { position, variant }
    }
}
