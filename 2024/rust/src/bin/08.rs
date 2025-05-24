use aoc::PuzzleInput;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let p = PuzzleInput::new("../data/8.txt");
    let part_1 = part_1(&p);
    println!("Part 1: {}", part_1);
    let part_2 = part_2(&p);
    println!("Part 2: {}", part_2);
}

fn part_1(puzzle_input: &PuzzleInput) -> usize {
    let matrix = Matrix::from_chars(puzzle_input.lines());
    let mut antinodes: HashSet<Position> = HashSet::new();
    matrix.iter().for_each(|antenna| {
        matrix.iter().for_each(|other| {
            if antenna.variant != '.'
                && antenna.variant == other.variant
                && antenna.position != other.position
            {
                let diff = other.position - antenna.position;
                let first_antinode = other.position + diff;
                if matrix.contains(first_antinode) {
                    antinodes.insert(first_antinode);
                }
                let second_antinode = antenna.position - diff;
                if matrix.contains(second_antinode) {
                    antinodes.insert(second_antinode);
                }
            }
        });
    });
    antinodes.len()
}

fn part_2(puzzle_input: &PuzzleInput) -> usize {
    let matrix = Matrix::from_chars(puzzle_input.lines());
    let mut antinodes: HashSet<Position> = HashSet::new();
    matrix.iter().for_each(|antenna| {
        matrix.iter().for_each(|other| {
            if antenna.variant != '.'
                && antenna.variant == other.variant
                && antenna.position != other.position
            {
                let diff = other.position - antenna.position;
                let mut first_antinode = antenna.position + diff;
                while matrix.contains(first_antinode) {
                    antinodes.insert(first_antinode);
                    first_antinode -= diff;
                }
                let mut second_antinode = other.position - diff;
                while matrix.contains(second_antinode) {
                    antinodes.insert(second_antinode);
                    second_antinode += diff;
                }
            }
        });
    });
    antinodes.len()
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

    fn get(&self, pos: Position) -> Option<&Antenna> {
        if self.contains(pos) {
            Some(&self.matrix[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    fn insert(&mut self, antenna: Antenna) {
        if self.contains(antenna.position) {
            self.matrix[antenna.position.y as usize][antenna.position.x as usize] = antenna;
        }
    }

    fn strf(&self) -> String {
        let mut s = String::new();
        for row in &self.matrix {
            for antenna in row {
                s.push(antenna.variant);
            }
            s.push('\n');
        }
        s
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

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        self.x += other.x;
        self.y += other.y;
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

impl std::ops::SubAssign for Position {
    fn sub_assign(&mut self, other: Position) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Mul<i32> for Position {
    type Output = Position;

    fn mul(self, other: i32) -> Position {
        Position {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl std::ops::MulAssign<i32> for Position {
    fn mul_assign(&mut self, other: i32) {
        self.x *= other;
        self.y *= other;
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
