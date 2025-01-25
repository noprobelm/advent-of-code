use aoc::PuzzleInput;
use std::collections::{hash_map::Entry, HashMap, HashSet};
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

// This is a truly awful solution, but I need a break.
fn part_2(guard: &mut Guard, map: &HashMap<IVec2, char>) -> u32 {
    let starting_position = guard.position;
    let starting_movement_vector = IVec2 { x: 0, y: -1 };
    let mut obstacles: HashSet<IVec2> = HashSet::new();
    map.iter().enumerate().for_each(|(i, (k, v))| {
        println!("{i}");
        let mut shadow_map = map.clone();
        let mut shadow_guard = Guard {
            position: starting_position,
            movement_vector: starting_movement_vector,
        };
        let obstacle = *k;
        shadow_map.insert(obstacle, '#');

        let mut k = 0;
        while shadow_map.contains_key(&shadow_guard.position) {
            shadow_guard.step(&shadow_map);
            k += 1;
            if k > 16641 {
                obstacles.insert(obstacle);
                break;
            }
        }
    });
    obstacles.len().try_into().unwrap()
}

fn obstacle_search(
    start: IVec2,
    stop: Option<isize>,
    search_vector: IVec2,
    map: &HashMap<IVec2, char>,
) -> Option<IVec2> {
    let mut position = start;
    let mut i = 0;
    while let Some(c) = map.get(&position) {
        if stop.is_some() && i >= stop.unwrap() {
            break;
        }
        if c == &'#' {
            return Some(position);
        }
        position += search_vector;
        i += 1;
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

struct Box {
    upper_left: IVec2,
    upper_right: IVec2,
    lower_left: IVec2,
    lower_right: IVec2,
}

impl Box {
    fn new(upper_left: IVec2, upper_right: IVec2, lower_left: IVec2, lower_right: IVec2) -> Self {
        Box {
            upper_left,
            upper_right,
            lower_left,
            lower_right,
        }
    }
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
                '#' => self.movement_vector = self.movement_vector.rotate(Rotation::Clockwise),
                _ => {
                    // println!("Not rotating!;");
                    // println!("{:?}", c);
                    self.position = next_position;
                }
            }
        } else {
            self.position = next_position;
        }

        &self.position
    }
}
