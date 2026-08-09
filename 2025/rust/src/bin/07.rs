use std::{
    collections::HashSet,
    ops::{Add, Sub},
    str::FromStr,
};

use aoc::PuzzleInput;

const SAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct IVec2 {
    x: i32,
    y: i32,
}

impl IVec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Sub for IVec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for IVec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Beam {
    position: IVec2,
}

impl Beam {
    fn new(position: IVec2) -> Self {
        Beam { position }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GridObject {
    Empty,
    Splitter,
    Start,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ParsePointError;

#[derive(Clone, Debug)]
struct Grid {
    data: Vec<GridObject>,
    start: IVec2,
    width: u32,
    height: u32,
}

impl FromStr for Grid {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split("\n").collect();
        let height = split.iter().len();
        let width = split[0].chars().count();
        let start_idx = split[0]
            .chars()
            .position(|c| c == 'S')
            .ok_or(ParsePointError)?;

        assert!(split.iter().all(|s| s.chars().count() == width));

        let data: Vec<GridObject> = s.chars().filter_map(|val| match val {
            'S' => Some(GridObject::Start),
            '.' => Some(GridObject::Empty),
            '^' => Some(GridObject::Splitter),
            '\n' => None,
            s => panic!(
                "Invalid character found while building Grid. Expected '.', 'S', or '^', found {s}"
            ),
        }).collect();

        let start = IVec2::new((start_idx % width) as i32, (start_idx / width) as i32);

        Ok(Grid {
            data,
            start,
            width: width as u32,
            height: height as u32,
        })
    }
}

impl Grid {
    fn index(&self, position: IVec2) -> usize {
        assert!(position.x >= 0 && position.x < self.width as i32);
        assert!(position.y >= 0 && position.y < self.height as i32);

        position.y as usize * self.width as usize + position.x as usize
    }

    fn get(&self, position: IVec2) -> GridObject {
        self.data[self.index(position)]
    }
}

fn part_1() -> u32 {
    let p = PuzzleInput::new("../data/7.txt");
    let grid = Grid::from_str(p.string()).expect("Invalid str input for grid");
    let mut y = grid.start.y as u32 + 1;
    let mut splits = 0;
    let mut current_tier = HashSet::from([Beam::new(IVec2::new(grid.start.x, grid.start.y + 1))]);

    while y < grid.height - 1 {
        let mut next_tier = HashSet::new();
        for beam in &current_tier {
            match grid.get(IVec2::new(beam.position.x, beam.position.y + 1)) {
                GridObject::Empty => {
                    next_tier.insert(Beam::new(IVec2::new(beam.position.x, beam.position.y + 1)));
                }
                GridObject::Splitter => {
                    splits += 1;
                    next_tier.insert(Beam::new(IVec2::new(
                        beam.position.x + 1,
                        beam.position.y + 1,
                    )));
                    next_tier.insert(Beam::new(IVec2::new(
                        beam.position.x - 1,
                        beam.position.y + 1,
                    )));
                }
                GridObject::Start => {
                    unreachable!()
                }
            }
        }
        current_tier = next_tier.clone();
        next_tier.clear();

        y += 1;
    }

    splits
}

fn main() {
    // let p = PuzzleInput::new("../data/1.txt");
    // let lines = p.lines();

    let part_1 = part_1();
    println!("Part 1: {part_1}");
}
