//! # Advent of Code - 2015 Day 18: No Such Thing as Too Much
//!
//! ## Problem Description: Part 1
//! After the million lights incident, the fire code has gotten stricter: now, at most ten thousand lights are allowed. You arrange them in a 100x100 grid.
//!
//! Never one to let you down, Santa again mails you instructions on the ideal lighting configuration. With so few lights, he says, you'll have to resort to animation.
//!
//! Start by setting your lights to the included initial configuration (your puzzle input). A # means "on", and a . means "off".
//!
//! Then, animate your grid in steps, where each step decides the next configuration based on the current one. Each light's next state (either on or off) depends on its current state and the current states of the eight lights adjacent to it (including diagonals). Lights on the edge of the grid might have fewer than eight neighbors; the missing ones always count as "off".
//!
//! For example, in a simplified 6x6 grid, the light marked A has the neighbors numbered 1 through 8, and the light marked B, which is on an edge, only has the neighbors marked 1 through 5:
//!
//! 1B5...
//! 234...
//! ......
//! ..123.
//! ..8A4.
//! ..765.
//!
//! The state a light should have next is based on its current state (on or off) plus the number of neighbors that are on:
//!
//! - A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
//! - A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
//! 
//! All of the lights update simultaneously; they all consider the same current state before moving to the next.
//!
//! Here's a few steps from an example configuration of another 6x6 grid:
//!
//! Initial state:
//! .#.#.#
//! ...##.
//! #....#
//! ..#...
//! #.#..#
//! ####..
//!
//! After 1 step:
//! ..##..
//! ..##.#
//! ...##.
//! ......
//! #.....
//! #.##..
//!
//! After 2 steps:
//! ..###.
//! ......
//! ..###.
//! ......
//! .#....
//! .#....
//!
//! After 3 steps:
//! ...#..
//! ......
//! ...#..
//! ..##..
//! ......
//! ......
//!
//! After 4 steps:
//! ......
//! ......
//! ..##..
//! ..##..
//! ......
//! ......
//!
//! After 4 steps, this example has four lights on.
//!
//! In your grid of 100x100 lights, given your initial configuration, how many lights are on after 100 steps?
//!
//! ## Problem Description: Part 2
//! You flip the instructions over; Santa goes on to point out that this is all just an implementation of Conway's Game of Life. At least, it was, until you notice that something's wrong with the grid of lights you bought: four lights, one in each corner, are stuck on and can't be turned off. The example above will actually run like this:
//!
//! Initial state:
//! ##.#.#
//! ...##.
//! #....#
//! ..#...
//! #.#..#
//! ####.#
//!
//! After 1 step:
//! #.##.#
//! ####.#
//! ...##.
//! ......
//! #...#.
//! #.####
//!
//! After 2 steps:
//! #..#.#
//! #....#
//! .#.##.
//! ...##.
//! .#..##
//! ##.###
//!
//! After 3 steps:
//! #...##
//! ####.#
//! ..##.#
//! ......
//! ##....
//! ####.#
//!
//! After 4 steps:
//! #.####
//! #....#
//! ...#..
//! .##...
//! #.....
//! #.#..#
//!
//! After 5 steps:
//! ##.###
//! .##..#
//! .##...
//! .##...
//! #.#...
//! ##...#
//!
//! After 5 steps, this example now has 17 lights on.
//!
//! In your grid of 100x100 lights, given your initial configuration, but with the four corners always in the on state, how many lights are on after 100 steps?
//!

use std::ops::Add;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use aoc::PuzzleInput;

fn main() {
    let p = PuzzleInput::new("../data/18.txt");
    let lines = PuzzleInput::lines(&p);

    let part_1_num_on = part_1(&lines);
    println!("{part_1_num_on}");

    let part_2_num_on = part_2(&lines);
    println!("{part_2_num_on}");
}

fn part_1(lines: &Vec<&str>) -> String {
    let mut lights: Vec<Vec<Light>> = Vec::new();
    for (y, row) in lines.iter().enumerate() {
        let mut light_row: Vec<Light> = Vec::new();
        for (x, c) in row.chars().enumerate() {
            let state = match c {
                '#' => State::On,
                _ => State::Off,
            };
            light_row.push(Light::new(Coordinate::new(x as i32, y as i32), state));
        }
        lights.push(light_row);
    }
    let mut automaton = Automaton::new(lights);
    automaton.evolve(100);
    automaton.summarize()
}

