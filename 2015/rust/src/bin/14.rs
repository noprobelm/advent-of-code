use aoc::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let p = PuzzleInput::new("../../../data/14.txt");
    let lines = p.lines();
    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

fn part_1(lines: &Vec<&str>) -> u32 {
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

fn part_2(lines: &Vec<&str>) -> u32 {
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
