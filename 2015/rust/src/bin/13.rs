//! # Advent of Code - 2015 Day 12: Perfectly Spherical Houses in a Vacuum
//!
//! ## Problem Description: Part 1
//!
//! In years past, the holiday feast with your family hasn't gone so well. Not everyone gets along!
//! This year, you resolve, will be different. You're going to find the optimal seating arrangement
//! and avoid all those awkward conversations.
//!
//! You start by writing up a list of everyone invited and the amount their happiness would increase
//! or decrease if they were to find themselves sitting next to each other person. You have a
//! circular table that will be just big enough to fit everyone comfortably, and so each person will
//! have exactly two neighbors.
//!
//! For example, suppose you have only four attendees planned, and you calculate their potential
//! happiness as follows:
//!
//! ## Problem Description: Part 2

use aoc::PuzzleInput;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let p = PuzzleInput::new("../../../data/13.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

fn part_1(lines: &Vec<&str>) -> i32 {
    let mut happiness_permutations: Vec<i32> = Vec::new();
    let mut relationships: HashMap<(&str, &str), i32> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let person = split[0];
        let neighbor = split.last().unwrap().split(".").nth(0).unwrap();
        let sign: i32 = match split[2] {
            "gain" => 1,
            "lose" => -1,
            _ => panic!["Expected 'gain' or 'lose' in place of word 3 of the puzzle input"],
        };
        let happiness: i32 = sign
            * split[3]
                .parse::<i32>()
                .expect("split[3] is not a valid i32!");
        relationships.entry((person, neighbor)).or_insert(happiness);
    }
    let people: Vec<&str> = relationships
        .keys()
        .map(|x| x.0)
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect();
    let permutations = people.clone().into_iter().permutations(people.len());
    for people in permutations {
        let mut pairs: Vec<(&str, &str)> = Vec::new();
        pairs.push((people[0], people.last().unwrap()));
        for (i, p) in people.iter().enumerate().take(people.len() - 1) {
            pairs.push((p, people[i + 1]));
        }
        let happiness = pairs.iter().fold(0, |acc, (p, n)| {
            acc + relationships.get(&(p, n)).unwrap() + relationships.get(&(n, p)).unwrap()
        });
        happiness_permutations.push(happiness);
    }
    *happiness_permutations.iter().max().unwrap()
}

fn part_2(lines: &Vec<&str>) -> i32 {
    let mut happiness_permutations: Vec<i32> = Vec::new();
    let mut relationships: HashMap<(&str, &str), i32> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let person = split[0];
        let neighbor = split.last().unwrap().split(".").nth(0).unwrap();
        let sign: i32 = match split[2] {
            "gain" => 1,
            "lose" => -1,
            _ => panic!["Expected 'gain' or 'lose' in place of word 3 of the puzzle input"],
        };
        let happiness: i32 = sign
            * split[3]
                .parse::<i32>()
                .expect("split[3] is not a valid i32!");
        relationships.entry((person, neighbor)).or_insert(happiness);
    }
    let mut people: Vec<&str> = relationships
        .keys()
        .map(|x| x.0)
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect();
    for person in &people {
        relationships.insert(("Jeff", person), 0);
        relationships.insert((person, "Jeff"), 0);
    }
    people.push("Jeff");
    let permutations = people.clone().into_iter().permutations(people.len());
    for people in permutations {
        let mut pairs: Vec<(&str, &str)> = Vec::new();
        pairs.push((people[0], people.last().unwrap()));
        for (i, p) in people.iter().enumerate().take(people.len() - 1) {
            pairs.push((p, people[i + 1]));
        }
        let happiness = pairs.iter().fold(0, |acc, (p, n)| {
            acc + relationships.get(&(p, n)).unwrap() + relationships.get(&(n, p)).unwrap()
        });
        happiness_permutations.push(happiness);
    }
    *happiness_permutations.iter().max().unwrap()
}
