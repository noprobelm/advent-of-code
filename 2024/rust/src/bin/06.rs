use aoc::PuzzleInput;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign};

fn main() {
    let p = PuzzleInput::new("../data/6.txt");
    let (mut guard, map) = parse_puzzle_input(&p);
    let starting_position = guard.position;
    let part_1 = part_1(&mut guard, &map);
    guard.position = starting_position;
    guard.movement_vector = IVec2 { x: 0, y: -1 };
    let part_2 = part_2(&mut guard, &map);
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn part_1(guard: &mut Guard, map: &HashMap<IVec2, char>) -> u32 {
    let mut visited: HashSet<IVec2> = HashSet::new();
    while map.contains_key(&guard.position) {
        visited.insert(guard.position);
        guard.step(map);
    }
    visited.len() as u32
}

fn part_2(guard: &mut Guard, map: &HashMap<IVec2, char>) -> u32 {
    let mut n = 0;
    while map.contains_key(&guard.position) {
        let clockwise_search_vector = guard.movement_vector.rotate(Rotation::Clockwise);
        let clockwise_start_position = guard.position;
        if let Some(upper_right) =
            obstacle_search(clockwise_start_position, clockwise_search_vector, map)
        {
            let reflected_search_vector = guard.movement_vector.reflect();
            let reflected_start_position =
                guard.position + guard.movement_vector.rotate(Rotation::CounterClockwise);
            if let Some(lower_left) =
                obstacle_search(reflected_start_position, reflected_search_vector, map)
            {
                let lower_right = IVec2 {
                    x: upper_right.x,
                    y: lower_left.y,
                } + clockwise_search_vector.reflect()
                    + reflected_search_vector;
                if let Some(c) = map.get(&lower_right) {
                    if c == &'#' {
                        n += 1;
                    }
                } else if let Some(c) = map.get(&(guard.position + guard.movement_vector)) {
                    if c == &'#' {
                        n += 1;
                    }
                }
            }
        }
        guard.step(map);
    }
    n
}

fn obstacle_search(
    start: IVec2,
    search_vector: IVec2,
    map: &HashMap<IVec2, char>,
) -> Option<IVec2> {
    let mut position = start;
    while let Some(c) = map.get(&position) {
        if c == &'#' {
            return Some(position);
        }
        position += search_vector;
    }
    None
}

fn parse_puzzle_input(p: &PuzzleInput) -> (Guard, HashMap<IVec2, char>) {
    let mut map: HashMap<IVec2, char> = HashMap::default();
    let mut guard = Guard::default();
    for (y, line) in p.lines().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let position = IVec2 {
                x: x as isize,
                y: y as isize,
            };
            if c == '^' {
                guard.position = position;
                map.insert(position, '.');
            } else {
                map.insert(position, c);
            }
        }
    }
    (guard, map)
}

enum Rotation {
    Clockwise,
    CounterClockwise,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct IVec2 {
    x: isize,
    y: isize,
}

impl IVec2 {
    fn rotate(&self, rotation: Rotation) -> IVec2 {
        match rotation {
            Rotation::Clockwise => IVec2 {
                x: -self.y,
                y: self.x,
            },
            Rotation::CounterClockwise => IVec2 {
                x: self.y,
                y: -self.x,
            },
        }
    }

    fn reflect(&self) -> IVec2 {
        IVec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for IVec2 {
    type Output = IVec2;

    fn add(self, other: IVec2) -> IVec2 {
        IVec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for IVec2 {
    fn add_assign(&mut self, other: IVec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Guard {
    position: IVec2,
    movement_vector: IVec2,
}

impl Default for Guard {
    fn default() -> Self {
        Guard {
            position: IVec2 { x: 0, y: 0 },
            movement_vector: IVec2 { x: 0, y: -1 },
        }
    }
}

impl Guard {
    fn step(&mut self, map: &HashMap<IVec2, char>) -> &IVec2 {
        let next_position = IVec2 {
            x: self.position.x + self.movement_vector.x,
            y: self.position.y + self.movement_vector.y,
        };
        if let Some(c) = map.get(&next_position) {
            match c {
                '.' => {
                    self.position = next_position;
                }
                '#' => self.movement_vector = self.movement_vector.rotate(Rotation::Clockwise),
                _ => {
                    panic!("Unexpected character found in map: '{}'.", c)
                }
            }
        } else {
            self.position = next_position;
        }

        &self.position
    }
}
