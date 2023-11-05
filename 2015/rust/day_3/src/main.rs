use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::path::Path;

fn main() {
    let instructions = map_instructions();
    let part_1 = part_1(&instructions);
    println!("{part_1}");

    let part_2 = part_2(&instructions);
    println!("{part_2}");
}

fn part_1(instructions: &Vec<Point>) -> usize {
    let mut location = Point { x: 0, y: 0 };
    let mut visited: HashSet<Point> = HashSet::from([location]);
    for instruction in instructions {
        location += *instruction;
        visited.insert(location);
    }
    visited.len()
}

fn part_2(instructions: &Vec<Point>) -> usize {
    let (mut santa, mut robot_santa) = (Point { x: 0, y: 0 }, Point { x: 0, y: 0 });
    let mut visited: HashSet<Point> = HashSet::from([Point { x: 0, y: 0 }]);
    for (i, instruction) in instructions.iter().enumerate() {
        match i % 2 {
            0 => {
                santa += *instruction;
                visited.insert(santa);
            }
            1 => {
                robot_santa += *instruction;
                visited.insert(robot_santa);
            }
            _ => {}
        }
    }
    visited.len()
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn map_instructions() -> Vec<Point> {
    let lines = lines_from_file("input.txt");
    let mut instructions: Vec<Point> = Vec::new();
    for line in lines {
        line.chars().for_each(|c| match c {
            '^' => instructions.push(Point { x: 0, y: 1 }),
            'v' => instructions.push(Point { x: 0, y: -1 }),
            '>' => instructions.push(Point { x: 1, y: 0 }),
            '<' => instructions.push(Point { x: -1, y: 0 }),
            _ => panic!("Expected one of 'v', '>', '^', '<', got {c}"),
        });
    }
    instructions
}

#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
