use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input.txt");
    let part_1 = part_1(&lines);
    println!("{part_1}");

    let part_2 = part_2(&lines);
    println!("{part_2}");
}

fn part_1(lines: &Vec<String>) -> u32 {
    let mut reindeer: Vec<Reindeer> = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let name = split[0];
        let speed = split[3].parse::<u32>().expect("Not a valid u32");
        let duration = split[6].parse::<u32>().expect("Not a valid u32");
        let recovery = split[13].parse::<u32>().expect("Not a valid u32");

        let r = Reindeer::new(name, speed, duration, recovery);
        reindeer.push(r);
    }
    for _ in 1..=2503 {
        for r in reindeer.iter_mut() {
            r.step();
        }
    }
    reindeer.iter().map(|r| r.distance_traveled).max().unwrap()
}

fn part_2(lines: &Vec<String>) -> u32 {
    let mut reindeer: Vec<Reindeer> = Vec::new();
    let mut points: HashMap<Reindeer, u32> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let name = split[0];
        let speed = split[3].parse::<u32>().expect("Not a valid u32");
        let duration = split[6].parse::<u32>().expect("Not a valid u32");
        let recovery = split[13].parse::<u32>().expect("Not a valid u32");

        let r = Reindeer::new(name, speed, duration, recovery);
        reindeer.push(r);
        points.insert(r, 0);
    }
    for _ in 1..=2503 {
        for r in reindeer.iter_mut() {
            r.step();
        }
        let max = reindeer.iter().map(|r| r.distance_traveled).max().unwrap();
        for r in reindeer.iter_mut() {
            if r.distance_traveled == max {
                r.points += 1;
            }
        }
    }
    reindeer.iter().map(|r| r.points).max().unwrap()
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Reindeer<'a> {
    name: &'a str,
    speed: u32,
    duration: u32,
    recovery: u32,
    state: MoveState,
    distance_traveled: u32,
    leg_distance: u32,
    rest_timer: u32,
    points: u32,
}

impl Reindeer<'_> {
    fn new(name: &str, speed: u32, duration: u32, recovery: u32) -> Reindeer<'_> {
        Reindeer {
            name,
            speed,
            duration,
            recovery,
            state: MoveState::Flying,
            distance_traveled: 0,
            leg_distance: 0,
            rest_timer: 0,
            points: 0,
        }
    }

    fn change_state(&mut self) {
        let state = match self.state {
            MoveState::Flying => MoveState::Resting,
            MoveState::Resting => MoveState::Flying,
        };
        self.state = state
    }

    fn step(&mut self) {
        match self.state {
            MoveState::Flying => {
                self.leg_distance += 1;
                self.distance_traveled += self.speed;
                if self.leg_distance == self.duration {
                    self.change_state();
                    self.leg_distance = 0;
                }
            }
            MoveState::Resting => {
                self.rest_timer += 1;
                if self.rest_timer == self.recovery {
                    self.change_state();
                    self.rest_timer = 0;
                }
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum MoveState {
    Flying,
    Resting,
}
