use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Index, IndexMut};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input.txt");

    let part_1 = part_1(&lines);
    println!("After following Santa's instructions for changing the on/off state of our xmas lights, {part_1} lights remain on");

    let part_2 = part_2(&lines);
    println!("After following Santa's instructions for adjusting the brightness of our xmas lights, '{part_2}' is the total measured brightness");
}

fn part_1(lines: &Vec<String>) -> u32 {
    let mut count: u32 = 0;
    let mut grid = Grid::new(1000, 1000);

    for line in lines {
        let (pos_1, pos_2, instruction) = parse_instructions(&line);

        match instruction {
            "toggle" => {
                for row in pos_1[0]..=pos_2[0] {
                    for col in pos_1[1]..=pos_2[1] {
                        &grid[(row, col)].toggle();
                    }
                }
            }
            "on" => {
                for row in pos_1[0]..=pos_2[0] {
                    for col in pos_1[1]..=pos_2[1] {
                        &grid[(row, col)].on();
                    }
                }
            }
            "off" => {
                for row in pos_1[0]..=pos_2[0] {
                    for col in pos_1[1]..=pos_2[1] {
                        &grid[(row, col)].off();
                    }
                }
            }
            _ => panic!["panik"],
        }
    }

    for row in 0..1000 {
        for col in 0..1000 {
            match &grid[(row, col)].state {
                State::On => count += 1,
                State::Off => {}
            }
        }
    }

    count
}

fn part_2(lines: &Vec<String>) -> u32 {
    let mut brightness: u32 = 0;
    let mut grid = Grid::new(1000, 1000);

    for line in lines {
        let (pos_1, pos_2, instruction) = parse_instructions(&line);

        match instruction {
            "toggle" => {
                for row in pos_1[0]..=pos_2[0] {
                    for col in pos_1[1]..=pos_2[1] {
                        &grid[(row, col)].increase_brightness(2);
                    }
                }
            }
            "on" => {
                for row in pos_1[0]..=pos_2[0] {
                    for col in pos_1[1]..=pos_2[1] {
                        &grid[(row, col)].increase_brightness(1);
                    }
                }
            }
            "off" => {
                for row in pos_1[0]..=pos_2[0] {
                    for col in pos_1[1]..=pos_2[1] {
                        &grid[(row, col)].decrease_brightness(1);
                    }
                }
            }
            _ => panic!["panik"],
        }
    }

    for row in 0..1000 {
        for col in 0..1000 {
            brightness += &grid[(row, col)].brightness;
        }
    }

    brightness
}

fn parse_instructions(unparsed: &String) -> (Vec<usize>, Vec<usize>, &str) {
    let split: Vec<&str> = unparsed.split(" ").collect();
    match split[0] {
        "toggle" => {
            let pos_1: Vec<usize> = split[1]
                .split(",")
                .map(|n| n.parse::<usize>().expect("Invalid num"))
                .collect();

            let pos_2: Vec<usize> = split[3]
                .split(",")
                .map(|n| n.parse::<usize>().expect("Invalid num"))
                .collect();

            let instruction: &str = "toggle";

            (pos_1, pos_2, instruction)
        }
        "turn" => {
            let pos_1: Vec<usize> = split[2]
                .split(",")
                .map(|n| n.parse::<usize>().expect("Invalid num"))
                .collect();

            let pos_2: Vec<usize> = split[4]
                .split(",")
                .map(|n| n.parse::<usize>().expect("Invalid num"))
                .collect();

            let instruction: &str = split[1];

            (pos_1, pos_2, instruction)
        }
        _ => panic!["panik"],
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Copy, Clone, Debug)]
struct Light {
    state: State,
    brightness: u32,
}

impl Light {
    fn new() -> Self {
        Light {
            state: State::Off,
            brightness: 0,
        }
    }

    fn toggle(&mut self) {
        self.state = match self.state {
            State::On => State::Off,
            State::Off => State::On,
        };
    }

    fn on(&mut self) {
        self.state = State::On;
    }

    fn off(&mut self) {
        self.state = State::Off;
    }

    fn increase_brightness(&mut self, n: u32) {
        self.brightness += n;
    }
    fn decrease_brightness(&mut self, n: u32) {
        if self.brightness > 0 {
            self.brightness -= n;
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum State {
    On,
    Off,
}

#[derive(Clone, Debug)]
struct Grid {
    grid: Vec<Vec<Light>>,
}

impl Grid {
    fn new(size_x: usize, size_y: usize) -> Self {
        let mut grid: Vec<Vec<Light>> = Vec::with_capacity(size_y);
        for y in 0..size_y {
            let mut row: Vec<Light> = Vec::with_capacity(size_x);
            for i in 0..size_x {
                let mut light = Light::new();
                row.push(light);
            }
            grid.push(row);
        }
        Grid { grid: grid }
    }
}

type GridIndex = (usize, usize);

impl Index<GridIndex> for Grid {
    type Output = Light;

    fn index(&self, index: GridIndex) -> &Self::Output {
        let (row, col) = index;
        &self.grid[row][col]
    }
}

impl IndexMut<GridIndex> for Grid {
    fn index_mut(&mut self, index: GridIndex) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.grid[row][col]
    }
}