fn part_2(lines: &Vec<&str>) -> String {
    let mut lights: Vec<Vec<Light>> = Vec::new();
    for (y, row) in lines.iter().enumerate() {
        let mut light_row: Vec<Light> = Vec::new();
        for (x, c) in row.chars().enumerate() {
            let state = match c {
                '#' => State::On,
                _ => State::Off,
            };
            if (x, y) == (0, 0) || (x, y) == (0, 99) || (x, y) == (99, 0) || (x, y) == (99, 99) {
                light_row.push(Light::new(Coordinate::new(x as i32, y as i32), State::On));
            }
            else {
                light_row.push(Light::new(Coordinate::new(x as i32, y as i32), state));
            }
        }
        lights.push(light_row);
    }
    let mut automaton = Automaton::new(lights);
    automaton.evolve_2(100);
    automaton.summarize()
}


#[derive(Debug, Clone)]
struct Automaton {
    generation: usize,
    lights: Vec<Vec<Light>>,
    xmax: i32,
    ymax: i32
}

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Light {
    coord: Coordinate,
    state: State,
}

impl Light {
    fn new(coord: Coordinate, state: State) -> Self {
        Self { coord, state }
    }
}

#[derive(Debug, EnumIter)]
enum Moore {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Automaton {
    fn new(lights: Vec<Vec<Light>>) -> Self {
        let generation: usize = 0;
        let xmax = (lights.len() - 1) as i32;
        let ymax = (lights.len() - 1) as i32;
        Self { generation, lights, xmax, ymax }
    }

    fn evolve(&mut self, generations: usize) {
        for _ in 0..generations {
            let mut new_lights: Vec<Vec<Light>> = Vec::new();
            for (y, row) in self.lights.iter().enumerate() {
                let mut new_row: Vec<Light> = Vec::new();
                for (x, light) in row.iter().enumerate() {
                    let neighbors = self.get_neighbors(light.coord);
                    let num_on = neighbors.iter().fold(0, |acc, light| if light.state == State::On {acc + 1} else {acc});
                    match light.state {
                        State::On => {
                            if num_on == 2 || num_on == 3 {
                                new_row.push(light.clone())
                            }
                            else {
                                new_row.push(Light::new(light.coord, State::Off))
                            }
                        },
                        State::Off => {
                            if num_on == 3 {
                                new_row.push(Light::new(light.coord, State::On))
                            }
                            else {
                                new_row.push(light.clone())
                            }
                        }
                    }
                }
                new_lights.push(new_row);
            }
            self.lights = new_lights;
            self.generation += 1;
        }

    }

    fn evolve_2(&mut self, generations: usize) {
        for _ in 0..generations {
            let mut new_lights: Vec<Vec<Light>> = Vec::new();
            for (y, row) in self.lights.iter().enumerate() {
                let mut new_row: Vec<Light> = Vec::new();
                for (x, light) in row.iter().enumerate() {
                    if (x, y) == (0, 0) || (x, y) == (0, 99) || (x, y) == (99, 0) || (x, y) == (99, 99) {
                        new_row.push(light.clone());
                        continue;
                    }
                    let neighbors = self.get_neighbors(light.coord);
                    let num_on = neighbors.iter().fold(0, |acc, light| if light.state == State::On {acc + 1} else {acc});
                    match light.state {
                        State::On => {
                            if num_on == 2 || num_on == 3 {
                                new_row.push(light.clone())
                            }
                            else {
                                new_row.push(Light::new(light.coord, State::Off))
                            }
                        },
                        State::Off => {
                            if num_on == 3 {
                                new_row.push(Light::new(light.coord, State::On))
                            }
                            else {
                                new_row.push(light.clone())
                            }
                        }
                    }
                }
                new_lights.push(new_row)
            }
            self.lights = new_lights;
            self.generation += 1;
        }

    }

    fn get_neighbors(&self, coord: Coordinate) -> Vec<&Light> {
        let mut neighbors: Vec<&Light> = Vec::new();
        for neighbor in Moore::iter() {
            let neighbor_coord = match neighbor {
                Moore::North => coord + Coordinate::new(0, 1),
                Moore::NorthEast => coord + Coordinate::new(1, 1),
                Moore::East => coord + Coordinate::new(1, 0),
                Moore::SouthEast => coord + Coordinate::new(1, -1),
                Moore::South => coord + Coordinate::new(0, -1),
                Moore::SouthWest => coord + Coordinate::new(-1, -1),
                Moore::West => coord + Coordinate::new(-1, 0),
                Moore::NorthWest => coord + Coordinate::new(-1, 1),
            };
            if neighbor_coord.x >= 0 && neighbor_coord.y >= 0 && neighbor_coord.x <= self.xmax && neighbor_coord.y <= self.ymax {
                neighbors.push(&self.lights[neighbor_coord.y as usize][neighbor_coord.x as usize]);
            }
        }
        neighbors
    }

    fn summarize(&self) -> String {
        let mut num_on = 0;
        let mut num_off = 0;
        for row in &self.lights {
            for light in row {
                match light.state {
                    State::On => num_on += 1,
                    State::Off => {num_off += 1}
                }
            }
        }
        format!("On: {}, Off: {}", num_on, num_off)
        }

}

