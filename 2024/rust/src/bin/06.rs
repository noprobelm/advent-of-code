use aoc::PuzzleInput;
use std::collections::{HashMap, HashSet};

fn main() {
    let p = PuzzleInput::new("../data/6.txt");
    let (guard, map) = parse_puzzle_input(&p);
    let part_1 = part_1(guard, map);
    println!("Part 1: {part_1}");
}

fn part_1(mut guard: Guard, map: HashMap<Position, char>) -> u32 {
    let mut visited: HashSet<Position> = HashSet::new();
    while map.contains_key(&guard.position) {
        visited.insert(guard.position.clone());
        guard.step(&map);
    }
    visited.len() as u32
}

fn parse_puzzle_input(p: &PuzzleInput) -> (Guard, HashMap<Position, char>) {
    let mut map: HashMap<Position, char> = HashMap::default();
    let mut guard = Guard::default();
    for (y, line) in p.lines().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let position = Position {
                x: x as isize,
                y: y as isize,
            };
            if c == '^' {
                guard.position = position.clone();
                map.insert(position, '.');
            } else {
                map.insert(position, c);
            }
        }
    }
    (guard, map)
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    x: isize,
    y: isize,
}

struct Map {
    map: HashMap<Position, char>,
}

impl Map {
    fn contains(&self, position: &Position) -> bool {
        self.map.contains_key(position)
    }
}

struct Guard {
    position: Position,
    movement_vector: Position,
}

impl Default for Guard {
    fn default() -> Self {
        Guard {
            position: Position { x: 0, y: 0 },
            movement_vector: Position { x: 0, y: -1 },
        }
    }
}

impl Guard {
    fn step(&mut self, map: &HashMap<Position, char>) {
        let next_position = Position {
            x: self.position.x + self.movement_vector.x,
            y: self.position.y + self.movement_vector.y,
        };
        if let Some(c) = map.get(&next_position) {
            match c {
                '.' => {
                    self.position = next_position;
                }
                '#' => self.turn(),
                _ => {
                    panic!("Unexpected character found in map: '{}'.", c)
                }
            }
        } else {
            self.position = next_position;
        }
    }

    fn turn(&mut self) {
        match self.movement_vector {
            Position { x: 0, y: -1 } => {
                self.movement_vector = Position { x: 1, y: 0 };
            }
            Position { x: 1, y: 0 } => {
                self.movement_vector = Position { x: 0, y: 1 };
            }
            Position { x: 0, y: 1 } => {
                self.movement_vector = Position { x: -1, y: 0 };
            }
            Position { x: -1, y: 0 } => {
                self.movement_vector = Position { x: 0, y: -1 };
            }
            _ => {
                panic!(
                    "Attempted to turn guard with invalid movement vector: '{:?}'",
                    self.movement_vector
                )
            }
        }
    }
}
