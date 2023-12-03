//! # Advent of Code - 2015 Day 3: Perfectly Spherical Houses in a Vacuum
//!
//! ## Problem Description: Part 1
//! Santa is delivering presents to an infinite two-dimensional grid of houses.
//!
//! He begins by delivering a present to the house at his starting location, and then an elf at the
//! North Pole calls him via radio and tells him where to move next. Moves are always exactly one
//! house to the north (^), south (v), east (>), or west (<). After each move, he delivers another
//! present to the house at his new location.
//!
//! However, the elf back at the north pole has had a little too much eggnog, and so his directions
//! are a little off, and Santa ends up visiting some houses more than once. How many houses receive
//! at least one present?
//!
//! For example:
//!
//! - `>` delivers presents to 2 houses: one at the starting location, and one to the east.
//! - `^>v<` delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
//! - `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at only 2 houses.

//! ## Problem Description: Part 2
//! The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to
//! deliver presents with him.
//!
//! Santa and Robo-Santa start at the same location (delivering two presents to the same starting
//! house), then take turns moving based on instructions from the elf, who is eggnoggedly reading
//! from the same script as the previous year.
//!
//! This year, how many houses receive at least one present?
//!
//! For example:
//! - `^v` delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
//! - `^>v<` now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
//! - `^v^v^v^v^v` now delivers presents to 11 houses, with Santa going one direction and Robo-Santa
//! going the other.

use aoc::PuzzleInput;
use std::collections::HashSet;
use std::ops::AddAssign;

/// Translates our puzzle input into a sequence of Points that are used in [part_1] and [part_2]
fn main() {
    let _p = PuzzleInput::new("../data/3.txt");

    let instructions = map_instructions();
    let part_1 = part_1(&instructions);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&instructions);
    println!("Part 2: {part_2}");
}

/// From starting location (0, 0), this fn will generate a set of all unique coordinates visited.
/// A set is used here so no coordinate is counted more than once. The length of the HashSet is the
/// puzzle answer.
fn part_1(instructions: &Vec<Point>) -> usize {
    let mut location = Point { x: 0, y: 0 };
    let mut visited: HashSet<Point> = HashSet::from([location]);
    for instruction in instructions {
        location += *instruction;
        visited.insert(location);
    }
    visited.len()
}

/// From starting location (0, 0) for robot Santa and "real" Santa, This time, each entity will
/// take turns moving about the grid of houses. As before, the length of the `visited` HashSet
/// is the puzzle answer.
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

fn map_instructions() -> Vec<Point> {
    let p = PuzzleInput::new("../data/3.txt");
    let lines = p.lines();
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

/// Abstraction used to keep track of which houses Santa has visited
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/// Implement the AddAssign trait so we can easily increment our `Point` with another `Point`
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
